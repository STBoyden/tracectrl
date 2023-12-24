use std::{
	collections::BTreeMap,
	net::{IpAddr, SocketAddr},
};

use axum::{
	extract::{ConnectInfo, Path, State},
	Extension,
	Json,
};
use chrono::Utc;
use common_macros::b_tree_map;
use sqlx::{types::ipnetwork::IpNetwork, PgPool};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
	api::{
		extractors::client::ClientId,
		types::{Layer, Log, Trace},
		Response,
		Store,
	},
	prelude::*,
};

fn socket_addr_to_ip_network(socket_addr: &SocketAddr) -> IpNetwork {
	let ip = socket_addr.ip();
	IpNetwork::new(ip, single_host_prefix(&ip))
		.expect("single_host_prefix created invalid prefix")
}
fn single_host_prefix(ip_addr: &IpAddr) -> u8 {
	match ip_addr {
		IpAddr::V4(_) => 32,
		IpAddr::V6(_) => 128,
	}
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct LogBody {
	#[schema(example = "hello")]
	pub message: String,
	#[schema(example = "&str")]
	pub message_type: String,
	#[schema(example = "Rust")]
	pub language: String,
	pub backtrace: Trace,
	#[schema(example = json!(b_tree_map!{
				1 => "fn main() {",
				2 => "    log(\"hello\");",
				3 => "}"
			}),
	)]
	pub snippet: BTreeMap<i32, String>,
	#[schema(minimum = 1, example = 2)]
	pub line_number: i32,
	#[schema(example = "src/main.rs")]
	pub file_name: String,
	#[schema(example = json!(["This file was compiled without debug symbols."]))]
	pub warnings: Vec<String>,
}

impl From<LogBody> for Log {
	fn from(value: LogBody) -> Log {
		let value = value.clone();

		Log {
			id: Uuid::new_v4(),
			message: value.message,
			message_type: value.message_type,
			language: value.language,
			backtrace: value.backtrace,
			snippet: value.snippet,
			line_number: value.line_number,
			warnings: value.warnings,
			file_name: value.file_name,
			date: Utc::now(),
			received_from: None,
		}
	}
}

async fn list_logs_for_user(
	pool: PgPool,
	ClientId(client_id): ClientId,
) -> Result<Json<Vec<Log>>> {
	let mut logs = vec![];

	let log_records = sqlx::query!(
		r###"
	SELECT * FROM "Logs"
	WHERE client_id = $1
	"###,
		client_id
	)
	.fetch_all(&pool)
	.await?;

	for log_record in log_records {
		let mut traces = vec![];
		let trace_records = sqlx::query!(
			r###"
			SELECT "Layers".*
			FROM "BacktracesLayers" 
			JOIN "Backtraces" ON backtrace_id="Backtraces".id 
			JOIN "Layers" 		ON layer_id="Layers".id 
			WHERE backtrace_id = $1
			"###,
			log_record.backtrace_id
		)
		.fetch_all(&pool)
		.await?;

		for trace in trace_records {
			traces.push(Layer {
				line_number: trace.line_number,
				column_number: trace.column_number,
				code: trace.code,
				name: trace.name,
				file_path: Some(trace.file_path),
			});
		}

		let log = Log {
			id: log_record.id,
			message: log_record.message.clone(),
			language: log_record.language.clone(),
			snippet: serde_json::from_value(log_record.snippet.clone())?,
			backtrace: Trace { layers: traces },
			warnings: log_record.warnings.clone(),
			date: log_record.date.and_utc(),
			received_from: log_record.received_from,
			message_type: log_record.message_type,
			line_number: log_record.line_number,
			file_name: log_record.file_name,
		};

		logs.push(log);
	}

	Ok(Json(logs))
}

async fn list_server_logs(pool: PgPool) -> Result<Json<Vec<Log>>> {
	let mut log_records = sqlx::query!(r#"SELECT * FROM "Logs""#)
		.fetch_all(&pool)
		.await?;

	let mut logs = vec![];

	for log in &mut log_records {
		let mut traces = vec![];
		let trace_records = sqlx::query!(
			r###"
			SELECT "Layers".*
			FROM "BacktracesLayers" 
			JOIN "Backtraces" ON backtrace_id="Backtraces".id 
			JOIN "Layers" 		ON layer_id="Layers".id 
			WHERE backtrace_id = $1
			"###,
			log.backtrace_id
		)
		.fetch_all(&pool)
		.await?;

		for trace in trace_records {
			traces.push(Layer {
				line_number: trace.line_number,
				column_number: trace.column_number,
				code: trace.code,
				name: trace.name,
				file_path: Some(trace.file_path),
			});
		}

		let log = Log {
			id: log.id,
			message: log.message.clone(),
			language: log.language.clone(),
			snippet: serde_json::from_value(log.snippet.clone())?,
			backtrace: Trace { layers: traces },
			warnings: log.warnings.clone(),
			date: log.date.and_utc(),
			received_from: log.received_from,
			message_type: log.message_type.clone(),
			line_number: log.line_number,
			file_name: log.file_name.clone(),
		};

		logs.push(log);
	}

	Ok(Json(logs))
}

#[utoipa::path(
	get,
	path="/api/logs",
	responses(
		(status=200, description="List all the logs received by the server, or logs sent to the given `client-id` if present.", body=[Log])
	),
	params(
		("client-id" = Option<i32>, Header, description = "Client ID (optional)"),
	),
)]
#[axum_macros::debug_handler]
pub async fn list_logs(
	client_id: Option<ClientId>,
	Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<Log>>> {
	if let Some(client_id) = client_id {
		list_logs_for_user(pool, client_id).await
	} else {
		list_server_logs(pool).await
	}
}

#[utoipa::path(
	post,
	path="/api/log",
	request_body=LogBody,
	responses(
		(status=200, description="Log was created"),
		(status=500, description="An internal server error occurred")
	),
	params(
		("client-id" = i32, Header, description = "Client ID"),
	),
)]
#[axum_macros::debug_handler]
pub async fn add_log(
	ClientId(client_id): ClientId,
	Extension(pool): Extension<PgPool>,
	State(store): State<Store>,
	ConnectInfo(addr): ConnectInfo<SocketAddr>,
	Json(log): Json<LogBody>,
) -> Result<Json<Response>> {
	let mut log: Log = log.into();
	let ip_network = socket_addr_to_ip_network(&addr);

	log.received_from = Some(ip_network);

	{
		let mut logs = store.logs.lock();

		logs.push(log.clone());
	}

	if let Err(err) = store.sender.send(log.clone()) {
		tracing::error!("Could not send log to back-end: {err}");
	} else {
		tracing::info!("Sent log to backend");
	}

	let backtrace_id =
		sqlx::query!(r#"INSERT INTO "Backtraces" DEFAULT VALUES RETURNING id"#)
			.fetch_one(&pool)
			.await?
			.id;

	let layer_results = log.backtrace.layers.iter().map(|layer| {
		sqlx::query!(
			r###"
				INSERT INTO "Layers" (line_number, column_number, code, name, file_path) 
				VALUES ($1, $2, $3, $4, $5) 
				RETURNING id
				"###,
			layer.line_number,
			layer.column_number,
			layer.code.clone(),
			layer.name.clone(),
			layer.file_path.clone(),
		)
		.fetch_one(&pool)
	});

	for future in layer_results {
		let layer = future.await?;

		sqlx::query!(
			r###"
			INSERT INTO "BacktracesLayers" (backtrace_id, layer_id) 
			VALUES ($1, $2)
			"###,
			backtrace_id,
			layer.id,
		)
		.execute(&pool)
		.await?;
	}

	let log_id = sqlx::query!(
		r###"
		INSERT INTO "Logs" (
			client_id,
			message, 
			message_type,
			language, 
			snippet, 
			line_number,
			backtrace_id, 
			warnings, 
			date,
			file_name,
			received_from
		) 
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) 
		RETURNING id
		"###,
		client_id,
		log.message.clone(),
		log.message_type.clone(),
		log.language.clone(),
		serde_json::to_value(&log.snippet)?,
		log.line_number,
		backtrace_id,
		&log.warnings.clone(),
		log.date.naive_utc(),
		log.file_name.clone(),
		ip_network,
	)
	.fetch_one(&pool)
	.await?
	.id;

	sqlx::query!(
		r###"
			UPDATE "Clients"
			SET last_connected = now(), logs_sent = logs_sent + 1
			WHERE id = $1
		"###,
		client_id
	)
	.execute(&pool)
	.await?;

	Ok(Json(Response {
		message: fmt!("Log was created with ID {log_id}"),
		datetime: log.date,
	}))
}

#[utoipa::path(
	get,
	path="/api/log/{id}",
	responses(
		(status=200, body=Log, description="The log with the given `id`"),
		(status=404, description="The log with the given `id` was not found"),
	),
	params(
		("client-id" = i32, Header, description = "Client ID"),
		("id" = Uuid, Path, description = "Log ID")
	),
)]
#[axum_macros::debug_handler]
pub async fn get_log(
	ClientId(client_id): ClientId,
	Extension(pool): Extension<PgPool>,
	Path(id): Path<Uuid>,
) -> Result<Json<Log>> {
	let log_record = sqlx::query!(
		r###"
			SELECT * FROM "Logs" 
			WHERE id = $1 AND backtrace_id = $2
	"###,
		id,
		client_id
	)
	.fetch_optional(&pool)
	.await?;

	if log_record.is_none() {
		return Err(Error::ResponseError(
			StatusCode::NOT_FOUND,
			format!("Log with ID {id} not found for client ID {client_id}"),
		));
	}

	let log_record = log_record.unwrap();

	let mut layers = vec![];
	let trace_records = sqlx::query!(
		r###"
			SELECT "Layers".*
			FROM "BacktracesLayers" 
			JOIN "Backtraces" ON backtrace_id="Backtraces".id 
			JOIN "Layers" 		ON layer_id="Layers".id 
			WHERE backtrace_id = $1
			"###,
		log_record.backtrace_id
	)
	.fetch_all(&pool)
	.await?;

	for trace in trace_records {
		layers.push(Layer {
			line_number: trace.line_number,
			column_number: trace.column_number,
			code: trace.code,
			name: trace.name,
			file_path: Some(trace.file_path),
		});
	}

	Ok(Json(Log {
		id: log_record.id,
		message: log_record.message.clone(),
		message_type: log_record.message_type.clone(),
		language: log_record.language.clone(),
		snippet: serde_json::from_value(log_record.snippet.clone())?,
		line_number: log_record.line_number,
		backtrace: Trace { layers },
		warnings: log_record.warnings.clone(),
		date: log_record.date.and_utc(),
		received_from: log_record.received_from,
		file_name: log_record.file_name,
	}))
}
