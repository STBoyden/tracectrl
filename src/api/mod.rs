mod client;
mod extractors;
mod log;
pub mod types;

use std::{net::SocketAddr, sync::Arc};

use axum::{
	body::Body,
	extract::ConnectInfo,
	http::Request,
	response::IntoResponse,
	routing::{get, post},
	Extension,
	Router,
};
use chrono::{DateTime, Utc};
use parking_lot::Mutex;
use reqwest::StatusCode;
use sqlx::PgPool;
use tower_http::{
	trace::{
		DefaultMakeSpan,
		DefaultOnFailure,
		DefaultOnRequest,
		DefaultOnResponse,
		TraceLayer,
	},
	LatencyUnit,
};
use tracing::Level;
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
	paths(
		log::list_logs,
		log::add_log,
		log::get_log,
		client::new_client,
		client::register_client
	),
	components(schemas(
		Uuid,
		Response,
		types::Log,
		types::Trace,
		types::Layer,
		log::LogBody,
		client::RegisterClientResponse,
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

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct Response {
	pub message: String,
	pub datetime: DateTime<Utc>,
}

pub struct ApiRouter;

#[axum_macros::debug_handler]
async fn fallback(
	ConnectInfo(addr): ConnectInfo<SocketAddr>,
	request: Request<Body>,
) -> impl IntoResponse {
	tracing::info!("{addr} requested path '{}': not found", request.uri());

	(StatusCode::NOT_FOUND, "Not found")
}

impl ApiRouter {
	pub fn new_router(log_sender: LogSender, pool: PgPool) -> Router {
		Router::new()
			.route("/logs", get(log::list_logs))
			.route("/log", post(log::add_log))
			.route("/log/:id", get(log::get_log))
			.route("/get_or_register_client", post(client::new_client))
			.route("/get_or_register_client/:id", post(client::register_client))
			.with_state(Store::new(log_sender))
			.fallback(fallback)
			.layer(Extension(pool))
			.layer(
				TraceLayer::new_for_http()
					.make_span_with(DefaultMakeSpan::new().include_headers(true))
					.on_request(DefaultOnRequest::new().level(Level::INFO))
					.on_failure(DefaultOnFailure::new().level(Level::ERROR))
					.on_response(
						DefaultOnResponse::new()
							.level(Level::INFO)
							.latency_unit(LatencyUnit::Micros),
					),
			)
	}
}
