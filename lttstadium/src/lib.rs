#![allow(dead_code)]

#[macro_use]
extern crate derive_builder;

use lttcore::utilities::PlayerIndexedData as PID;
use lttcore::{bot::Bot, play::ActionResponse, pov::player::GamePlayer};
use lttcore::{
    play::{settings::NumPlayers, Play, Seed, SettingsPtr},
    pov::game_progression::GameProgression,
};
use rayon::prelude::*;
use std::sync::Arc;

#[derive(Builder, Clone)]
pub struct FightCard<T: Play> {
    bots: PID<Arc<dyn Bot<Game = T> + Send + Sync + 'static>>,
    settings: SettingsPtr<T::Settings>,
    iterations: usize,
}

impl<T: Play> FightCard<T> {
    fn run(&self, callback: impl Fn((GameProgression<T>, PID<GamePlayer<T>>)) + Send + Sync) {
        (0..self.iterations)
            .into_par_iter()
            .map(|_i| {
                let game_seed = Seed::random();

                let bot_seeds = self
                    .settings
                    .number_of_players()
                    .player_indexed_data(|_| Seed::random());

                let mut game: GameProgression<T> =
                    GameProgression::from_settings_and_seed(self.settings.clone(), game_seed);

                let mut game_players = self
                    .settings
                    .number_of_players()
                    .player_indexed_data(|p| game.game_player(p));

                while !game.is_concluded() {
                    let actions = game
                        .which_players_input_needed()
                        .player_indexed_data(|player| {
                            let pov = game_players[player].player_pov();
                            let action = self.bots[player].run(&pov, &bot_seeds[player]);
                            ActionResponse::Response(action)
                        });

                    let game_advance = game.submit_actions(actions);

                    for (player, game_player) in game_players.iter_mut() {
                        let update = game_advance.player_update(player);
                        game_player.update(update);
                    }
                }

                (game, game_players)
            })
            .for_each(callback)
    }
}
