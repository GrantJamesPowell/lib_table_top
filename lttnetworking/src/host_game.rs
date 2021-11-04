use async_trait::async_trait;
use futures_util::{Stream, StreamExt};
use lttcore::play::{ActionResponse, EnumeratedGameAdvance};
use lttcore::utilities::PlayerItemCollector;
use lttcore::{GameObserver, GamePlayer, GameProgression, Play, Player};
use std::time::Instant;

#[async_trait(?Send)]
pub trait GameHostRuntime<T: Play + Send>: Send {
    async fn send_game_player(&mut self, _game_player: GamePlayer<T>) {}
    async fn send_observer(&mut self, _observer: GameObserver<T>) {}
    async fn send_updates(&mut self, _game_advance: EnumeratedGameAdvance<T>) {}
    async fn tick(&mut self, _time: Instant) {}
}

pub enum GameHostRequest<T: Play> {
    RuntimeTick { time: Instant },
    RequestObserver,
    SubmitActionResponse {
        player: Player,
        response: ActionResponse<T>,
    },
}

pub async fn host_game<T: Play>(
    mut game: GameProgression<T>,
    mut mailbox: impl Stream<Item = GameHostRequest<T>> + Unpin,
    mut runtime: impl GameHostRuntime<T>,
) -> GameProgression<T> {
    initialize(&mut runtime, &mut game).await;

    while !game.is_concluded() {
        let mut returned_actions: PlayerItemCollector<ActionResponse<T>> =
            game.which_players_input_needed().into();

        while !returned_actions.unaccounted_for_players().is_empty() {
            match mailbox.next().await {
                None => return game,
                Some(msg) => match msg {
                    GameHostRequest::RuntimeTick { time } => {
                        runtime.tick(time).await;
                    }
                    GameHostRequest::RequestObserver => {
                        runtime.send_observer(game.game_observer()).await
                    }
                    GameHostRequest::SubmitActionResponse { player, response } => {
                        returned_actions.add(player, response);
                    }
                },
            }
        }

        let game_advance = game.submit_actions(returned_actions.into_items());
        runtime.send_updates(game_advance).await;
    }

    game
}

async fn initialize<T: Play>(runtime: &mut impl GameHostRuntime<T>, game: &GameProgression<T>) {
    runtime.send_observer(game.game_observer()).await;

    for player in game.game_players() {
        runtime.send_game_player(player).await;
    }
}
