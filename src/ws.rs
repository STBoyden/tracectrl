use std::net::SocketAddr;

use crate::utils::peer_map::PeerMap;

use futures_channel::mpsc::unbounded;
use futures_util::{future, stream::TryStreamExt, StreamExt};
use tokio::net::TcpStream;

pub async fn handle_connection(
	peers_map: PeerMap,
	raw_stream: TcpStream,
	addr: SocketAddr,
) {
	tracing::debug!("Recieved connection from {addr}");

	'inner: {
		let stream = match tokio_tungstenite::accept_async(raw_stream).await {
			Ok(stream) => stream,
			Err(err) => {
				tracing::error!("Could not establish websocket handshake with {addr}: {err}");
				break 'inner;
			}
		};

		tracing::debug!("Websocket connection established with {addr}");
		let (tx, rx) = unbounded();
		peers_map.lock().insert(addr, tx);

		let (outgoing, incoming) = stream.split();

		let receive_ids = incoming.try_for_each(|msg| {
			_ = msg;

			future::ok(())
		});
	};
}
