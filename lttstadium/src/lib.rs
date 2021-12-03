#![allow(dead_code)]

#[macro_use]
extern crate derive_builder;

use lttcore::utilities::PlayerIndexedData as PID;
use lttcore::{bot::Contender, play::ActionResponse, pov::player::GamePlayer};
use lttcore::{
    play::{settings::NumPlayers, Play, Seed, SettingsPtr},
    pov::game_progression::GameProgression,
};
use rayon::prelude::*;
use std::panic::{catch_unwind, AssertUnwindSafe};

#[derive(Builder, Clone)]
pub struct FightCard<T: Play> {
    contenders: PID<Contender<T>>,
    #[builder(default, setter(into))]
    settings: SettingsPtr<T::Settings>,
    #[builder(default = "500")]
    iterations: usize,
}

impl<T: Play> FightCard<T> {
    pub fn run(
        &self,
        callback: impl Fn((usize, GameProgression<T>, PID<GamePlayer<T>>)) + Send + Sync,
    ) {
        (0..self.iterations)
            .into_par_iter()
            .map(|i| {
                let game_seed = Seed::random();

                let mut bots = self
                    .settings
                    .number_of_players()
                    .player_indexed_data(|player| {
                        self.contenders[player].make_stateful_bot_instance()
                    });

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

                while let Some(player_input_needed) = game.which_players_input_needed() {
                    let actions = player_input_needed.player_indexed_data(|player| {
                        let pov = &game_players[player].player_pov();
                        let seed = &bot_seeds[player];

                        // # Safety
                        //
                        // It's _probably_ not technically "unsafe" to reuse a bot who's is
                        // potentially in a weird state after it's panicked. For good measure,
                        // we resign and don't continue to reuse the bot state.
                        let mut bot_wrapper = AssertUnwindSafe(&mut bots[player]);
                        catch_unwind(move || bot_wrapper.on_action_request(pov, seed))
                            .map(ActionResponse::Response)
                            .unwrap_or_else(|_| ActionResponse::Resign)
                    });

                    let game_advance = game.submit_actions(actions);

                    for (player, game_player) in game_players.iter_mut() {
                        let update = game_advance.player_update(player);
                        game_player.update(update);
                    }
                }

                (i, game, game_players)
            })
            .for_each(callback)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lttcore::{examples::tic_tac_toe::bot::prebuilt::TicTacToePanicBot, play::Player};

    #[test]
    fn handles_panicking_bots() {
        let contenders = vec![
            (Player::new(0), Contender::new(TicTacToePanicBot)),
            (Player::new(1), Contender::new(TicTacToePanicBot)),
        ];

        let fight_card = FightCardBuilder::default()
            .iterations(1)
            .contenders(contenders.into_iter().collect())
            .build()
            .unwrap();

        fight_card.run(|_| {})
    }
}
