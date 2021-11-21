use super::{Action, Board, Marker, Position, TicTacToe};
use crate::bots::Bot;
use crate::pov::PlayerPov;
use crate::Play;

pub trait TicTacToeBot {
    fn claim_space(marker: Marker, board: &Board, rng: &mut impl rand::Rng) -> Position;
}

#[derive(Debug)]
pub struct TicTacToeBotWrapper<T: TicTacToeBot>(pub T);

impl<T: TicTacToeBot> Bot for TicTacToeBotWrapper<T> {
    type Game = TicTacToe;

    fn run(
        player_pov: &PlayerPov<'_, Self::Game>,
        rng: &mut impl rand::Rng,
    ) -> <Self::Game as Play>::Action {
        let marker = Marker::try_from(player_pov.player).expect("is a valid player for TicTacToe");
        let position = T::claim_space(marker, player_pov.public_info.board(), rng);

        Action { position }
    }
}
