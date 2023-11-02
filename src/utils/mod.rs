use std::ops::{Deref, DerefMut};

pub mod arctex;
pub mod peer_map;
mod url_try_froms;

pub struct W<T>(pub T);

impl<T> Deref for W<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T> DerefMut for W<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
