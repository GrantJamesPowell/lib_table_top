#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LeftOrRight {
    #[default]
    Left,
    Right,
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
