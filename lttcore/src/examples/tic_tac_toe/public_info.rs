use super::{Board, Marker, Position, Status};
use crate::{
    play::{Player, Score, View},
    utilities::PlayerIndexedData as PID,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// [`PublicInfo`](crate::play::Play::PublicInfo) of the [`TicTacToe`] game
///
/// This [`Deref`]s to a [`TicTacToe`] struct so you can use the immutable methods on
/// [`TicTacToe`] on this struct
#[derive(Clone, Copy, Default, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicInfo {
    /// The current [`Board`]
    pub board: Board,
    /// The [`Marker`] who gave up, if any
    pub resigned: Option<Marker>,
}

impl Score for PublicInfo {
    fn is_score_human_interpertable() -> bool {
        false
    }

    fn score(&self) -> Option<PID<u64>> {
        self.status().winner().map(|winner| {
            [
                (Player::from(winner), 1),
                (Player::from(winner.opponent()), 0),
            ]
            .into_iter()
            .collect()
        })
    }
}

impl PublicInfo {
    /// Returns the status of the current game (taking into account resignation info)
    /// see [`Board::status`] for more info
    /// ```
    /// use lttcore::ttt;
    /// use lttcore::examples::tic_tac_toe::{PublicInfo, Status::*, Marker::*};
    ///
    /// let mut game = PublicInfo::default();
    /// game.resigned = Some(X);
    /// assert_eq!(game.status(), WinByResignation { winner: O });
    /// ```
    pub fn status(&self) -> Status {
        self.resigned.map_or_else(
            || self.board.status(),
            |loser| Status::WinByResignation {
                winner: loser.opponent(),
            },
        )
    }
}

/// Update to the [`PublicInfo`]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicInfoUpdate {
    /// A [`Marker`] has resigned from the game
    Resign(Marker),
    /// A [`Marker`] has claimed a [`Position`]
    Claim(Marker, Position),
}

impl View for PublicInfo {
    type Update = PublicInfoUpdate;

    fn update(&mut self, update: Cow<'_, Self::Update>) {
        match update.as_ref() {
            PublicInfoUpdate::Resign(marker) => {
                self.resigned = Some(*marker);
            }
            PublicInfoUpdate::Claim(marker, position) => {
                self.board = self
                    .board
                    .claim_space(*marker, *position)
                    .expect("ttt recieves a valid PublicInfoUpdate");
            }
        }
    }
}
