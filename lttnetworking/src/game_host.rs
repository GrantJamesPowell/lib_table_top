use futures_util::{Sink, SinkExt, Stream, StreamExt};
use lttcore::play::{ActionResponse, EnumeratedGameAdvance};
use lttcore::pov::{ObserverUpdate, PlayerUpdate};
use lttcore::utilities::{PlayerIndexedData as PID, PlayerItemCollector as PIC};
use lttcore::{GameObserver, GamePlayer, GameProgression, Play, Player};
use smallvec::SmallVec;

use uuid::Uuid;

pub type Connections<Stream> = SmallVec<[(Uuid, Stream); 2]>;

pub enum GameHostRequest<T: Play> {
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

use GameHostRequest::*;

pub async fn game_host<T: Play>(
    mut game: GameProgression<T>,
    mut mailbox: impl Stream<Item = GameHostRequest<T>> + Unpin,
    mut players: PID<impl Sink<PlayerMsg<T>> + Unpin>,
    mut observer: impl Sink<ObserverMsg<T>> + Unpin,
) -> GameProgression<T> {
    initialize(&mut game, &mut players, &mut observer).await;

    while !game.is_concluded() {
        let mut returned_actions: PIC<ActionResponse<T>> = game.which_players_input_needed().into();

        while !returned_actions.unaccounted_for_players().is_empty() {
            match mailbox.next().await {
                None => return game,
                Some(msg) => match msg {
                    SubmitActionResponse { player, response } => {
                        returned_actions.add(player, response);
                    }
                },
            }
        }

        let game_advance = game.submit_actions(returned_actions.into_items());
        send_update(game_advance, &mut players, &mut observer).await;
    }

    game
}

async fn initialize<T: Play>(
    game: &GameProgression<T>,
    players: &mut PID<impl Sink<PlayerMsg<T>> + Unpin>,
    observer: &mut (impl Sink<ObserverMsg<T>> + Unpin),
) {
    send(observer, ObserverMsg::SyncState(game.game_observer())).await;

    for game_player in game.game_players() {
        let player = game_player.player();
        send(&mut players[player], PlayerMsg::SyncState(game_player)).await;
    }
}

async fn send_update<T: Play>(
    update: EnumeratedGameAdvance<T>,
    players: &mut PID<impl Sink<PlayerMsg<T>> + Unpin>,
    observer: &mut (impl Sink<ObserverMsg<T>> + Unpin),
) {
    let observer_update = update.observer_update().into_owned();
    send(observer, ObserverMsg::Update(observer_update)).await;

    for (player, sink) in players.iter_mut() {
        let player_update = update.player_update(player).into_owned();
        send(sink, PlayerMsg::Update(player_update)).await;
    }
}

async fn send<T>(sink: &mut (impl Sink<T> + Unpin), msg: T) {
    match sink.send(msg).await {
        Ok(_) => {},
        Err(_) => panic!("GameHost can't send on a channel, and I can't figure out how to force Stream::Error to be debug")
    }
}
