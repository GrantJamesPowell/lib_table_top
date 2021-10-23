use lttcore::Player;

/// Conveniences for Player 0 and Player 1
///
/// Markers implement `Into<Player>` and `PartialEq` with Player,
/// most methods and functions accept an `impl Into<Player>` so markers
/// can be used in their stead
/// ```
/// use lttcore::Player;
/// use tic_tac_toe::Marker::*;
///
/// let p0: Player = 0.into();
/// let p1: Player = 1.into();
///
/// assert_eq!(p0, X);
/// assert_eq!(p1, O);
/// ```
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Marker {
    X,
    O,
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

impl Into<Player> for Marker {
    fn into(self) -> Player {
        match self {
            Marker::X => 0.into(),
            Marker::O => 1.into(),
        }
    }
}
