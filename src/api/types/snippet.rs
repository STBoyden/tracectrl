use std::ops::Deref;

use utoipa::ToSchema;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, ToSchema, sqlx::FromRow)]
pub struct Snippet {
	#[schema(example = 2, minimum = 1)]
	pub line: i32,
	#[schema(example = r#"fn main() {
	log("hello");
}"#)]
	pub code: String,
	#[schema(example = "src/main.rs")]
	pub file: Option<String>,
}

impl Default for Snippet {
	fn default() -> Self {
		Self {
			line: 1,
			code: String::from(r#"log("hello");"#),
			file: None,
		}
	}
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, ToSchema, sqlx::FromRow)]
#[schema(example = Snippet::default)]
pub struct Layer(pub Snippet);

impl Default for Layer {
	fn default() -> Self {
		Self(Snippet {
			line: 1,
			code: String::from(r#"log("hello");"#),
			file: None,
		})
	}
}

impl Deref for Layer {
	type Target = Snippet;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
