#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Marker {
    X,
    O,
}

use Marker::*;

impl Marker {
    /// Returns the opponent of a Marker
    ///
    /// ```
    /// use tic_tac_toe::Marker::*;
    ///
    /// assert_eq!(X.opponent(), O);
    /// assert_eq!(O.opponent(), X);
    /// ```
    pub fn opponent(&self) -> Self {
        match self {
            X => O,
            O => X,
        }
    }
}

use std::fmt;

impl fmt::Display for Marker {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                X => "X",
                O => "O",
            }
        )
    }
}
