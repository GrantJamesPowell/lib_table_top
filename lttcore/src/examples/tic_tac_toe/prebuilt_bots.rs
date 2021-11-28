use rand::prelude::IteratorRandom;

use super::{Board, Position, TicTacToeBot};
use crate::Seed;

/// A bot that just randomly picks an open space... Not very good strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RandomSelector;

impl TicTacToeBot for RandomSelector {
    fn claim_space(&self, board: &Board, seed: Seed) -> Position {
        board
            .empty_spaces()
            .choose_stable(&mut seed.rng())
            .expect("the bot won't be called if the board is full")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Intermediate;

impl TicTacToeBot for Intermediate {
    fn claim_space(&self, board: &Board, seed: Seed) -> Position {
        // Pick the center space if we're starting
        if board.is_empty() {
            return Position::new(1, 1);
        }

        // Take the winning spot, or block opponent from winning
        for triple in board.triples() {
            match triple {
                [(_, Some(a)), (_, Some(b)), (pos, None)] if a == b => return pos,
                [(_, Some(a)), (pos, None), (_, Some(b))] if a == b => return pos,
                [(pos, None), (_, Some(a)), (_, Some(b))] if a == b => return pos,
                _ => continue,
            }
        }

        // Else, choose randomly
        RandomSelector.claim_space(board, seed)
    }
}

#[cfg(test)]
mod tests {
    use super::super::bot_test_helpers::{assert_bot_takes_position, assert_bot_wins};
    use crate::{
        seed::{SEED_0, SEED_42},
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

        assert_bot_takes_position(RandomSelector, board, Position::new(1, 0), SEED_42);

        #[rustfmt::skip]
        let board = ttt!([
            X X O
            O O -
            X O X
        ]);

        assert_bot_takes_position(RandomSelector, board, Position::new(2, 1), SEED_42);
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

        assert_bot_takes_position(Intermediate, board, Position::new(1, 1), SEED_0);
        assert_bot_takes_position(Intermediate, board, Position::new(1, 1), SEED_42);

        // Block opponent from winning
        #[rustfmt::skip]
        let board = ttt!([
            O - -
            - - -
            X - O
        ]);

        assert_bot_takes_position(Intermediate, board, Position::new(1, 1), SEED_0);
        assert_bot_takes_position(Intermediate, board, Position::new(1, 1), SEED_42);

        // Takes win if possible
        #[rustfmt::skip]
        let board = ttt!([
            O - O
            - - -
            X - X
        ]);

        assert_bot_wins(Intermediate, board, SEED_0);
        assert_bot_wins(Intermediate, board, SEED_42);

        // Doesn't freak out if there isn't an obvious move
        #[rustfmt::skip]
        let board = ttt!([
            O X O
            - - -
            X - -
        ]);

        assert_bot_takes_position(Intermediate, board, Position::new(2, 0), SEED_42);
    }
}
