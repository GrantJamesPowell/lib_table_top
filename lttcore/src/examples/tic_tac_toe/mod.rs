mod action;
mod board;
pub mod helpers;
mod marker;
mod public_info;
mod settings;

pub use action::{Action, ActionError};
pub use board::{BoardIndex, Col, Position, Row, POSSIBLE_WINS};
pub use marker::Marker;
pub use public_info::{PublicInfo, PublicInfoUpdate};
pub use settings::Settings;

use crate::play::view::NoSecretPlayerInfo;
use crate::{
    play::{ActionResponse, DebugMsgs, GameAdvance},
    utilities::number_of_players::TWO_PLAYER,
    NumberOfPlayers, Play, Player, PlayerSet,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    /// There are still available positions to be claimed on the board
    InProgress { next_up: Player },
    /// All positions have been claimed and there is no winner
    Draw,
    /// Win by resignation
    WinByResignation { winner: Player },
    /// There *is* a winner via connecting three spaces
    Win {
        winner: Player,
        positions: [Position; 3],
    },
}

impl Status {
    pub fn winner(&self) -> Option<Player> {
        match self {
            Status::Win { winner, .. } | Status::WinByResignation { winner, .. } => Some(*winner),
            _ => None,
        }
    }
}

use Status::{Draw, InProgress, Win, WinByResignation};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicTacToe {
    board: [[Option<Player>; 3]; 3],
    resigned: PlayerSet,
}

impl TicTacToe {
    /// Resigns a player, ending the game
    ///
    /// ```
    /// use lttcore::Play;
    /// use lttcore::examples::tic_tac_toe::{TicTacToe, Status::*, Marker::*, PublicInfoUpdate::*};
    ///
    /// let mut game: TicTacToe = Default::default();
    /// assert_eq!(game.status(), InProgress{ next_up: X.into() });
    /// assert_eq!(game.resign(X), Resign(X.into()));
    /// assert_eq!(game.status(), WinByResignation { winner: O.into() });
    /// ```
    pub fn resign(&mut self, player: impl Into<Player>) -> PublicInfoUpdate {
        let player = player.into();
        self.resigned.insert(player);
        PublicInfoUpdate::Resign(player)
    }

    pub fn resigned(&self) -> &PlayerSet {
        &self.resigned
    }

    /// Returns the status of the current game
    /// ```
    /// use lttcore::{Play, Player};
    /// use lttcore::ttt;
    /// use lttcore::examples::tic_tac_toe::{TicTacToe, Row, Col, Status::*, Marker::*};
    ///
    /// // In progress
    /// let game: TicTacToe = Default::default();
    /// assert_eq!(game.status(), InProgress{ next_up: X.into() });
    ///
    /// // A draw
    /// let game: TicTacToe = ttt!([
    ///   O X O
    ///   X X O
    ///   X O X
    /// ]);
    /// assert_eq!(game.status(), Draw);
    ///
    /// // If someone resigns
    /// let mut game: TicTacToe = Default::default();
    /// game.resign(X);
    /// assert_eq!(game.status(), WinByResignation { winner: O.into() });
    ///
    /// // With a winning position
    /// let game: TicTacToe = ttt!([
    ///   - - -
    ///   - - -
    ///   X X X
    /// ]);
    ///
    /// assert_eq!(
    ///   game.status(),
    ///   Win {
    ///     winner: X.into(),
    ///     positions: [
    ///       (Col::new(0), Row::new(0)),
    ///       (Col::new(0), Row::new(1)),
    ///       (Col::new(0), Row::new(2))
    ///     ]
    ///   }
    /// );
    /// ```
    pub fn status(&self) -> Status {
        if let Some(loser) = self.resigned().players().next() {
            return WinByResignation {
                winner: helpers::opponent(loser),
            };
        }

        POSSIBLE_WINS
            .iter()
            .find_map(|&positions| {
                let [a, b, c] = positions.map(|pos| self.at_position(pos));

                if a == b && b == c {
                    a.map(|winner| Win { winner, positions })
                } else {
                    None
                }
            })
            .unwrap_or_else(|| {
                if self.has_open_spaces() {
                    InProgress {
                        next_up: self.whose_turn(),
                    }
                } else {
                    Draw
                }
            })
    }

    /// Claims a space for a marker, returns an error if that space is taken
    ///
    /// ```
    /// use lttcore::examples::tic_tac_toe::{TicTacToe, Marker::*, Col, Row, ActionError::*};
    ///
    /// let mut game: TicTacToe = Default::default();
    /// let pos = (Col::new(0), Row::new(0));
    ///
    /// assert_eq!(game.at_position(pos), None);
    /// assert!(game.claim_space(X, pos).is_ok());
    /// assert_eq!(game.at_position(pos), Some(X.into()));
    ///
    /// // Taking an already claimed space returns an error
    /// assert_eq!(game.claim_space(O, pos), Err(SpaceIsTaken { attempted: pos }));
    /// ```
    pub fn claim_space(
        &mut self,
        player: impl Into<Player>,
        position: Position,
    ) -> Result<PublicInfoUpdate, ActionError> {
        let player = player.into();

        if self.at_position(position).is_some() {
            return Err(ActionError::SpaceIsTaken {
                attempted: position,
            });
        }

        let (c, r) = position;
        let (c, r): (usize, usize) = (c.into(), r.into());
        self.board[c][r] = Some(player);
        Ok(PublicInfoUpdate::Claim(player, position))
    }

    /// Claims the next available space on the board.
    /// Designed to be deterministic to be used for defaulting moves
    ///
    /// ```
    /// use lttcore::ttt;
    /// use lttcore::examples::tic_tac_toe::{Marker::*, PublicInfoUpdate::*, Col, Row};
    ///
    /// let mut game = ttt!([
    ///     - - -
    ///     - - -
    ///     - - -
    /// ]);
    ///
    /// let update = game.claim_next_available_space(X).unwrap();
    /// assert_eq!(update, Claim(0.into(), (Col::new(0), Row::new(0))));
    ///
    /// assert_eq!(
    ///   game,
    ///   ttt!([
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
    ///   game,
    ///   ttt!([
    ///     - - -
    ///     O X O
    ///     X O X
    ///   ])
    /// );
    /// ```
    pub fn claim_next_available_space(
        &mut self,
        player: impl Into<Player>,
    ) -> Result<PublicInfoUpdate, ActionError> {
        let position = self
            .empty_spaces()
            .next()
            .ok_or(ActionError::AllSpacesTaken)?;
        self.claim_space(player, position)
    }

    /// Returns the marker at a position, since this requires [`Row`] and [`Col`] structs
    /// the indexing will always be inbound
    ///
    /// ```
    /// use lttcore::ttt;
    /// use lttcore::examples::tic_tac_toe::{Row, Col, Marker::*};
    ///
    /// let game = ttt!([
    ///   X - -
    ///   - - -
    ///   - X O
    /// ]);
    /// assert_eq!(game.at_position((Col::new(2), Row::new(0))), Some(X.into()));
    /// assert_eq!(game.at_position((Col::new(0), Row::new(2))), Some(O.into()));
    /// assert_eq!(game.at_position((Col::new(0), Row::new(0))), None);
    /// ```
    pub fn at_position(&self, (c, r): Position) -> Option<Player> {
        let c: usize = c.into();
        let r: usize = r.into();
        self.board[c][r]
    }

    /// Returns a marker at a position, if the row or col is greater than 2, this returns None
    ///
    /// ```
    /// use lttcore::ttt;
    /// use lttcore::examples::tic_tac_toe::{Row, Col, Marker::*};
    ///
    /// let game = ttt!([
    ///   X - -
    ///   - - -
    ///   - X O
    /// ]);
    /// assert_eq!(game.at((2, 0)), Some(X.into()));
    /// assert_eq!(game.at((0, 2)), Some(O.into()));
    /// assert_eq!(game.at((0, 0)), None);
    ///
    /// // Out of bounds numbers return None
    /// assert_eq!(game.at((0, 1000)), None);
    /// assert_eq!(game.at((1000, 0)), None);
    /// ```
    pub fn at(&self, (c, r): (usize, usize)) -> Option<Player> {
        let col = Col::try_new(c)?;
        let row = Row::try_new(r)?;

        self.at_position((col, row))
    }

    /// Iterator over the empty spaces on the board
    ///
    /// ```
    /// use lttcore::ttt;
    /// use lttcore::examples::tic_tac_toe::{TicTacToe, Row, Col, Marker::*, Position};
    ///
    /// let game: TicTacToe = Default::default();
    /// assert_eq!(game.empty_spaces().count(), 9);
    ///
    /// let game = ttt!([
    ///   X X X
    ///   X X X
    ///   X X X
    /// ]);
    /// assert_eq!(game.empty_spaces().count(), 0);
    ///
    /// let game = ttt!([
    ///   X O X
    ///   - - -
    ///   - X O
    /// ]);
    /// assert_eq!(game.empty_spaces().count(), 4);
    /// assert_eq!(
    ///   game.empty_spaces().collect::<Vec<_>>(),
    ///   vec![
    ///    (Col::new(0), Row::new(0)),
    ///    (Col::new(1), Row::new(0)),
    ///    (Col::new(1), Row::new(1)),
    ///    (Col::new(1), Row::new(2))
    ///   ]
    /// );
    /// ```
    pub fn empty_spaces(&self) -> impl Iterator<Item = Position> + '_ {
        self.spaces().filter_map(|(pos, player)| match player {
            None => Some(pos),
            Some(_) => None,
        })
    }

    /// Iterate over the spaces on the board and the marker in the space (if there is one)
    ///
    /// ```
    /// use lttcore::ttt;
    /// use lttcore::examples::tic_tac_toe::{Row, Col, Marker::*, Position};
    ///
    /// let game = ttt!([
    ///   X O X
    ///   - - -
    ///   - X O
    /// ]);
    /// assert_eq!(
    ///   game.spaces().collect::<Vec<_>>(),
    ///   vec![
    ///     ((Col::new(0), Row::new(0)), None),
    ///     ((Col::new(0), Row::new(1)), Some(X.into())),
    ///     ((Col::new(0), Row::new(2)), Some(O.into())),
    ///     ((Col::new(1), Row::new(0)), None),
    ///     ((Col::new(1), Row::new(1)), None),
    ///     ((Col::new(1), Row::new(2)), None),
    ///     ((Col::new(2), Row::new(0)), Some(X.into())),
    ///     ((Col::new(2), Row::new(1)), Some(O.into())),
    ///     ((Col::new(2), Row::new(2)), Some(X.into()))
    ///   ]
    /// );
    /// ```
    pub fn spaces(&self) -> impl Iterator<Item = (Position, Option<Player>)> + '_ {
        self.board.iter().enumerate().flat_map(|(col_num, col)| {
            col.iter()
                .enumerate()
                .map(move |(row_num, &player)| ((Col::new(col_num), Row::new(row_num)), player))
        })
    }

    /// Iterate over the spaces on the board that are taken
    ///
    /// ```
    /// use lttcore::ttt;
    /// use lttcore::examples::tic_tac_toe::{Row, Col, Marker::*};
    ///
    /// let game = ttt!([
    ///   X O X
    ///   - - -
    ///   - X O
    /// ]);
    /// assert_eq!(
    ///   game.taken_spaces().collect::<Vec<_>>(),
    ///   vec![
    ///     ((Col::new(0), Row::new(1)), X.into()),
    ///     ((Col::new(0), Row::new(2)), O.into()),
    ///     ((Col::new(2), Row::new(0)), X.into()),
    ///     ((Col::new(2), Row::new(1)), O.into()),
    ///     ((Col::new(2), Row::new(2)), X.into())
    ///   ]
    /// );
    pub fn taken_spaces(&self) -> impl Iterator<Item = (Position, Player)> + '_ {
        self.spaces()
            .filter_map(|(pos, player)| player.map(|p| (pos, p)))
    }

    /// Return the marker who's turn it is
    ///
    /// ```
    /// use lttcore::ttt;
    /// use lttcore::examples::tic_tac_toe::{TicTacToe, Marker::*};
    ///
    /// // Starts with X
    /// let game: TicTacToe = Default::default();
    /// assert_eq!(game.whose_turn(), X);

    /// // Once the first player goes, it's the second player's turn
    /// let game = ttt!([
    ///   - - -
    ///   - - -
    ///   X - -
    /// ]);
    /// assert_eq!(game.whose_turn(), O);

    /// // Once O goes, it's X's turn again
    /// let game = ttt!([
    ///   - - -
    ///   - - -
    ///   X O -
    /// ]);
    /// assert_eq!(game.whose_turn(), X);

    /// // The next player to go is always the one with the fewest spaces
    /// let game = ttt!([
    ///   O O O
    ///   O O O
    ///   - O O
    /// ]);
    /// assert_eq!(game.whose_turn(), X);
    /// ```
    pub fn whose_turn(&self) -> Player {
        let mut counts: HashMap<Player, usize> = HashMap::new();

        for (_, player) in self.taken_spaces() {
            *counts.entry(player).or_insert(0) += 1;
        }

        TWO_PLAYER
            .players()
            .min_by_key(|player| counts.get(player).copied().unwrap_or(0))
            .unwrap_or_else(NumberOfPlayers::starting_player)
    }

    /// Convenience method to construct a board from arrays of `Option<Marker>`, mostly used as the
    /// implementation of the `ttt!` macro
    /// ```
    /// // An empty board
    /// use lttcore::examples::tic_tac_toe::{TicTacToe, Col, Row, Marker::*};
    /// let game = TicTacToe::from_markers(
    ///   [
    ///     [None, None, None],
    ///     [None, None, None],
    ///     [None, None, None]
    ///   ]
    /// );
    ///
    /// assert_eq!(game, Default::default());
    ///
    /// // With some things on the board
    ///
    /// let game = TicTacToe::from_markers(
    ///   [
    ///     [Some(X), None, None],
    ///     [Some(O), Some(X), None],
    ///     [None, None, None]
    ///   ]
    /// );
    ///
    /// assert_eq!(
    ///   game.taken_spaces().collect::<Vec<_>>(),
    ///   vec![
    ///     ((Col::new(0), Row::new(0)), X.into()),
    ///     ((Col::new(1), Row::new(0)), O.into()),
    ///     ((Col::new(1), Row::new(1)), X.into())
    ///   ]
    /// )
    /// ```
    pub fn from_markers(board: [[Option<Marker>; 3]; 3]) -> Self {
        let board = board.map(|col| col.map(|opt_marker| opt_marker.map(|marker| marker.into())));

        Self {
            board,
            ..Default::default()
        }
    }

    /// is the board full?
    ///
    /// ```
    /// use lttcore::ttt;
    ///
    /// let game = ttt!([
    ///   X X X
    ///   X - X
    ///   X X X
    /// ]);
    ///
    /// assert!(game.has_open_spaces());
    ///
    /// let game = ttt!([
    ///   X X X
    ///   X X X
    ///   X X X
    /// ]);
    ///
    /// assert!(!game.has_open_spaces());
    ///
    /// ```
    pub fn has_open_spaces(&self) -> bool {
        self.taken_spaces().count() < 9
    }
}

impl Play for TicTacToe {
    type Action = Action;
    type ActionError = ActionError;
    type PublicInfo = PublicInfo;
    type PlayerSecretInfo = NoSecretPlayerInfo;
    type Settings = Settings;

    fn which_players_input_needed(&self, _settings: &Self::Settings) -> PlayerSet {
        match self.status() {
            Status::InProgress { next_up } => next_up.into(),
            _ => Default::default(),
        }
    }

    fn public_info(&self, _settings: &Self::Settings) -> Cow<'_, Self::PublicInfo> {
        Cow::Owned(PublicInfo::from_ttt(*self))
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

        let mut debug_msgs: DebugMsgs<Self> = Default::default();

        let public_info_update = {
            match response.as_ref() {
                Resign => self.resign(player),
                Timeout => self
                    .claim_next_available_space(player)
                    .expect("Tried to apply an action to a full board"),

                Response(Action { position }) => match self.claim_space(player, *position) {
                    Ok(update) => update,
                    Err(err) => {
                        debug_msgs.insert(player, err);
                        self.claim_next_available_space(player)
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
