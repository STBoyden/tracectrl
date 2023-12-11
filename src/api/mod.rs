mod log;
pub mod types;

use std::sync::Arc;

use axum::{
	http::StatusCode,
	routing::{get, post},
	Extension,
	Router,
};
use chrono::{DateTime, Utc};
use parking_lot::Mutex;
use sqlx::PgPool;
use utoipa::{OpenApi, ToSchema};

use crate::{
	api::types::Log,
	utils::{log_socket::LogSender, uuid::Uuid},
};

#[derive(OpenApi)]
#[openapi(
	info(
		title = "TraceCTRL",
		description = "API documentation for the TraceCTRL REST server",
	),
	paths(log::list_logs, log::add_log, log::get_log),
	components(schemas(
		Uuid,
		Response,
		types::Log,
		types::Trace,
		types::Snippet,
		types::Layer,
		log::LogBody,
	))
)]
pub struct ApiDoc;

#[derive(Debug, Clone)]
pub struct Store {
	pub logs: Arc<Mutex<Vec<Log>>>,
	pub sender: LogSender,
}

impl Store {
	pub fn new(sender: LogSender) -> Self {
		Self {
			logs: Arc::default(),
			sender,
		}
	}
}

pub type ApiResult<T> = Result<T, StatusCode>;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct Response {
	pub message: String,
	pub datetime: DateTime<Utc>,
}

pub struct ApiRouter;

impl ApiRouter {
	pub fn new_router(log_sender: LogSender, pool: PgPool) -> Router {
		Router::new()
			.route("/logs", get(log::list_logs))
			.route("/log", post(log::add_log))
			.route("/log/:id", get(log::get_log))
			.with_state(Store::new(log_sender))
			.layer(Extension(pool))
	}
}
