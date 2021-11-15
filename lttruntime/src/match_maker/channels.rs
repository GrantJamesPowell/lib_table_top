use crate::messages::MatchMakerRequest;
use lttcore::Player;
use lttcore::id::GameId;
use tokio::sync::{oneshot, mpsc};

pub type GameRequestTicket = oneshot::Receiver<(GameId, Player)>;
pub type GameRequestTicketResolver = oneshot::Sender<(GameId, Player)>;

pub type MatchMakerRequestReceiver<T> = mpsc::UnboundedReceiver<
    (MatchMakerRequest<T>, GameRequestTicketResolver)
>;
pub type MatchMakerRequestSender<T> = mpsc::UnboundedSender<
    (MatchMakerRequest<T>, GameRequestTicketResolver)
>;
