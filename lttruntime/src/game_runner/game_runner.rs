use super::channels;
use super::game_meta::{GameMeta, ObserverConnection, PlayerConnection};
use super::{game_host, observer_connections, player_connections};
use dashmap::DashMap;
use lttcore::encoder::Encoder;
use lttcore::id::GameId;
use lttcore::{GameProgression, Play, Player};
use std::time::Duration;

#[derive(Debug)]
pub struct GameRunner<T: Play, E: Encoder> {
    games: DashMap<GameId, GameMeta<T>>,
    _phantom: std::marker::PhantomData<E>,
}

impl<T: Play, E: Encoder> GameRunner<T, E> {
    pub fn new() -> Self {
        Self {
            games: Default::default(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn spawn_game(&self, game_progression: GameProgression<T>) -> GameId {
        let game_id = GameId::new();
        let (to_game_host_msg_sender, to_game_host_msg_receiver) = channels::to_game_host();
        let (to_observer_msg_sender, to_observer_msg_receiver) = channels::to_observer();
        let (add_observer_connection_sender, add_observer_connection_receiver) =
            channels::add_connection();

        tokio::spawn(observer_connections::observer_connections::<T, E>(
            observer_connections::Inbox {
                to_observer_msg_receiver,
                add_observer_connection_receiver,
            },
            observer_connections::Outbox {
                to_game_host_msg_sender: to_game_host_msg_sender.clone(),
            },
        ));

        let (to_player_msg_senders, mut to_player_msg_receivers) =
            channels::to_players(game_progression.players());

        let (add_player_connection_senders, mut add_player_connection_receivers) =
            channels::add_player_connections(game_progression.players());

        let (from_player_msg_senders, mut from_player_msg_receivers) =
            channels::from_player_msgs(game_progression.players());

        for player in game_progression.players() {
            tokio::spawn(player_connections::player_connections::<T, E>(
                player,
                Duration::from_millis(1000),
                player_connections::Inbox {
                    from_player_msg_receiver: from_player_msg_receivers.remove(player).unwrap(),
                    to_player_msg_receiver: to_player_msg_receivers.remove(player).unwrap(),
                    add_player_connection_receiver: add_player_connection_receivers
                        .remove(player)
                        .unwrap(),
                },
                player_connections::Outbox {
                    to_game_host_msg_sender: to_game_host_msg_sender.clone(),
                },
            ));
        }

        tokio::spawn(game_host::game_host(
            game_progression,
            to_game_host_msg_receiver,
            to_player_msg_senders,
            to_observer_msg_sender,
        ));

        self.games.insert(
            game_id,
            GameMeta::new(
                add_observer_connection_sender,
                add_player_connection_senders,
                from_player_msg_senders,
            ),
        );

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