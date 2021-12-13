//! A collection of prebuilt [`TicTacToeBot`]
//!
//! tic-tac-toe is a solved game, and this bot collection reflects that. These mostly take the fun
//! out of writing your own bot because these implementations are pretty solid. These bots (and the
//! `TicTacToe` game in general mostly exist to serve as an example for other game implementations.

use std::fmt::Display;

use super::super::{Board, Position, TicTacToe, TicTacToeBot};
use crate::{
    bot::{defective::panicking_bot, BotContext},
    examples::tic_tac_toe::{
        board::consts::{BOTTOM_LEFT, BOTTOM_RIGHT, CENTER},
        Marker,
    },
};
use rand::prelude::IteratorRandom;
use serde::{Deserialize, Serialize};

macro_rules! display_name {
    ($ty:ty) => {
        impl Display for $ty {
            fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                write!(fmt, "{}", stringify!($ty))
            }
        }
    };
}

panicking_bot!(TicTacToe);
display_name!(TicTacToePanicBot);

/// A bot that  randomly picks an open space
///
/// ... Not very good strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RandomSelector;

display_name!(RandomSelector);

impl TicTacToeBot for RandomSelector {
    fn claim_space(&self, board: &Board, context: &BotContext<'_, TicTacToe>) -> Position {
        board
            .empty_spaces()
            .choose_stable(&mut context.rng_for_turn())
            .expect("the bot won't be called if the board is full")
    }
}

/// An itermediate level bot
///
/// This bot always chooses the center if going first. From there this bot will try to block an
/// opponent from winning if the opponent has a square they could win on, or take a spot that would
/// win them the game. If all else fails this bot chooses a square randomly
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntermediateSkill;

display_name!(IntermediateSkill);

impl TicTacToeBot for IntermediateSkill {
    fn claim_space(&self, board: &Board, context: &BotContext<'_, TicTacToe>) -> Position {
        // Pick the center space if we're starting
        if board.is_empty() {
            return CENTER;
        }

        // Take the winning spot, or block opponent from winning
        //
        // Note: if exists two disjoint squares one giving a win to this bot and the other giving a
        // win to the opponent, it will pick the square that happens to show up first in
        // `board.triples()`
        for triple in board.triples() {
            match triple {
                [(_, Some(a)), (_, Some(b)), (pos, None)] if a == b => return pos,
                [(_, Some(a)), (pos, None), (_, Some(b))] if a == b => return pos,
                [(pos, None), (_, Some(a)), (_, Some(b))] if a == b => return pos,
                _ => continue,
            }
        }

        // Else, choose randomly
        RandomSelector.claim_space(board, context)
    }
}

/// A perfect bot
///
/// This bot will win if possible and draw if not
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpertSkill;

display_name!(ExpertSkill);

impl TicTacToeBot for ExpertSkill {
    fn claim_space(&self, board: &Board, context: &BotContext<'_, TicTacToe>) -> Position {
        // Take a corner if we're the first move
        if board.is_empty() {
            return BOTTOM_LEFT;
        }

        let me: Marker = board.whose_turn();

        // Look for winning spots
        for triple in board.triples() {
            match triple {
                [(_, Some(a)), (_, Some(b)), (pos, None)] if a == b && a == me => return pos,
                [(_, Some(a)), (pos, None), (_, Some(b))] if a == b && a == me => return pos,
                [(pos, None), (_, Some(a)), (_, Some(b))] if a == b && a == me => return pos,
                _ => continue,
            }
        }

        // Look to block
        for triple in board.triples() {
            match triple {
                [(_, Some(a)), (_, Some(b)), (pos, None)] if a == b => return pos,
                [(_, Some(a)), (pos, None), (_, Some(b))] if a == b => return pos,
                [(pos, None), (_, Some(a)), (_, Some(b))] if a == b => return pos,
                _ => continue,
            }
        }

        // If I have bottom left, take bottom right if available
        if board[BOTTOM_LEFT] == Some(me) && board[BOTTOM_RIGHT].is_none() {
            return BOTTOM_RIGHT;
        }

        // If the middle is open, take it
        if board[CENTER].is_none() {
            return CENTER;
        }

        // We will never get here because one of the above conditions must have been ture
        RandomSelector.claim_space(board, context)
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::{assert_bot_takes_position, assert_bot_wins};
    use crate::{
        examples::tic_tac_toe::board::consts::MIDDLE_LEFT,
        play::seed::{SEED_0, SEED_42},
        ttt,
    };

    use super::*;

    #[test]
    fn test_random_selector() {
        #[rustfmt::skip]
        let board = ttt!([
            - - -
            - - -
            - - -
        ]);

        assert_bot_takes_position(&RandomSelector, &board, (1, 0), SEED_42);

        #[rustfmt::skip]
        let board = ttt!([
            X X O
            O O -
            X O X
        ]);

        assert_bot_takes_position(&RandomSelector, &board, (2, 1), SEED_42);
    }

    #[test]
    fn test_intermediate() {
        // Always claims the middle
        #[rustfmt::skip]
        let board = ttt!([
            - - -
            - - -
            - - -
        ]);

        assert_bot_takes_position(&IntermediateSkill, &board, CENTER, SEED_0);
        assert_bot_takes_position(&IntermediateSkill, &board, CENTER, SEED_42);

        // Block opponent from winning
        #[rustfmt::skip]
        let board = ttt!([
            O - -
            - - -
            X - O
        ]);

        assert_bot_takes_position(&IntermediateSkill, &board, CENTER, SEED_0);
        assert_bot_takes_position(&IntermediateSkill, &board, CENTER, SEED_42);

        // Takes win if possible
        #[rustfmt::skip]
        let board = ttt!([
            O - O
            - - -
            X - X
        ]);

        assert_bot_wins(&IntermediateSkill, &board, SEED_0);
        assert_bot_wins(&IntermediateSkill, &board, SEED_42);

        // Doesn't freak out if there isn't an obvious move
        #[rustfmt::skip]
        let board = ttt!([
            O X O
            - - -
            X - -
        ]);

        assert_bot_takes_position(&IntermediateSkill, &board, BOTTOM_RIGHT, SEED_42);
    }

    #[test]
    fn test_expert() {
        // It always takes the bottom left
        let board = ttt!([
            - - -
            - - -
            - - -
        ]);

        assert_bot_takes_position(&ExpertSkill, &board, BOTTOM_LEFT, SEED_42);

        // If we do have the the bottom left, take the bottom right
        let board = ttt!([
            - - -
            - O -
            X - -
        ]);

        assert_bot_takes_position(&ExpertSkill, &board, BOTTOM_RIGHT, SEED_42);

        // If we don't have the bottom left, take the center
        let board = ttt!([
            - - -
            - - -
            O - -
        ]);

        assert_bot_takes_position(&ExpertSkill, &board, CENTER, SEED_42);

        // It springs the trap if opponent fell for it
        let board = ttt!([
            - - -
            O - -
            X O X
        ]);

        assert_bot_takes_position(&ExpertSkill, &board, CENTER, SEED_42);

        // Blocks opponent win
        let board = ttt!([
            O - -
            - - X
            O X -
        ]);

        assert_bot_takes_position(&ExpertSkill, &board, MIDDLE_LEFT, SEED_42);

        // Take a win over stopping opponents win
        let board = ttt!([X - X - --O - O]);

        assert_bot_wins(&ExpertSkill, &board, SEED_42);
    }
}
