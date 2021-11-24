use super::game_runner::GameRunner;
use super::match_maker::{run_match_maker, GameRequestTicket, MatchMakerRequestSender};
use crate::messages::MatchMakerRequest;
use crate::{ObserverConnection, PlayerConnection};
use lttcore::encoder::Encoding;
use lttcore::{id::GameId, Play, Player};
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};

pub struct Runtime<T: Play> {
    game_runner: Arc<GameRunner<T>>,
    match_maker_request_sender: MatchMakerRequestSender<T>,
}

impl<T: Play> Runtime<T> {
    pub fn start() -> Self {
        let game_runner = Arc::new(GameRunner::new());
        let (match_maker_request_sender, match_maker_request_receiver) = mpsc::unbounded_channel();

        tokio::spawn(run_match_maker::<T>(
            match_maker_request_receiver,
            Arc::clone(&game_runner),
        ));

        Self {
            game_runner,
            match_maker_request_sender,
        }
    }

    pub fn match_make(&self, request: MatchMakerRequest<T>) -> GameRequestTicket {
        let (resolver, ticket) = oneshot::channel();

        self.match_maker_request_sender
            .send((request, resolver))
            .expect("match maker hasn't failed");

        ticket
    }

    pub fn play_game(
        &self,
        game_id: GameId,
        player: Player,
        encoding: Encoding,
    ) -> Option<PlayerConnection<T>> {
        self.game_runner.play_game(game_id, player, encoding)
    }

    pub fn observe_game(&self, game_id: GameId, encoding: Encoding) -> Option<ObserverConnection> {
        self.game_runner.observe_game(game_id, encoding)
    }
}
