use crate::messages::MatchMakerRequest;
use lttcore::id::GameId;

use lttcore::Player;

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot::Receiver as OneShotReceiver;

pub type GameRequestTicket = OneShotReceiver<(GameId, Player)>;
pub type MatchMakerRequestReceiver<T> = UnboundedReceiver<MatchMakerRequest<T>>;
pub type MatchMakerRequestSender<T> = UnboundedSender<MatchMakerRequest<T>>;
