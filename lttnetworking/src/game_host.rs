
use futures_util::{Stream, StreamExt};
use lttcore::play::{ActionResponse};
use lttcore::utilities::PlayerItemCollector;
use lttcore::{GameProgression, Play, Player};
use std::time::Instant;

pub enum GameHostRequest<T: Play> {
    RuntimeTick {
        time: Instant,
    },
    RequestObserver,
    SubmitActionResponse {
        player: Player,
        response: ActionResponse<T>,
    },
}

pub async fn game_host<T: Play>(
    mut game: GameProgression<T>,
    mut mailbox: impl Stream<Item = GameHostRequest<T>> + Unpin,
) -> GameProgression<T> {
    // initialize(&mut runtime, &mut game).await;

    while !game.is_concluded() {
        let returned_actions: PlayerItemCollector<ActionResponse<T>> =
            game.which_players_input_needed().into();

        while !returned_actions.unaccounted_for_players().is_empty() {
            match mailbox.next().await {
                None => return game,
                Some(msg) => match msg {
                    GameHostRequest::RuntimeTick { time: _ } => {
                        // runtime.tick(time).await;
                    }
                    GameHostRequest::RequestObserver => {
                        // runtime.send_observer(game.game_observer()).await
                    }
                    GameHostRequest::SubmitActionResponse { player: _, response: _ } => {
                        // returned_actions.add(player, response);
                    }
                },
            }
        }

        let _game_advance = game.submit_actions(returned_actions.into_items());
        // runtime.send_updates(game_advance).await;
    }

    game
}

// async fn initialize<T: Play>(runtime: &mut impl GameHostRuntime<T>, game: &GameProgression<T>) {
//     // runtime.send_observer(game.game_observer()).await;
//
//     for player in game.game_players() {
//         // runtime.send_game_player(player).await;
//     }
// }
