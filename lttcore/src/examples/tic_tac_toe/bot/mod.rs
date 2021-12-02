//! Working with tic-tac-toe bot players

pub mod prebuilt;
pub mod test_helpers;

mod tic_tac_toe_bot;
pub use tic_tac_toe_bot::{TicTacToeBot, TicTacToeBotWrapper};

mod tic_tac_toe_with_history_bot;
pub use tic_tac_toe_with_history_bot::{TicTacToeWithHistoryBot, TicTacToeWithHistoryBotWrapper};
