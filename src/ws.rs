use std::net::SocketAddr;

use crate::utils::{log_socket::LogReceiver, peer_map::PeerMap};

use futures_channel::mpsc::unbounded;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;

pub async fn handle_connection(
	peers_map: PeerMap,
	raw_stream: TcpStream,
	addr: SocketAddr,
	mut log_receiver: LogReceiver,
) {
	// TODO(depends on log server): Send logs received on the log server, and pass through
	// to the websockets.

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
		let (tx, _rx) = unbounded();
		peers_map.lock().insert(addr, tx);

		let (mut outgoing, _incoming) = stream.split();

		// let _receive_ids = incoming.try_for_each(|_| future::ok(()));

		while let Ok(log) = log_receiver.recv().await {
			let log = log.clone();

			if let Err(err) = outgoing
				.send(
					serde_json::to_string(&log)
						.expect("could not parse log into JSON")
						.into(),
				)
				.await
			{
				tracing::error!("Could not send log to front-end: {err}");
				continue;
			};
		}
	};
}
