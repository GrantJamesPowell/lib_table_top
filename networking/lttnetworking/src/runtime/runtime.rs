use super::game_meta::GameMeta;
use crate::runtime::channels;
use crate::runtime::{game_host, observer_connections, player_connections};
use crate::runtime::{ObserverConnection, PlayerConnection};
use dashmap::DashMap;
use lttcore::encoder::Encoder;
use lttcore::id::GameId;
use lttcore::{GameProgression, Play, Player};

#[derive(Debug)]
pub struct Runtime<T: Play, E: Encoder> {
    games: DashMap<GameId, GameMeta<T>>,
    _phantom: std::marker::PhantomData<E>,
}

impl<T: Play, E: Encoder> Runtime<T, E> {
    pub fn spawn_game(&self, game_progression: GameProgression<T>) -> GameId {
        let game_id = GameId::new();
        let (to_game_host_msg_sender, to_game_host_msg_receiver) = channels::to_game_host();
        let (to_observer_msg_sender, to_observer_msg_receiver) = channels::to_observer();
        let (player_msg_senders, player_msg_receivers) =
            channels::to_players(game_progression.players());

        let (add_observer_connection_sender, add_observer_connection_receiver) =
            channels::add_connection();

        tokio::spawn(observer_connections::observer_connections::<T, E>(
            observer_connections::Inbox {
                to_observer_msg_receiver,
                add_observer_connection_receiver,
            },
            observer_connections::Outbox {
                to_game_host_msg_sender,
            },
        ));

        tokio::spawn(game_host::game_host(
            game_progression,
            to_game_host_msg_receiver,
            player_msg_senders,
            to_observer_msg_sender,
        ));

        game_id
    }

    pub fn observe_game(&self, game_id: GameId) -> Option<ObserverConnection> {
        self.games.get(&game_id).map(|meta| meta.add_observer())
    }

    pub fn play_game(&self, game_id: GameId, player: Player) -> Option<PlayerConnection<T>> {
        self.games
            .get(&game_id)
            .and_then(|meta| meta.add_player(player))
    }
}
