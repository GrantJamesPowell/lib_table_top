use super::game_runner::GameRunner;
use super::match_maker::match_maker::MatchMakerQueues;
use lttcore::encoder::Encoder;
use lttcore::Play;
use std::sync::Arc;

pub struct Runtime<T: Play, E: Encoder> {
    game_runner: Arc<GameRunner<T, E>>,
    match_maker_queues: MatchMakerQueues<T>,
    // match_maker_request_sender: MatchMakerRequestSender,
}

impl<T: Play, E: Encoder> Runtime<T, E> {
    pub fn start() -> Self {
        let game_runner = Arc::new(GameRunner::new());
        let match_maker_queues = MatchMakerQueues::new();

        Self {
            game_runner,
            match_maker_queues,
        }
    }
}
