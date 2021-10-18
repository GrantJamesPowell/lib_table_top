#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LeftOrRight {
    #[default]
    Left,
    Right,
}

use LeftOrRight::*;

impl LeftOrRight {
    /// Reverse the direction
    ///
    /// ```
    /// use lttcore::common::direction::LeftOrRight::*;
    ///
    /// assert_eq!(Left.reverse(), Right);
    /// assert_eq!(Right.reverse(), Left);
    /// ```
    pub fn reverse(&self) -> Self {
        match self {
            Left => Right,
            Right => Left,
        }
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Compass {
    #[default]
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArrowKey {
    #[default]
    Up,
    Down,
    Left,
    Right,
}
