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
//! ```
//!
//! ### Building/Testing a `TicTacToeBot`
//!
//! For bot writers, [`TicTacToe`] provides the following things
//!
//! * A simplified [Bot](`crate::bot::Bot`) wrapper called [`TicTacToeBot`]
//! * Convenience functions for testing in [`test_helpers`](`bot::test_helpers`)
//! * Prebuilt example bots in the [prebuilt](`bot::prebuilt`) module
//!
//! Note: [`TicTacToe`] is a solved game and the prebuilt bots reflect that, [`TicTacToe`] in
//! general is more designed to serve as a learning/testing example
//!
//! ```
//! use serde::{Serialize, Deserialize};
//! use lttcore::{play::Seed, ttt};
//! use lttcore::examples::tic_tac_toe::{Position, Board, TicTacToeBot};
//! use lttcore::examples::tic_tac_toe::bot::{
//!   prebuilt::RandomSelector,
//!   test_helpers::{assert_bot_takes_position, assert_bot_wins}
//! };
//!
//! #[derive(Clone, Serialize, Deserialize)]
//! struct MySuperCoolBot {
//!     favorite_number: usize,
//! }
//!
//! impl TicTacToeBot for MySuperCoolBot {
//!     fn claim_space(&self, board: &Board, seed: &Seed) -> Position {
//!         match board.at((self.favorite_number, self.favorite_number)) {
//!             Ok(None) => Position::new(self.favorite_number, self.favorite_number),
//!             _ => RandomSelector.claim_space(board, seed)
//!         }
//!     }
//! }
//!
//! let bot = MySuperCoolBot { favorite_number: 1 };
//!
//! #[rustfmt::skip]
//! let board = ttt!([
//!     - - -
//!     - - -
//!     - - -
//! ]);
//! assert_bot_takes_position(&bot, &board, (1, 1), Seed::random());
//!
//! #[rustfmt::skip]
//! let board = ttt!([
//!   X O X
//!   O - X
//!   X O O
//! ]);
//! assert_bot_wins(&bot, &board, Seed::random())
//! ```
//!
//! # Where to go now?
//!
//! The [`board`] module (and specifically the [`Board`](board::Board)) struct are good starting
//! points to learn how to interact with this game. The [`Board`](board::Board) is what
//! [`TicTacToeBot`]s are passed when they are invoked

#![warn(missing_docs)]

mod action;
pub mod board;
pub mod bot;
mod macros;
mod marker;
mod public_info;
mod settings;

pub use action::{Action, ActionError};
pub use board::{Board, Col, Position, Row, Status};
pub use bot::TicTacToeBot;
pub use marker::Marker;
pub use public_info::{PublicInfo, PublicInfoUpdate};
pub use settings::Settings;

use crate::{
    play::{
        settings::NumPlayers,
        view::{NoGameSecretInfo, NoGameSecretInfoUpdate, NoSecretPlayerInfo},
        ActionResponse, GameState, GameStateUpdate, Play, Player,
    },
    utilities::{PlayerIndexedData as PID, PlayerSet},
    LibTableTopIdentifier,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// The main tic-tac-toe struct (implements [`Play`])
///
/// see the [module](self) documentation
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicTacToe;

impl LibTableTopIdentifier for TicTacToe {
    fn lib_table_top_identifier() -> &'static str {
        "TicTacToe"
    }
}

impl Play for TicTacToe {
    type Action = Action;
    type ActionError = ActionError;
    type PublicInfo = PublicInfo;
    type PlayerSecretInfo = NoSecretPlayerInfo;
    type Settings = Settings;
    type GameSecretInfo = NoGameSecretInfo;

    fn initial_state_for_settings(
        settings: &Self::Settings,
        _rng: &mut impl rand::Rng,
    ) -> GameState<Self> {
        let public_info = PublicInfo::default();
        let action_requests = Some(PlayerSet::from(Player::from(
            public_info.board.whose_turn(),
        )));
        let player_secret_info = settings
            .number_of_players()
            .player_indexed_data(|_player| NoSecretPlayerInfo);

        GameState {
            player_secret_info,
            game_secret_info: NoGameSecretInfo,
            public_info,
            action_requests,
        }
    }

    fn resolve(
        game_state: &GameState<Self>,
        _settings: &Self::Settings,
        actions: Cow<'_, PID<ActionResponse<Self>>>,
        _rng: &mut impl rand::Rng,
    ) -> GameStateUpdate<Self> {
        use ActionResponse::{Resign, Response, Timeout};

        let (player, response) = actions
            .as_ref()
            .iter()
            .next()
            .expect("Tic Tac Toe is single player at a time");

        let marker = Marker::try_from(player).expect("only p0 or p1 was passed");
        let mut debug_msgs: PID<ActionError> = Default::default();

        let public_info_update = match response {
            Resign => PublicInfoUpdate::Resign(marker),
            Timeout => {
                let available = game_state
                    .public_info
                    .board
                    .empty_spaces()
                    .next()
                    .expect("can't resolve a full board");

                PublicInfoUpdate::Claim(marker, available)
            }
            Response(Action { position }) => {
                if game_state.public_info.board[*position].is_none() {
                    PublicInfoUpdate::Claim(marker, *position)
                } else {
                    debug_msgs.insert(
                        player,
                        ActionError::SpaceIsTaken {
                            attempted: *position,
                        },
                    );
                    let available = game_state
                        .public_info
                        .board
                        .empty_spaces()
                        .next()
                        .expect("can't resolve a full board");

                    PublicInfoUpdate::Claim(marker, available)
                }
            }
        };

        let action_requests = match &public_info_update {
            PublicInfoUpdate::Claim(marker, position) => {
                let after_move = game_state
                    .public_info
                    .board
                    .claim_space(*marker, *position)
                    .expect("we just validated this claim");

                match after_move.status() {
                    Status::InProgress { next_up } => Some(Player::from(next_up).into()),
                    Status::Win { .. } | Status::Draw { .. } | Status::WinByResignation { .. } => {
                        None
                    }
                }
            }
            PublicInfoUpdate::Resign(_) => None,
        };

        GameStateUpdate {
            debug_msgs,
            public_info_update,
            action_requests,
            player_secret_info_updates: PID::empty(),
            game_secret_info_update: NoGameSecretInfoUpdate,
        }
    }
}
