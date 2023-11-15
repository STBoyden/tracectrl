use std::ops::Deref;

use utoipa::ToSchema;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct Snippet {
	#[schema(example = 1, minimum = 1)]
	pub line: u32,
	#[schema(example = r#"log("hello")"#)]
	pub code: String,
}

impl Default for Snippet {
	fn default() -> Self {
		Self {
			line: 1,
			code: String::from(r#"log("hello")"#),
		}
	}
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, ToSchema)]
#[schema(example = Snippet::default)]
pub struct Layer(pub Snippet);

impl Default for Layer {
	fn default() -> Self {
		Self(Snippet {
			line: 1,
			code: String::from(r#"log("hello")"#),
		})
	}
}

impl Deref for Layer {
	type Target = Snippet;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
