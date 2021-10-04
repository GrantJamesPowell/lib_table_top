#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Marker {
    X,
    O,
}

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
        use Marker::*;

        match self {
            X => O,
            O => X,
        }
    }
}
