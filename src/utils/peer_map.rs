use std::{net::SocketAddr, ops::Deref, sync::Arc};

use dashmap::DashMap;
use futures_channel::mpsc::UnboundedSender;
use parking_lot::Mutex;
use tokio_tungstenite::tungstenite::protocol::Message;

pub type Tx = UnboundedSender<Message>;
type PeerMapInner = Arc<Mutex<DashMap<SocketAddr, Tx>>>;

#[derive(Clone, Debug)]
pub struct PeerMap(PeerMapInner);

impl PeerMap {
	pub fn new() -> Self {
		Self(Arc::new(Mutex::new(DashMap::new())))
	}
}
unsafe impl Send for PeerMap {}
impl Deref for PeerMap {
	type Target = PeerMapInner;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
