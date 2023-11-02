//! Main module for utilities
//!
//! Contains:
//! - Contains a [`W`] type that acts a newtype wrapper for implementing external traits
//!   on external types (for example the implementations in [`url_try_froms`]).
//! - An [`ArcTex`] type that is a convenience wrapper over `Arc<Mutex<T>>`
//! - A [`PeerMap`] type for tracking the connections to the websocket server, using the
//!   socket address as the key and
//! - A [`Deref`] and [`DerefMut`] implementation for all [`W`]'s
//!
//! [`ArcTex`]: arctex::ArcTex
//! [`PeerMap`]: peer_map::PeerMap

use std::ops::{Deref, DerefMut};

pub mod apidoc;
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
