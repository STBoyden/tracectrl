use std::collections::BTreeMap;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Trace {
	layers: BTreeMap<u32, String>,
}

impl Trace {
	pub fn new() -> Self {
		Self {
			layers: BTreeMap::new(),
		}
	}
}
