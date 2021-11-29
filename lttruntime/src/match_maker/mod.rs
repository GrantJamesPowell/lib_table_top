mod channels;
use crate::game_runner::GameRunner;
pub use channels::{GameRequestTicket, MatchMakerRequestReceiver, MatchMakerRequestSender};
use lttcore::play::Play;
use std::sync::Arc;

pub async fn run_match_maker<T: Play>(
    _mailbox: MatchMakerRequestReceiver,
    _game_runner: Arc<GameRunner<T>>,
) {
    todo!()
}
