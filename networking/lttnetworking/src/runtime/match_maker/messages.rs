use lttcore::id::GameId;
use lttcore::play::{LttSettings, Mode};
use tokio::sync::oneshot::Receiver as OneShotReceiver;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub struct MatchMakerRequest<T: LttSettings> {
    mode: Mode<T>,
    _phantom: std::marker::PhantomData<T>
}

pub type MatchMakerTicket = OneShotReceiver<GameId>;
pub type MatchMakerRequestReceiver<T> = UnboundedReceiver<MatchMakerRequest<T>>;
pub type MatchMakerRequestSender<T> = UnboundedSender<MatchMakerRequest<T>>;
