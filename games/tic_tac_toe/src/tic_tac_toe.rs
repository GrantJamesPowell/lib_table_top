use crate::{Action, ActionError, Board, SpectatorView, SpectatorViewUpdate, Status};
use lttcore::{
    number_of_players::TWO_PLAYER,
    play::{DebugMsg, DebugMsgs, GameAdvance},
    ActionResponse, NumberOfPlayers, Play, Player, PlayerSet,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
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

impl DerefMut for TicTacToe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.board
    }
}

impl TicTacToe {
    /// Resigns a player, ending the game
    ///
    /// ```
    /// use lttcore::Play;
    /// use tic_tac_toe::{TicTacToe, Status::*, Marker::*, SpectatorViewUpdate::*};
    ///
    /// let settings = Default::default();
    /// let mut game: TicTacToe = Default::default();
    /// assert_eq!(game.spectator_view(&settings).status(), InProgress{ next_up: X.into() });
    /// assert_eq!(game.resign(X), Resign(X.into()));
    /// assert_eq!(game.spectator_view(&settings).status(), WinByResignation { winner: O.into() });
    /// ```
    pub fn resign(&mut self, player: impl Into<Player>) -> SpectatorViewUpdate {
        let player = player.into();
        self.resigned.add(player);
        SpectatorViewUpdate::Resign(player)
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
        use ActionResponse::*;

        let (player, response) = actions
            .next()
            .expect("Tic Tac Toe is single player at a time");

        let mut new_state = self.clone();
        let mut debug_msgs: DebugMsgs<Self> = Default::default();

        let spectator_update = {
            match response {
                Resign => new_state.resign(player),
                Response(attempted_action @ Action { position }) => {
                    match new_state.claim_space(player, position) {
                        Ok(update) => update,
                        Err(err) => {
                            let msg = DebugMsg {
                                attempted_action,
                                error: err,
                            };
                            debug_msgs.push((player, msg));
                            new_state
                                .claim_next_available_space(player)
                                .expect("Tried to apply an action to a full board")
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
