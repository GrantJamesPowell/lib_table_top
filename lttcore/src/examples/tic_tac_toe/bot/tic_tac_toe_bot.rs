use crate::bot::{Bot, BotContext, BotError};
use crate::examples::tic_tac_toe::{Action, Board, Position, TicTacToe};

use crate::pov::player::PlayerPov;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{fmt::Display, panic::RefUnwindSafe};

/// A simplified [`Bot`](`crate::bot::Bot`) wrapper specialized for playing [`TicTacToe`]
pub trait TicTacToeBot:
    RefUnwindSafe + Clone + Sync + Send + 'static + Serialize + DeserializeOwned
{
    /// Method to choose which [`Position`] to claim given a [`Board`] and a [`BotContext`]. Your
    /// bot will only be called when it's your turn to make a move, so [`Board::whose_turn`] will
    /// be the marker that represents your bots. For examples checkout the
    /// [`prebuilt`](super::prebuilt) module
    fn claim_space(&self, board: &Board, context: &BotContext<'_, TicTacToe>) -> Position;

    /// Turn the type that implements [`TicTacToeBot`] into one that implements [`Bot`] for
    /// `Bot<Game = TicTacToe>`
    fn into_bot(self) -> TicTacToeBotWrapper<Self> {
        TicTacToeBotWrapper(self)
    }
}

/// Wrapper type to implement [`Bot`](`crate::bot::Bot`) where `Bot::Game = TicTacToe` for any [`TicTacToeBot`]
///
/// You likely will only interact with this type through [`TicTacToeBot::into_bot`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct TicTacToeBotWrapper<T: TicTacToeBot>(T);

impl<T: TicTacToeBot + Display> Display for TicTacToeBotWrapper<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        // Just use the inner attribute's `Display`
        write!(fmt, "{}", self.0)
    }
}

impl<T: TicTacToeBot> Bot for TicTacToeBotWrapper<T> {
    type Game = TicTacToe;

    fn on_action_request(
        &mut self,
        player_pov: &PlayerPov<'_, TicTacToe>,
        bot_context: &BotContext<'_, TicTacToe>,
    ) -> Result<Action, BotError<TicTacToe>> {
        let position = self
            .0
            .claim_space(&player_pov.public_info.board, bot_context);
        Ok(Action { position })
    }
}
