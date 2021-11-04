use crate::{
    helpers::opponent,
    Action,
    ActionError::{self, *},
    Col, Position, PublicInfo, PublicInfoUpdate, Row, POSSIBLE_WINS,
};
use lttcore::{
    play::{ActionResponse, DebugMsgs, GameAdvance, PlayerSecretInfos},
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

use Status::*;

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
    /// use tic_tac_toe::{TicTacToe, Status::*, Marker::*, PublicInfoUpdate::*};
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
    /// use tic_tac_toe::{ttt, TicTacToe, Row, Col, Status::*, Marker::*};
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
                winner: opponent(loser),
            };
        }

        POSSIBLE_WINS
            .iter()
            .filter_map(|&positions| {
                let [a, b, c] = positions.map(|pos| self.at_position(pos));

                if a == b && b == c {
                    a.map(|winner| Win { winner, positions })
                } else {
                    None
                }
            })
            .next()
            .unwrap_or_else(|| {
                if !self.has_open_spaces() {
                    Draw
                } else {
                    InProgress {
                        next_up: self.whose_turn(),
                    }
                }
            })
    }

    /// Claims a space for a marker, returns an error if that space is taken
    ///
    /// ```
    /// use tic_tac_toe::{TicTacToe, Marker::*, Col, Row, ActionError::*};
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
            return Err(SpaceIsTaken {
                attempted: position,
            });
        }

        let (c, r) = position;
        self.board[c.as_usize()][r.as_usize()] = Some(player);
        Ok(PublicInfoUpdate::Claim(player, position))
    }

    /// Claims the next available space on the board.
    /// Designed to be deterministic to be used for defaulting moves
    ///
    /// ```
    /// use tic_tac_toe::{ttt, Marker::*, PublicInfoUpdate::*, Col, Row};
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
        let position = self.empty_spaces().next().ok_or(AllSpacesTaken)?;
        self.claim_space(player, position)
    }

    /// Returns the marker at a position, since this requires [`Row`] and [`Col`] structs
    /// the indexing will always be inbound
    ///
    /// ```
    /// use tic_tac_toe::{ttt, Row, Col, Marker::*};
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
        self.board[c.as_usize()][r.as_usize()]
    }

    /// Returns a marker at a position, if the row or col is greater than 2, this returns None
    ///
    /// ```
    /// use tic_tac_toe::{ttt, Row, Col, Marker::*};
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
        let col = Col::try_new(c.try_into().ok()?)?;
        let row = Row::try_new(r.try_into().ok()?)?;

        self.at_position((col, row))
    }

    /// Iterator over the empty spaces on the board
    ///
    /// ```
    /// use tic_tac_toe::{ttt, TicTacToe, Row, Col, Marker::*, Position};
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
    /// use tic_tac_toe::{ttt, Row, Col, Marker::*, Position};
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
            col.iter().enumerate().map(move |(row_num, &player)| {
                (
                    (
                        Col::new(col_num.try_into().unwrap()),
                        Row::new(row_num.try_into().unwrap()),
                    ),
                    player,
                )
            })
        })
    }

    /// Iterate over the spaces on the board that are taken
    ///
    /// ```
    /// use tic_tac_toe::{ttt, Row, Col, Marker::*};
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
    /// use tic_tac_toe::{ttt, TicTacToe, Marker::*};
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
            .min_by_key(|player| counts.get(player).cloned().unwrap_or(0))
            .unwrap_or(NumberOfPlayers::starting_player())
    }

    /// Convenience method to construct a board from arrays of ints, mostly used as the
    /// implementation of the `ttt!` macro
    /// 0 => None
    /// 1 => Some(X | Player::new(0))
    /// 2 => Some(O | Player::new(1))
    ///
    /// ```
    /// // An empty board
    /// use tic_tac_toe::{TicTacToe, Col, Row, Marker::*};
    /// let game = TicTacToe::from_ints(
    ///   [
    ///     [0, 0, 0],
    ///     [0, 0, 0],
    ///     [0, 0, 0]
    ///   ]
    /// );
    ///
    /// assert_eq!(game, Default::default());
    ///
    /// // With some things on the board
    ///
    /// let game = TicTacToe::from_ints(
    ///   [
    ///     [1, 0, 0],
    ///     [2, 1, 0],
    ///     [0, 0, 0]
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
    ///
    /// # Panics
    ///
    /// Will panic if the number is outside of 0..=2
    ///
    /// ```should_panic
    /// use tic_tac_toe::TicTacToe;
    ///
    /// TicTacToe::from_ints(
    ///   [
    ///     [0, 0, 0],
    ///     [0, 3, 0],
    ///     [0, 0, 0]
    ///   ]
    /// );
    /// ```
    pub fn from_ints(board: [[u16; 3]; 3]) -> Self {
        let board = board.map(|col| {
            col.map(|n| match n {
                0 => None,
                1 => Some(0.into()),
                2 => Some(1.into()),
                _ => panic!("Invalid number, must ints must be within 0..=2"),
            })
        });

        Self {
            board,
            ..Default::default()
        }
    }

    /// is the board full?
    ///
    /// ```
    /// use tic_tac_toe::ttt;
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

    fn which_players_input_needed(&self, _settings: &Self::Settings) -> PlayerSet {
        match self.status() {
            Status::InProgress { next_up } => next_up.into(),
            _ => Default::default(),
        }
    }

    fn public_info(&self, _settings: &Self::Settings) -> Self::PublicInfo {
        PublicInfo::from_ttt(self.clone())
    }

    fn initial_state_for_settings(
        _settings: &<Self as Play>::Settings,
        _rng: &mut impl rand::Rng,
    ) -> Self {
        Default::default()
    }

    fn number_of_players_for_settings(_settings: &<Self as Play>::Settings) -> NumberOfPlayers {
        TWO_PLAYER
    }

    fn player_secret_info(&self, _settings: &<Self as Play>::Settings) -> PlayerSecretInfos<Self> {
        TWO_PLAYER
            .players()
            .map(|player| (player, Default::default()))
            .collect()
    }

    fn advance<'a>(
        &'a mut self,
        settings: &Self::Settings,
        mut actions: impl Iterator<Item = (Player, Cow<'a, ActionResponse<<Self as Play>::Action>>)>,
        _rng: &mut impl rand::Rng,
    ) -> GameAdvance<Self> {
        use ActionResponse::*;

        let (player, response) = actions
            .next()
            .expect("Tic Tac Toe is single player at a time");

        let mut debug_msgs: DebugMsgs<Self> = Default::default();

        let public_info_update = {
            match response.as_ref() {
                Resign => self.resign(player),
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
