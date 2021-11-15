use lttcore::id::GameId;
use lttcore::play::Mode;
use lttcore::Play;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot::Receiver as OneShotReceiver;

pub struct MatchMakerRequest<T: Play> {
    mode: Mode<T>,
    _phantom: std::marker::PhantomData<T>,
}

pub type MatchMakerTicket = OneShotReceiver<GameId>;
pub type MatchMakerRequestReceiver<T> = UnboundedReceiver<MatchMakerRequest<T>>;
pub type MatchMakerRequestSender<T> = UnboundedSender<MatchMakerRequest<T>>;
