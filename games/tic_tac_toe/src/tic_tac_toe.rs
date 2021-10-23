use crate::{Action, ActionError, Board, SpectatorView, Status};
use lttcore::{
    number_of_players::TWO_PLAYER,
    play::{ActionResponse, DebugMsg, DebugMsgs, GameAdvance},
    NumberOfPlayers, Play, Player, PlayerSet,
};
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct TicTacToe {
    board: Board,
    resigned: PlayerSet,
}

impl From<Board> for TicTacToe {
    fn from(board: Board) -> Self {
        Self {
            board,
            ..Default::default()
        }
    }
}

impl Deref for TicTacToe {
    type Target = Board;

    fn deref(&self) -> &Self::Target {
        &self.board
    }
}

impl TicTacToe {
    /// Resigns a player, ending the game
    ///
    /// ```
    /// use lttcore::Play;
    /// use tic_tac_toe::{TicTacToe, Status::*, Marker::*};
    ///
    /// let settings = Default::default();
    /// let mut game: TicTacToe = Default::default();
    /// assert_eq!(game.spectator_view(&settings).status(), InProgress{ next_up: X.into() });
    /// game.resign(X); // or game.resign(0.into());
    /// assert_eq!(game.spectator_view(&settings).status(), WinByResignation { winner: O.into() });
    /// ```
    pub fn resign(&mut self, player: impl Into<Player>) {
        self.resigned.add(player.into());
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn resigned(&self) -> &PlayerSet {
        &self.resigned
    }
}

impl Play for TicTacToe {
    type Action = Action;
    type ActionError = ActionError;
    type SpectatorView = SpectatorView;
    type Status = Status;

    fn action_requests(&self, settings: &Self::Settings) -> PlayerSet {
        match self.spectator_view(settings).status() {
            Status::InProgress { next_up } => next_up.into(),
            _ => Default::default(),
        }
    }

    fn spectator_view(&self, _settings: &Self::Settings) -> Self::SpectatorView {
        SpectatorView::from_board_and_resigned(self.board, self.resigned)
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

    fn player_views(
        &self,
        _settings: &<Self as Play>::Settings,
    ) -> HashMap<Player, Self::PlayerView> {
        TWO_PLAYER
            .players()
            .map(|player| (player, Default::default()))
            .collect()
    }

    fn advance(
        &self,
        _settings: &Self::Settings,
        mut actions: impl Iterator<Item = (Player, ActionResponse<<Self as Play>::Action>)>,
        _rng: &mut impl rand::Rng,
    ) -> (Self, GameAdvance<Self>) {
        use crate::spectator_view::Update;
        use ActionResponse::*;

        let (player, response) = actions
            .next()
            .expect("Tic Tac Toe is single player at a time");

        let mut new_state = self.clone();
        let mut debug_msgs: DebugMsgs<Self> = Default::default();

        let spectator_update = {
            match response {
                Resign => {
                    new_state.resign(player);
                    Update::Resign(player)
                }
                Response(attempted_action @ Action { position }) => {
                    match new_state.board.claim_space(player, position) {
                        Ok(_) => Update::Claim(player, position),
                        Err(err) => {
                            let replacement = new_state.board.empty_spaces().next().unwrap();

                            new_state.board.claim_space(player, replacement).unwrap();

                            debug_msgs.push((
                                player,
                                DebugMsg {
                                    attempted_action,
                                    replaced_action: Action {
                                        position: replacement,
                                    },
                                    error: err,
                                },
                            ));

                            Update::Claim(player, position)
                        }
                    }
                }
            }
        };

        (
            new_state,
            GameAdvance {
                debug_msgs,
                spectator_update,
                player_updates: Default::default(),
            },
        )
    }
}
