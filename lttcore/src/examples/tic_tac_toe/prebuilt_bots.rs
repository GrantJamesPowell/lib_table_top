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
            return (1, 1).try_into().expect("center square is on the board");
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
    use crate::{
        examples::tic_tac_toe::ActionError,
        seed::{SEED_0, SEED_42},
        ttt,
    };

    use super::*;

    #[test]
    fn test_random_selector() {
        #[rustfmt::skip]
        let before = ttt!([
            - - -
            - - -
            - - -
        ]);

        #[rustfmt::skip]
        let after = ttt!([
            - - -
            - - -
            - X -
        ]);

        test_bot(RandomSelector, before, after, SEED_42);

        #[rustfmt::skip]
        let before = ttt!([
            X X O
            O O -
            X O X
        ]);

        #[rustfmt::skip]
        let after = ttt!([
            X X O
            O O X
            X O X
        ]);

        test_bot(RandomSelector, before, after, SEED_42);
    }

    #[test]
    fn test_intermediate() {
        // Always claims the middle
        #[rustfmt::skip]
        let before = ttt!([
            - - -
            - - -
            - - -
        ]);

        #[rustfmt::skip]
        let after = ttt!([
            - - -
            - X -
            - - -
        ]);

        test_bot(Intermediate, before, after, SEED_0);
        test_bot(Intermediate, before, after, SEED_42);

        // Block opponent from winning
        #[rustfmt::skip]
        let before = ttt!([
            O - -
            - - -
            X - O
        ]);

        #[rustfmt::skip]
        let after = ttt!([
            O - - 
            - X -
            X - O
        ]);

        test_bot(Intermediate, before, after, SEED_0);
        test_bot(Intermediate, before, after, SEED_42);

        // Takes win if possible
        #[rustfmt::skip]
        let before = ttt!([
            O - O
            - - -
            X - X
        ]);

        #[rustfmt::skip]
        let after = ttt!([
            O - O
            - - -
            X X X
        ]);

        test_bot(Intermediate, before, after, SEED_0);
        test_bot(Intermediate, before, after, SEED_42);

        // Doesn't freak out if there isn't an obvious move
        #[rustfmt::skip]
        let before = ttt!([
            O X O
            - - -
            X - -
        ]);

        #[rustfmt::skip]
        let after = ttt!([
            O X O
            - - -
            X - X
        ]);

        test_bot(Intermediate, before, after, SEED_42);
    }

    fn test_bot(bot: impl TicTacToeBot, mut before: Board, after: Board, seed: Seed) {
        let pos = bot.claim_space(&before, seed);
        match before.claim_space(before.whose_turn(), pos) {
            Err(ActionError::SpaceIsTaken { .. }) => {
                panic!("Bot tried to claim space {} but it was already taken", pos)
            }
            Err(ActionError::AllSpacesTaken) => {
                panic!("Test was given a full starting board which is invalid")
            }
            Ok(_) => {
                assert_eq!(
                    before, after,
                    "Bot picked {} instead of the expected position",
                    pos
                )
            }
        }
    }
}
