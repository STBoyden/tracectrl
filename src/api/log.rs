use std::collections::BTreeMap;

use axum::{
	extract::{Path, State},
	http::StatusCode,
	Extension,
	Json,
};
use chrono::Utc;
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
	api::{
		types::{Log, Snippet},
		ApiResult,
		Response,
		Store,
	},
	prelude::*,
};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct LogBody {
	#[schema(example = "hello")]
	pub message: String,
	#[schema(example = "Rust")]
	pub language: String,
	#[schema(example=json!({"1": r#"log("hello")"#}))]
	pub backtrace: BTreeMap<i32, String>,
	pub snippet: Snippet,
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
pub async fn list_logs(State(store): State<Store>) -> Json<Vec<Log>> {
	let logs = store.logs.lock().clone();

	Json(logs)
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
	Json(log): Json<LogBody>,
) -> Result<Json<Response>> {
	let log: Log = log.into();

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

	let layer_results = log.backtrace.layers.iter().map(|layer| {
		sqlx::query!(
			r#"INSERT INTO snippets (line, code) VALUES ($1, $2) RETURNING id"#,
			layer.line,
			layer.code.clone()
		)
		.fetch_one(&pool)
	});

	let backtrace_id =
		sqlx::query!(r#"INSERT INTO backtraces DEFAULT VALUES RETURNING id"#)
			.fetch_one(&pool)
			.await?
			.id;

	let layers = log.backtrace.layers.len();
	for future in layer_results {
		let layer = future.await?;

		sqlx::query!(
			r#"INSERT INTO backtrace_snippet (backtrace_id, snippet_id, amount) VALUES ($1, $2, $3)"#,
			backtrace_id,
			layer.id,
			layers as i32
		)
		.execute(&pool)
		.await?;
	}

	let log_id= sqlx::query!(
		r#"INSERT INTO logs (id, message, language, snippet_id, backtrace_id, warnings, date) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id"#,
		log.id,
		log.message.clone(),
		log.language.clone(),
		snippet_id,
		backtrace_id,
		&log.warnings.clone(),
		log.date.naive_utc(),
	).fetch_one(&pool)
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
		(status=404, description="The log with the given `id` was not found"),
		(status=200, body=Log, description="The log with the given `id`")
	),
	params(
		("id" = Uuid, Path, description = "Log ID")
	),
)]
#[axum_macros::debug_handler]
pub async fn get_log(
	State(store): State<Store>,
	Path(id): Path<Uuid>,
) -> ApiResult<Json<Log>> {
	let logs = store.logs.lock().clone();

	let log = logs
		.iter()
		.find(|log| log.id == id)
		.ok_or(StatusCode::NOT_FOUND)?
		.clone();

	Ok(Json(log))
}
