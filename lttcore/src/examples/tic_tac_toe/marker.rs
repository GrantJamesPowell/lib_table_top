use serde::{Deserialize, Serialize};

use crate::Player;

/// Conveniences for Player 0 and Player 1
///
/// Markers implement `Into<Player>` and `PartialEq` with Player,
/// most methods and functions accept an `impl Into<Player>` so markers
/// can be used in their stead
/// ```
/// use lttcore::Player;
/// use lttcore::examples::tic_tac_toe::Marker::*;
///
/// let p0: Player = 0.into();
/// let p1: Player = 1.into();
///
/// assert_eq!(p0, X);
/// assert_eq!(p1, O);
/// ```
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Marker {
    X,
    O,
}

impl Marker {
    pub fn opponent(&self) -> Self {
        match self {
            Marker::X => Marker::O,
            Marker::O => Marker::X,
        }
    }
}

impl PartialEq<Marker> for Player {
    fn eq(&self, &other: &Marker) -> bool {
        let p: Player = other.into();
        *self == p
    }
}

impl PartialEq<Player> for Marker {
    fn eq(&self, &other: &Player) -> bool {
        let p: Player = (*self).into();
        other == p
    }
}

impl TryFrom<Player> for Marker {
    type Error = &'static str;

    fn try_from(player: Player) -> Result<Self, Self::Error> {
        match u8::from(player) {
            0 => Ok(Marker::X),
            1 => Ok(Marker::O),
            _ => Err("Only players 0 or 1 can play `TicTacToe`"),
        }
    }
}

impl From<Marker> for Player {
    fn from(marker: Marker) -> Player {
        match marker {
            Marker::X => 0.into(),
            Marker::O => 1.into(),
        }
    }
}
