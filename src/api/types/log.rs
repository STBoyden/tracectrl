use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use common_macros::b_tree_map;
use sqlx::types::ipnetwork::IpNetwork;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::api::types::Trace;

fn _default_received_from() -> Option<IpNetwork> {
	None
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, ToSchema, sqlx::FromRow)]
pub struct Log {
	pub id: Uuid,
	#[schema(example = "hello")]
	pub message: String,
	#[schema(example = "&str")]
	pub message_type: String,
	#[schema(example = "Rust")]
	pub language: String,
	#[schema(example = json!(b_tree_map!{
				1 => "fn main() {",
				2 => "    log(\"hello\");",
				3 => "}"
			}),
	)]
	pub snippet: BTreeMap<i32, String>,
	#[schema(example = "src/main.rs")]
	pub file_name: String,
	#[schema(example = 1, minimum = 1)]
	pub line_number: i32,
	pub backtrace: Trace,
	#[schema(example = json!(["This program was compiled without symbols."]))]
	pub warnings: Vec<String>,
	pub date: DateTime<Utc>,
	#[serde(skip_deserializing)]
	#[schema(nullable, default = _default_received_from)]
	pub received_from: Option<IpNetwork>,
}
