mod channels;
use crate::game_runner::GameRunner;
pub use channels::{GameRequestTicket, MatchMakerRequestReceiver, MatchMakerRequestSender};
use lttcore::encoder::Encoder;
use lttcore::Play;
use std::sync::Arc;

pub async fn run_match_maker<T: Play, E: Encoder>(
    _mailbox: MatchMakerRequestReceiver<T>,
    _game_runner: Arc<GameRunner<T, E>>,
) {
    todo!()
}
