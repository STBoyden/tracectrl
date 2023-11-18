use std::collections::BTreeMap;

use axum::{
	extract::{Path, State},
	http::StatusCode,
	Json,
};
use chrono::Utc;
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
	pub backtrace: BTreeMap<u32, String>,
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
		(status=200, description="Log was created")
	),
)]
#[axum_macros::debug_handler]
pub async fn add_log(
	State(store): State<Store>,
	Json(log): Json<LogBody>,
) -> Json<Response> {
	let mut logs = store.logs.lock();

	let log: Log = log.into();
	logs.push(log.clone());

	if let Err(err) = store.sender.send(log.clone()) {
		tracing::error!("Could not send log to back-end: {err}");
	}

	Json(Response {
		message: fmt!("Log was created with ID {}", log.id),
		datetime: log.date,
	})
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
