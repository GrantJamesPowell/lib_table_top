use futures_util::{Sink, SinkExt, Stream, StreamExt};
use lttcore::play::{ActionResponse, EnumeratedGameAdvance};
use lttcore::pov::{ObserverUpdate, PlayerUpdate};
use lttcore::utilities::{PlayerIndexedData as PID, PlayerItemCollector as PIC};
use lttcore::{GameObserver, GamePlayer, GameProgression, Play, Player};

pub enum GameHostMsg<T: Play> {
    RequestObserverState,
    RequestPlayerState {
        player: Player,
    },
    SubmitActionResponse {
        player: Player,
        response: ActionResponse<T>,
    },
}

pub enum PlayerMsg<T: Play> {
    SyncState(GamePlayer<T>),
    Update(PlayerUpdate<'static, T>),
}

pub enum ObserverMsg<T: Play> {
    SyncState(GameObserver<T>),
    Update(ObserverUpdate<'static, T>),
}

use GameHostMsg::*;

pub async fn server_player<T: Play>(
    player: Player,
    mut mailbox: impl Stream<Item = PlayerMsg<T>> + Unpin,
    mut to_game_host: impl Sink<GameHostMsg<T>> + Unpin,
    mut to_client: impl Sink<PlayerMsg<T>> + Unpin,
) {
    use PlayerMsg::*;

    send(&mut to_game_host, RequestPlayerState { player }).await;

    while let Some(msg) = mailbox.next().await {
        match msg {
            SyncState(_) => {
                send(&mut to_client, msg).await;
            }
            Update(_) => {}
        }
    }
}

pub async fn run_game_host<T: Play>(
    mut game: GameProgression<T>,
    mut mailbox: impl Stream<Item = GameHostMsg<T>> + Unpin,
    mut to_players: PID<impl Sink<PlayerMsg<T>> + Unpin>,
    mut to_observer: impl Sink<ObserverMsg<T>> + Unpin,
) -> GameProgression<T> {
    while !game.is_concluded() {
        let mut returned_actions: PIC<ActionResponse<T>> = game.which_players_input_needed().into();

        while !returned_actions.unaccounted_for_players().is_empty() {
            match mailbox.next().await {
                None => return game,
                Some(msg) => match msg {
                    RequestObserverState => {
                        let msg = ObserverMsg::SyncState(game.game_observer());
                        send(&mut to_observer, msg).await;
                    }
                    RequestPlayerState { player } => {
                        for gp in game.game_players().filter(|x| x.player() == player) {
                            let player = gp.player();
                            send(&mut to_players[player], PlayerMsg::SyncState(gp)).await;
                        }
                    }
                    SubmitActionResponse { player, response } => {
                        returned_actions.add(player, response);
                    }
                },
            }
        }

        let game_advance = game.submit_actions(returned_actions.into_items());
        send_update(game_advance, &mut to_players, &mut to_observer).await;
    }

    game
}

async fn initialize<T: Play>(
    game: &GameProgression<T>,
    to_players: &mut PID<impl Sink<PlayerMsg<T>> + Unpin>,
    to_observer: &mut (impl Sink<ObserverMsg<T>> + Unpin),
) {
    send(to_observer, ObserverMsg::SyncState(game.game_observer())).await;

    for game_player in game.game_players() {
        let player = game_player.player();
        send(&mut to_players[player], PlayerMsg::SyncState(game_player)).await;
    }
}

async fn send_update<T: Play>(
    update: EnumeratedGameAdvance<T>,
    to_players: &mut PID<impl Sink<PlayerMsg<T>> + Unpin>,
    to_observer: &mut (impl Sink<ObserverMsg<T>> + Unpin),
) {
    let observer_update = update.observer_update().into_owned();
    send(to_observer, ObserverMsg::Update(observer_update)).await;

    for (player, to_player) in to_players.iter_mut() {
        let player_update = update.player_update(player).into_owned();
        send(to_player, PlayerMsg::Update(player_update)).await;
    }
}

async fn send<T>(sink: &mut (impl Sink<T> + Unpin), msg: T) {
    match sink.send(msg).await {
        Ok(_) => {},
        Err(_) => panic!("GameHost can't send on a channel, and I can't figure out how to force Stream::Error to be debug")
    }
}
