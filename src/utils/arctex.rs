use std::{ops::Deref, sync::Arc};

use parking_lot::Mutex;

#[derive(Debug, Clone)]
pub struct ArcTex<T>(Arc<Mutex<T>>);

impl<T> ArcTex<T> {
	pub fn new(val: T) -> Self {
		Self(Arc::new(Mutex::new(val)))
	}
}

impl<T> Deref for ArcTex<T> {
	type Target = Arc<Mutex<T>>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
