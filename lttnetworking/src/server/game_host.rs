use super::messages::{
    GameHostMsg::{self, *},
    ObserverMsg, PlayerMsg,
};
use lttcore::play::{ActionResponse, EnumeratedGameAdvance};
use lttcore::utilities::{PlayerIndexedData as PID, PlayerItemCollector as PIC};
use lttcore::{GameProgression, Play};
use tokio::sync::mpsc::{Receiver, Sender};

pub async fn run_game_host<T: Play>(
    mut game: GameProgression<T>,
    mut mailbox: Receiver<GameHostMsg<T>>,
    to_players: PID<Sender<PlayerMsg<T>>>,
    to_observer: Sender<ObserverMsg<T>>,
) -> GameProgression<T> {
    while !game.is_concluded() {
        let mut returned_actions: PIC<ActionResponse<T>> = game.which_players_input_needed().into();

        while !returned_actions.unaccounted_for_players().is_empty() {
            match mailbox.recv().await {
                None => return game,
                Some(msg) => match msg {
                    RequestObserverState => {
                        let msg = ObserverMsg::SyncState(game.game_observer());
                        to_observer.send(msg).await.unwrap();
                    }
                    RequestPlayerState { player } => {
                        for gp in game.game_players().filter(|x| x.player() == player) {
                            let player = gp.player();
                            to_players[player]
                                .send(PlayerMsg::SyncState(gp))
                                .await
                                .unwrap();
                        }
                    }
                    SubmitActionResponse { player, response } => {
                        returned_actions.add(player, response);
                    }
                },
            }
        }

        let game_advance = game.submit_actions(returned_actions.into_items());
        send_update(game_advance, &to_players, &to_observer).await;
    }

    game
}

async fn send_update<T: Play>(
    update: EnumeratedGameAdvance<T>,
    to_players: &PID<Sender<PlayerMsg<T>>>,
    to_observer: &Sender<ObserverMsg<T>>,
) {
    let observer_update = update.observer_update().into_owned();
    to_observer
        .send(ObserverMsg::Update(observer_update))
        .await
        .unwrap();

    for (player, to_player) in to_players.iter() {
        let player_update = update.player_update(player).into_owned();
        to_player
            .send(PlayerMsg::Update(player_update))
            .await
            .unwrap();
    }
}
