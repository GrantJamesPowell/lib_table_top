use super::super::{ActionError, Board, Position, Status, TicTacToeBot};
use crate::Seed;

/// Test helper that asserts a bot will make a winning move on a certain board
#[track_caller]
pub fn assert_bot_wins(bot: &impl TicTacToeBot, mut board: Board, seed: Seed) {
    if let Status::InProgress { next_up } = board.status() {
        let pos = bot.claim_space(&board, seed);
        match board.claim_space(next_up, pos) {
            Err(ActionError::SpaceIsTaken { .. }) => {
                panic!("Bot tried to claim space {} but it was already taken", pos)
            }
            Err(ActionError::AllSpacesTaken) => {
                panic!("Test was given a full starting board which is invalid")
            }
            Ok(_) => {
                match board.status() {
                    Status::Win { .. } => {
                        // Success
                    }
                    Status::Draw => {
                        panic!("Bot picked position {} which resulted in a draw", pos)
                    }
                    Status::InProgress { .. } => {
                        panic!("Bot picked position {} which did not end the game", pos)
                    }
                    Status::WinByResignation { .. } => unreachable!(),
                }
            }
        }
    } else {
        panic!("Game was already over")
    }
}

#[track_caller]
pub fn assert_bot_takes_position(
    bot: &impl TicTacToeBot,
    mut before: Board,
    expected: impl TryInto<Position>,
    seed: Seed,
) {
    let expected = expected
        .try_into()
        .unwrap_or_else(|_| panic!("expected was not within the bounds of the board"));
    let pos = bot.claim_space(&before, seed);
    match before.claim_space(before.whose_turn(), pos) {
        Err(ActionError::SpaceIsTaken { .. }) => {
            panic!("Bot tried to claim space {} but it was already taken", pos);
        }
        Err(ActionError::AllSpacesTaken) => {
            panic!("Test was given a full starting board which is invalid");
        }
        Ok(_) => {
            assert_eq!(
                pos, expected,
                "Bot picked {} instead of the expected position {}",
                pos, expected
            );
        }
    }
}
