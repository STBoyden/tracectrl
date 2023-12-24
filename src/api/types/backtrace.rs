use utoipa::ToSchema;

use crate::api::types::Layer;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct Trace {
	pub layers: Vec<Layer>,
}
