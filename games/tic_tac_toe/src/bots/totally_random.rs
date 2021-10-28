use crate::TicTacToe;
use lttcore::{
    bots::{Bot, BotContext},
    Play, Player, Spectator,
};
use rand::prelude::*;
use std::error::Error;

struct TotallyRandom;

impl Bot for TotallyRandom {
    type Game = TicTacToe;

    fn run<'a>(
        bot_context: BotContext<'a, Self::Game>,
        rng: &mut impl rand::Rng,
    ) -> Result<<Self::Game as Play>::Action, Box<dyn Error>> {
        let pos = bot_context
            .spectator
            .public_info
            .empty_spaces()
            .choose_stable(rng)
            .unwrap();

        Ok(pos.into())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{ttt, scenario};
//
//     scenario!(
//       TotallyRandom,
//       ttt!(
//         X O X
//         O X O
//         O X -
//       ),
//       ttt!(
//         X O X
//         O X O
//         O X O
//       )
//     )
// }
