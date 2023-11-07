#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Snippet {
	line: u32,
	code: String,
}
