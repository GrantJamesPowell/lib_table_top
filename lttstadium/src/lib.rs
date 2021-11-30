#![allow(dead_code)]

#[macro_use]
extern crate derive_builder;

use lttcore::utilities::PlayerIndexedData as PID;
use lttcore::{
    bot::{Bot, Contender},
    play::ActionResponse,
    pov::player::GamePlayer,
};
use lttcore::{
    play::{settings::NumPlayers, Play, Seed, SettingsPtr},
    pov::game_progression::GameProgression,
};
use rayon::prelude::*;
use std::panic::catch_unwind;
use std::panic::AssertUnwindSafe;

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

                let bot_seeds = self
                    .settings
                    .number_of_players()
                    .player_indexed_data(|_| Seed::random());

                let mut bots: PID<Box<dyn Bot<Game = T>>> = self
                    .contenders
                    .iter()
                    .map(|(player, contender)| (player, contender.make_bot_instance()))
                    .collect();

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

                            // On `UnwindSafety`, it is _hella_ not safe to continue to use the bot
                            // state if the bot panics, thats why we _immediately_ resign if the
                            // bot panics while calculating a move
                            let mut bot_wrapper = AssertUnwindSafe(&mut bots[player]);
                            let seed = &bot_seeds[player];

                            catch_unwind(move || bot_wrapper.run(&pov, seed))
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
            (Player::new(0), Contender::new("panic", TicTacToePanicBot)),
            (Player::new(1), Contender::new("panic", TicTacToePanicBot)),
        ];

        let fight_card = FightCardBuilder::default()
            .iterations(1)
            .contenders(contenders.into_iter().collect())
            .build()
            .unwrap();

        fight_card.run(|_| {})
    }
}
