use chrono::{DateTime, Utc};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::api::types::{Snippet, Trace};

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
}
