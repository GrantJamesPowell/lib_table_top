#![allow(dead_code)]

#[macro_use]
extern crate derive_builder;

use lttcore::{bot::BotContextBuilder, utilities::PlayerIndexedData as PID};
use lttcore::{bot::Contender, play::ActionResponse};
use lttcore::{
    play::{settings::NumPlayers, Play, Seed, SettingsPtr},
    pov::game_progression::GameProgression,
};
use rayon::prelude::*;
use std::{
    panic::{catch_unwind, AssertUnwindSafe},
    time::Duration,
};

#[derive(Builder, Clone)]
pub struct FightCard<T: Play> {
    contenders: PID<Contender<T>>,
    #[builder(default, setter(into))]
    settings: SettingsPtr<T::Settings>,
    #[builder(default = "500")]
    iterations: usize,
    #[builder(default = "Duration::from_millis(500)")]
    bot_action_duration: Duration,
}

impl<T: Play> FightCard<T> {
    pub fn run(&self, callback: impl Fn((usize, GameProgression<T>)) + Send + Sync) {
        (0..self.iterations)
            .into_par_iter()
            .map(|i| {
                let game_seed = Seed::random();

                let mut bots = self
                    .settings
                    .number_of_players()
                    .player_indexed_data(|player| self.contenders[player].make_bot_instance());

                let bot_seeds = self
                    .settings
                    .number_of_players()
                    .player_indexed_data(|_| Seed::random());

                let mut game: GameProgression<T> =
                    GameProgression::from_settings_and_seed(self.settings.clone(), game_seed);

                while !game.is_concluded() {
                    let actions = game
                        .which_players_input_needed()
                        .map(|player| {
                            let pov = &game.player_pov(player);

                            let context = BotContextBuilder::default()
                                .seed(&bot_seeds[player])
                                .time_budget(self.bot_action_duration)
                                .turn_num(game.turn_num())
                                .build()
                                .unwrap();

                            // # Safety
                            //
                            // It's _probably_ not technically "unsafe" to reuse a bot who's is
                            // potentially in a weird state after it's panicked. For good measure,
                            // we resign and don't continue to reuse the bot state.
                            let mut bot_wrapper = AssertUnwindSafe(&mut bots[player]);
                            let action =
                                catch_unwind(move || bot_wrapper.on_action_request(pov, &context))
                                    .map(ActionResponse::Response)
                                    .unwrap_or_else(|_| ActionResponse::Resign);

                            (player, action)
                        })
                        .collect();

                    let update = game.resolve(actions);
                    game.update(update);
                }

                (i, game)
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
