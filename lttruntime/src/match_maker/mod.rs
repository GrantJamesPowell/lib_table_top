mod channels;
use crate::game_runner::GameRunner;
pub use channels::{GameRequestTicket, MatchMakerRequestReceiver, MatchMakerRequestSender};
use lttcore::Play;
use std::sync::Arc;

pub async fn run_match_maker<T: Play>(
    _mailbox: MatchMakerRequestReceiver<T>,
    _game_runner: Arc<GameRunner<T>>,
) {
    todo!()
}
