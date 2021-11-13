use super::game_meta::GameMeta;

use crate::runtime::{ObserverConnection, PlayerConnection};

use dashmap::DashMap;
use lttcore::id::GameId;

use lttcore::{Play, Player};

#[derive(Debug)]
pub struct Runtime<T: Play> {
    games: DashMap<GameId, GameMeta<T>>,
}

impl<T: Play> Runtime<T> {
    // pub fn spawn_game(&self, game_progression: GameProgression<T>) -> GameId {
    //     let _game_id = GameId::new();

    //     let (_to_game_host, game_host_mailbox) = unbounded_channel::<ToGameHostMsg<T>>();
    //     let (to_observer, _observer_mailbox) = unbounded_channel::<ToObserverMsg<T>>();
    //     let (to_players, _players_mailboxs): (PID<_>, PID<_>) = game_progression
    //         .players()
    //         .into_iter()
    //         .map(|player| {
    //             let (to_player, player_mailbox) = unbounded_channel::<ToPlayerMsg<T>>();
    //             ((player, to_player), (player, player_mailbox))
    //         })
    //         .unzip();

    //     tokio::spawn(game_host::game_host(
    //         game_progression,
    //         game_host_mailbox,
    //         to_players,
    //         to_observer,
    //     ));

    //     todo!()
    // }

    pub fn observe_game(&self, game_id: GameId) -> Option<ObserverConnection> {
        self.games.get(&game_id).map(|meta| meta.add_observer())
    }

    pub fn play_game(&self, game_id: GameId, player: Player) -> Option<PlayerConnection<T>> {
        self.games
            .get(&game_id)
            .and_then(|meta| meta.add_player(player))
    }
}
