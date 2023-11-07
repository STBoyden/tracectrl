use std::{net::SocketAddr, ops::Deref};

use crate::utils::arctex::ArcTex;

use dashmap::DashMap;
use futures_channel::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::protocol::Message;

type Tx = UnboundedSender<Message>;
type PeerMapInner = ArcTex<DashMap<SocketAddr, Tx>>;

#[derive(Debug, Clone)]
pub struct PeerMap(PeerMapInner);

impl PeerMap {
	pub fn new() -> Self {
		Self(ArcTex::new(DashMap::new()))
	}
}

impl Deref for PeerMap {
	type Target = PeerMapInner;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
