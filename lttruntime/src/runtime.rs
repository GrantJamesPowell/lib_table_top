use super::game_runner::{GameRunner, ObserverConnection, PlayerConnection};
use super::match_maker::match_maker::MatchMakerQueues;
use lttcore::encoder::Encoder;
use lttcore::{id::GameId, Play, Player};
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

    pub fn play_game(&self, game_id: GameId, player: Player) -> Option<PlayerConnection<T>> {
        self.game_runner.play_game(game_id, player)
    }

    pub fn observe_game(&self, game_id: GameId) -> Option<ObserverConnection> {
        self.game_runner.observe_game(game_id)
    }
}
