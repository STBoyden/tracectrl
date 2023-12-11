use std::collections::BTreeMap;

use utoipa::ToSchema;

use crate::api::types::{Layer, Snippet};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct Trace {
	#[schema(example=json!([Layer::default()]))]
	pub layers: Vec<Layer>,
}

impl From<BTreeMap<i32, String>> for Trace {
	fn from(value: BTreeMap<i32, String>) -> Self {
		let mut layers = vec![];

		for (key, value) in &value {
			layers.push(Layer(Snippet {
				line: *key,
				code: value.clone(),
			}));
		}

		Self { layers }
	}
}
