use utoipa::ToSchema;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, ToSchema, sqlx::FromRow)]
pub struct Layer {
	#[schema(example = 2, minimum = 1)]
	pub line_number: i32,
	#[schema(example = 4, minimum = 1)]
	pub column_number: i32,
	#[schema(example = "log(\"hello\");")]
	pub code: String,
	#[schema(example = "main.rs")]
	pub name: String,
	#[schema(example = "src/main.rs")]
	pub file_path: Option<String>,
}

impl Default for Layer {
	fn default() -> Self {
		Self {
			line_number: 1,
			column_number: 5,
			code: String::from("log(\"hello\");"),
			name: String::from("main.rs"),
			file_path: Some(String::from("src/main.rs")),
		}
	}
}
