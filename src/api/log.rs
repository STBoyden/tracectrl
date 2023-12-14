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
use sqlx::{types::ipnetwork::IpNetwork, PgPool};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
	api::{
		types::{Layer, Log, Snippet, Trace},
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
	#[schema(example = "Rust")]
	pub language: String,
	#[schema(example=json!({"1": r#"log("hello");"#}))]
	pub backtrace: BTreeMap<i32, String>,
	pub snippet: Snippet,
	#[schema(example = json!(["This file was compiled without debug symbols."]))]
	pub warnings: Vec<String>,
}

impl From<LogBody> for Log {
	fn from(value: LogBody) -> Log {
		let value = value.clone();

		Log {
			id: Uuid::new_v4(),
			message: value.message,
			language: value.language,
			backtrace: value.backtrace.into(),
			snippet: value.snippet,
			warnings: value.warnings,
			date: Utc::now(),
			received_from: None,
		}
	}
}

#[utoipa::path(
	get,
	path="/api/logs",
	responses(
		(status=200, description="List all the logs received by the server", body=[Log])
	),
)]
#[axum_macros::debug_handler]
pub async fn list_logs(Extension(pool): Extension<PgPool>) -> Result<Json<Vec<Log>>> {
	let mut log_records = sqlx::query!(r#"SELECT * FROM logs"#)
		.fetch_all(&pool)
		.await?;

	let mut logs = vec![];

	for log in &mut log_records {
		let snippet_record = sqlx::query!(
			r#"SELECT line, code, file FROM snippets WHERE id = $1"#,
			log.snippet_id
		)
		.fetch_one(&pool)
		.await?;

		let mut traces = vec![];
		let trace_records = sqlx::query!(
			r###"
			SELECT line, code, file
			FROM backtrace_snippet 
			JOIN backtraces ON backtrace_id=backtraces.id 
			JOIN snippets 	ON snippet_id=snippets.id 
			WHERE backtrace_id = $1
			"###,
			log.backtrace_id
		)
		.fetch_all(&pool)
		.await?;

		for trace in trace_records {
			traces.push(Layer(Snippet {
				line: trace.line,
				code: trace.code,
				file: trace.file,
			}));
		}

		let log = Log {
			id: log.id,
			message: log.message.clone(),
			language: log.language.clone(),
			snippet: Snippet {
				line: snippet_record.line,
				code: snippet_record.code,
				file: snippet_record.file,
			},
			backtrace: Trace { layers: traces },
			warnings: log.warnings.clone(),
			date: log.date.and_utc(),
			received_from: log.received_from,
		};

		logs.push(log);
	}

	Ok(Json(logs))
}

#[utoipa::path(
	post,
	path="/api/log",
	request_body=LogBody,
	responses(
		(status=200, description="Log was created"),
		(status=500, description="An internal server error occurred")
	),
)]
#[axum_macros::debug_handler]
pub async fn add_log(
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
	}

	let snippet_id = sqlx::query!(
		r#"INSERT INTO snippets (line, code) VALUES ($1, $2) RETURNING id"#,
		log.snippet.line,
		log.snippet.code.clone()
	)
	.fetch_one(&pool)
	.await?
	.id;

	let backtrace_id =
		sqlx::query!(r#"INSERT INTO backtraces DEFAULT VALUES RETURNING id"#)
			.fetch_one(&pool)
			.await?
			.id;

	let rows_changed = sqlx::query!(
		r###"
		INSERT INTO backtrace_snippet (backtrace_id, snippet_id)
		VALUES ($1, $2)
		"###,
		backtrace_id,
		snippet_id
	)
	.execute(&pool)
	.await?
	.rows_affected();

	if rows_changed == 0 {
		tracing::error!(
			"Could not insert snippet {snippet_id} for backtrace {backtrace_id}."
		);
	}

	let layer_results = log
		.backtrace
		.layers
		.iter()
		.filter(|layer| log.snippet.line != layer.line && log.snippet.code != layer.code)
		.map(|layer| {
			sqlx::query!(
				r#"INSERT INTO snippets (line, code) VALUES ($1, $2) RETURNING id"#,
				layer.line,
				layer.code.clone()
			)
			.fetch_one(&pool)
		});

	for future in layer_results {
		let layer = future.await?;

		sqlx::query!(
			r###"
			INSERT INTO backtrace_snippet (backtrace_id, snippet_id) 
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
		INSERT INTO logs (id, message, language, snippet_id, backtrace_id, warnings, date, received_from) 
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8) 
		RETURNING id
		"###,
		log.id,
		log.message.clone(),
		log.language.clone(),
		snippet_id,
		backtrace_id,
		&log.warnings.clone(),
		log.date.naive_utc(),
		ip_network,
	)
	.fetch_one(&pool)
	.await?
	.id;

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
		("id" = Uuid, Path, description = "Log ID")
	),
)]
#[axum_macros::debug_handler]
pub async fn get_log(
	Extension(pool): Extension<PgPool>,
	Path(id): Path<Uuid>,
) -> Result<Json<Log>> {
	let log_record = sqlx::query!(r#"SELECT * FROM logs WHERE id = $1"#, id)
		.fetch_one(&pool)
		.await?;

	let snippet_record = sqlx::query!(
		r#"SELECT line, code, file FROM snippets WHERE id = $1"#,
		log_record.snippet_id
	)
	.fetch_one(&pool)
	.await?;

	let mut layers = vec![];
	let trace_records = sqlx::query!(
		r###"
			SELECT line, code, file
			FROM backtrace_snippet 
			JOIN backtraces ON backtrace_id=backtraces.id 
			JOIN snippets 	ON snippet_id=snippets.id 
			WHERE backtrace_id = $1
			"###,
		log_record.backtrace_id
	)
	.fetch_all(&pool)
	.await?;

	for trace in trace_records {
		layers.push(Layer(Snippet {
			line: trace.line,
			code: trace.code,
			file: trace.file,
		}));
	}

	Ok(Json(Log {
		id: log_record.id,
		message: log_record.message.clone(),
		language: log_record.language.clone(),
		snippet: Snippet {
			line: snippet_record.line,
			code: snippet_record.code,
			file: snippet_record.file,
		},
		backtrace: Trace { layers },
		warnings: log_record.warnings.clone(),
		date: log_record.date.and_utc(),
		received_from: log_record.received_from,
	}))
}
