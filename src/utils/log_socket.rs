use tokio::sync::broadcast::{self, Receiver, Sender};

use crate::api::types::Log;

pub type LogSender = Sender<Log>;
pub type LogReceiver = Receiver<Log>;

pub fn new_log_socket() -> (LogSender, LogReceiver) {
	let (tx, rx) = broadcast::channel(16);

	(tx, rx)
}
