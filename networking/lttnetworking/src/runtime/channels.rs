use bytes::Bytes;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub type ByteSink = UnboundedSender<Bytes>;
pub type ByteStream = UnboundedReceiver<Bytes>;
