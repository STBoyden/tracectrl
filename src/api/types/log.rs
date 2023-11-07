use crate::api::types::{Snippet, Trace};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Log {
	language: String,
	snippet: Snippet,
	backtrace: Trace,
}
