use crate::messages::MatchMakerRequest;
use lttcore::id::GameId;
use lttcore::Player;
use tokio::sync::{mpsc, oneshot};

pub type GameRequestTicket = oneshot::Receiver<(GameId, Player)>;
pub type GameRequestTicketResolver = oneshot::Sender<(GameId, Player)>;

pub type MatchMakerRequestReceiver =
    mpsc::UnboundedReceiver<(MatchMakerRequest, GameRequestTicketResolver)>;
pub type MatchMakerRequestSender =
    mpsc::UnboundedSender<(MatchMakerRequest, GameRequestTicketResolver)>;
