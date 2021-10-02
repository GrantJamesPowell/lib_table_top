use lib_table_top_core::{Play, Player, View};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Board([[Option<Player>; 3]; 3]);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Col(u8);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Row(u8);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position(Col, Row);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Action(Position);

pub type Settings = [Player; 2];

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum SettingsError {
    /// Returned when both players are the same
    #[error("Players must be different")]
    PlayersCantBeTheSame,
}

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum ActionError {
    /// Returned when trying to claim an already claimed space
    #[error("space ({:?}, {:?}) is taken", attempted.0, attempted.1)]
    SpaceIsTaken { attempted: Position },

    /// Returned when the wrong player tries to take a turn
    #[error("not {:?}'s turn", attempted)]
    OtherPlayerTurn { attempted: Player },

    /// Returned for an invalid position
    #[error("Invalid position, rows and cols must be in [0, 1, 2]")]
    InvalidPosition,
}

pub struct SpectatorView(Board);

impl View for SpectatorView {
    type Update = Action;

    fn update(&mut self, action: Self::Update) {}
}

#[derive(Clone)]
struct TicTacToe {
    board: Board,
}

impl Play for TicTacToe {
    type Action = Action;
    type ActionError = ActionError;

    type Settings = Settings;
    type SettingsError = SettingsError;

    type SpectatorView = SpectatorView;

    fn action_requests(&self, settings: &Self::Settings) -> Vec<Player> {
        // self.board
        //
        for col in self.board.0 {
            for row in col {}
        }

        todo!()
    }

    fn player_view(&self) -> <Self as Play>::PlayerView {
        Default::default()
    }

    fn spectator_view(&self) -> <Self as Play>::SpectatorView {
        SpectatorView(self.board)
    }

    fn initial_state_for_settings(_settings: &<Self as Play>::Settings) -> Self {
        TicTacToe {
            board: Board([[None; 3]; 3]),
        }
    }

    fn advance(
        &mut self,
        _settings: &Self::Settings,
        _actions: &[(Player, <Self as Play>::Action)],
        _rng: &mut impl rand::Rng,
    ) -> Result<<<Self as Play>::SpectatorView as View>::Update, <Self as Play>::ActionError> {
        todo!()
    }
}
