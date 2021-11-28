//! An implementation of [tic-tac-toe](https://en.wikipedia.org/wiki/Tic-tac-toe)
//!
//! # Why is this here?
//!
//! [`TicTacToe`] is designed to be used as a reference implementation of how a
//! board game should implement the [`Play`] trait. It is also useful to test higher level
//! features that operate on things implementing [`Play`].
//!
//! # What other board games is tic-tac-toe similar to?
//!
//! [`TicTacToe`] has the following properties which may or may not make it a good example to
//! follow for a board game you're interested in implementing
//!
//! * tic-tac-toe is not configurable, there are no custom settings.
//! * tic-tac-toe has no secret information
//! * tic-tac-toe has sequential turns (never simultaneous)
//!
//! # Quick examples
//!
//! ### Board Literals via `ttt!` Macro
//!
//! [`TicTacToe`] provides the `ttt!` macro which allows for [`Board`] literals to be used in
//! source code.
//!
//! ```
//! use lttcore::ttt;
//! use lttcore::examples::tic_tac_toe::{Status, Marker::*};
//!
//! #[rustfmt::skip]
//! let board = ttt!([
//!   X - X
//!   - O -
//!   O - -
//! ]);
//!
//! assert_eq!(board.whose_turn(), X);
//! assert_eq!(board.status(), Status::InProgress { next_up: X });
//! assert!(false)
//! ```
//!
//! ### Building/Testing a TicTacToeBot
//!
//! For bot writers, [`TicTacToe`] provides the following things
//!
//! * A simplified [Bot](`crate::bots::Bot`) wrapper called [`TicTacToeBot`]
//! * Convenience functions for testing in [test_helpers](`bot::test_helpers`)
//! * Prebuilt example bots in the [prebuilt](`bot::prebuilt`) module
//!
//! Note: [`TicTacToe`] is a solved game and the prebuilt bots reflect that, [`TicTacToe`] in
//! general is more designed to serve as a learning/testing example
//!
//! ```
//! use lttcore::{Seed, ttt};
//! use lttcore::examples::tic_tac_toe::{Position, Board, TicTacToeBot};
//! use lttcore::examples::tic_tac_toe::bot::{
//!   prebuilt::RandomSelector,
//!   test_helpers::{assert_bot_takes_position, assert_bot_wins}
//! };
//!
//! struct MySuperCoolBot {
//!     favorite_number: usize,
//! }
//!
//! impl TicTacToeBot for MySuperCoolBot {
//!     fn claim_space(&self, board: &Board, seed: Seed) -> Position {
//!         if let Some(pos) = Position::try_new(self.favorite_number, self.favorite_number) {
//!             if board[pos].is_none() {
//!                 return pos;
//!             }
//!         }
//!
//!         RandomSelector.claim_space(board, seed)
//!     }
//! }
//!
//! #[rustfmt::skip]
//! let board = ttt!([
//!     - - -
//!     - - -
//!     - - -
//! ]);
//!
//! assert_bot_takes_position(
//!     MySuperCoolBot { favorite_number: 1 },
//!     board,
//!     Position::new(1, 1),
//!     Seed::random(),
//! );
//!
//! #[rustfmt::skip]
//! let board = ttt!([
//!   X O X
//!   O - X
//!   X O O
//! ]);
//!
//! assert_bot_wins(MySuperCoolBot { favorite_number: 1 }, board, Seed::random())
//! ```

mod action;
pub mod board;
pub mod bot;
mod macros;
mod marker;
mod public_info;
mod settings;

pub use action::{Action, ActionError};
pub use board::{Board, Col, Position, Row, Status};
pub use bot::{TicTacToeBot, TicTacToeBotWrapper};
pub use marker::Marker;
pub use public_info::{PublicInfo, PublicInfoUpdate};
pub use settings::Settings;

use crate::play::view::NoSecretPlayerInfo;
use crate::{
    play::{ActionResponse, DebugMsgs, GameAdvance},
    Play, Player, PlayerSet,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicTacToe {
    board: Board,
    resigned: Option<Marker>,
}

impl From<Board> for TicTacToe {
    fn from(board: Board) -> Self {
        Self {
            board,
            resigned: None,
        }
    }
}

impl TicTacToe {
    /// Resigns a player, ending the game
    ///
    /// ```
    /// use lttcore::Play;
    /// use lttcore::examples::tic_tac_toe::{TicTacToe, Status::*, Marker::*, PublicInfoUpdate::*};
    ///
    /// let mut game: TicTacToe = Default::default();
    /// assert_eq!(game.status(), InProgress{ next_up: X });
    /// assert_eq!(game.resign(X), Resign(X));
    /// assert_eq!(game.status(), WinByResignation { winner: O });
    /// ```
    pub fn resign(&mut self, marker: Marker) -> PublicInfoUpdate {
        self.resigned = Some(marker);
        PublicInfoUpdate::Resign(marker)
    }

    pub fn resigned(&self) -> Option<Marker> {
        self.resigned.clone()
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Returns the status of the current game (taking into account resignation info)
    /// see `Board::status` for more info
    /// ```
    /// use lttcore::ttt;
    /// use lttcore::examples::tic_tac_toe::{TicTacToe, Status::*, Marker::*};
    ///
    /// let mut game: TicTacToe = Default::default();
    /// game.resign(X);
    /// assert_eq!(game.status(), WinByResignation { winner: O });
    /// ```
    pub fn status(&self) -> Status {
        if let Some(loser) = self.resigned() {
            Status::WinByResignation {
                winner: loser.opponent(),
            }
        } else {
            self.board().status()
        }
    }

    /// Claims a space for a marker, returns an error if that space is taken
    ///
    /// ```
    /// use lttcore::examples::tic_tac_toe::{TicTacToe, Marker::*, Position, ActionError::*};
    ///
    /// let mut game: TicTacToe = Default::default();
    /// let pos = Position::new(0, 0);
    ///
    /// assert_eq!(game.board()[pos], None);
    /// assert!(game.claim_space(X, pos).is_ok());
    /// assert_eq!(game.board()[pos], Some(X.into()));
    ///
    /// // Taking an already claimed space returns an error
    /// assert_eq!(game.claim_space(O, pos), Err(SpaceIsTaken { attempted: pos.into() }));
    /// ```
    pub fn claim_space(
        &mut self,
        marker: Marker,
        position: impl Into<Position>,
    ) -> Result<PublicInfoUpdate, ActionError> {
        let position = position.into();
        self.board
            .claim_space(marker, position)
            .map(|_| PublicInfoUpdate::Claim(marker, position))
    }

    /// Claims the next available space on the board.
    /// Designed to be deterministic to be used for defaulting moves
    ///
    /// ```
    /// use lttcore::ttt;
    /// use lttcore::examples::tic_tac_toe::{TicTacToe, Marker::*, PublicInfoUpdate::*, Position};
    ///
    /// let mut game: TicTacToe = ttt!([
    ///     - - -
    ///     - - -
    ///     - - -
    /// ]).into();
    ///
    /// let update = game.claim_next_available_space(X).unwrap();
    /// assert_eq!(update, Claim(X, Position::new(0, 0)));
    ///
    /// assert_eq!(
    ///   game.board(),
    ///   &ttt!([
    ///     - - -
    ///     - - -
    ///     X - -
    ///   ])
    /// );
    ///
    /// game.claim_next_available_space(O).unwrap();
    /// game.claim_next_available_space(X).unwrap();
    /// game.claim_next_available_space(O).unwrap();
    /// game.claim_next_available_space(X).unwrap();
    /// game.claim_next_available_space(O).unwrap();
    ///
    /// assert_eq!(
    ///   game.board(),
    ///   &ttt!([
    ///     X O -
    ///     O X -
    ///     X O -
    ///   ])
    /// );
    /// ```
    pub fn claim_next_available_space(
        &mut self,
        marker: Marker,
    ) -> Result<PublicInfoUpdate, ActionError> {
        let position = self
            .board
            .empty_spaces()
            .next()
            .ok_or(ActionError::AllSpacesTaken)?;

        self.claim_space(marker, position)
    }
}

impl Play for TicTacToe {
    type Action = Action;
    type ActionError = ActionError;
    type PublicInfo = PublicInfo;
    type PlayerSecretInfo = NoSecretPlayerInfo;
    type Settings = Settings;

    fn string_id() -> &'static str {
        "TicTacToe"
    }

    fn which_players_input_needed(&self, _settings: &Self::Settings) -> PlayerSet {
        match self.status() {
            Status::InProgress { next_up } => Player::from(next_up).into(),
            _ => Default::default(),
        }
    }

    fn public_info(&self, _settings: &Self::Settings) -> Cow<'_, Self::PublicInfo> {
        Cow::Owned(PublicInfo::from(*self))
    }

    fn initial_state_for_settings(
        _settings: &<Self as Play>::Settings,
        _rng: &mut impl rand::Rng,
    ) -> Self {
        Default::default()
    }

    fn player_secret_info(
        &self,
        _: &<Self as Play>::Settings,
        _: Player,
    ) -> Cow<'_, Self::PlayerSecretInfo> {
        Cow::Owned(Default::default())
    }

    fn advance<'a>(
        &'a mut self,
        settings: &Self::Settings,
        mut actions: impl Iterator<Item = (Player, Cow<'a, ActionResponse<Self>>)>,
        _rng: &mut impl rand::Rng,
    ) -> GameAdvance<Self> {
        use ActionResponse::{Resign, Response, Timeout};

        let (player, response) = actions
            .next()
            .expect("Tic Tac Toe is single player at a time");

        let marker = Marker::try_from(player).expect("only p0 or p1 was passed");

        let mut debug_msgs: DebugMsgs<Self> = Default::default();

        let public_info_update = {
            match response.as_ref() {
                Resign => self.resign(marker),
                Timeout => self
                    .claim_next_available_space(marker)
                    .expect("Tried to apply an action to a full board"),

                Response(Action { position }) => match self.claim_space(marker, *position) {
                    Ok(update) => update,
                    Err(err) => {
                        debug_msgs.insert(player, err);
                        self.claim_next_available_space(marker)
                            .expect("Tried to apply an action to a full board")
                    }
                },
            }
        };

        GameAdvance {
            debug_msgs,
            public_info_update,
            next_players_input_needed: self.which_players_input_needed(settings),
            player_secret_info_updates: Default::default(),
        }
    }
}
