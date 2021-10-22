use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
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

#[derive(
    Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum Compass {
    #[default]
    North,
    East,
    South,
    West,
}

#[derive(
    Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum ArrowKey {
    #[default]
    Up,
    Down,
    Left,
    Right,
}
