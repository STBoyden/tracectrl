use chrono::{DateTime, Utc};
use sqlx::types::ipnetwork::IpNetwork;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::api::types::{Snippet, Trace};

fn _default_received_from() -> Option<IpNetwork> {
	None
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, ToSchema, sqlx::FromRow)]
pub struct Log {
	pub id: Uuid,
	#[schema(example = "hello")]
	pub message: String,
	#[schema(example = "Rust")]
	pub language: String,
	pub snippet: Snippet,
	pub backtrace: Trace,
	#[schema(example = json!(["This program was compiled without symbols."]))]
	pub warnings: Vec<String>,
	pub date: DateTime<Utc>,
	#[serde(skip_deserializing)]
	#[schema(nullable, default = _default_received_from)]
	pub received_from: Option<IpNetwork>,
}
