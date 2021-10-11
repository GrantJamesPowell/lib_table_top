#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Player(u8);

impl Player {
    pub fn new(n: u8) -> Self {
        n.into()
    }
}

impl From<u8> for Player {
    fn from(n: u8) -> Self {
        Self(n)
    }
}
