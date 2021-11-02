use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Player(u8);

impl Player {
    pub const fn new(n: u8) -> Self {
        Self(n)
    }

    pub fn all() -> impl Iterator<Item = Player> {
        (0..=u8::MAX).map(Self::new)
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }

    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }

    pub fn as_u64(&self) -> u64 {
        self.0 as u64
    }
    /// Return the previous player, wrapping around from 0 => 255
    ///
    /// ```
    /// use lttcore::Player;
    ///
    /// let p0: Player = 0.into();
    /// let p1: Player = 1.into();
    /// let p2: Player = 2.into();
    /// let p255: Player = 255.into();
    ///
    /// assert_eq!(p0.previous(), p255);
    /// assert_eq!(p1.previous(), p0);
    /// assert_eq!(p2.previous(), p1);
    /// ```
    pub fn previous(&self) -> Self {
        Self(self.0.wrapping_sub(1))
    }

    /// Return the next player, wrapping around from 255 => 0
    ///
    /// ```
    /// use lttcore::Player;
    ///
    /// let p0: Player = 0.into();
    /// let p1: Player = 1.into();
    /// let p2: Player = 2.into();
    /// let p255: Player = 255.into();
    ///
    /// assert_eq!(p0.next(), p1);
    /// assert_eq!(p1.next(), p2);
    /// assert_eq!(p255.next(), p0);
    /// ```
    pub fn next(&self) -> Self {
        Self(self.0.wrapping_add(1))
    }
}

impl From<u8> for Player {
    fn from(n: u8) -> Self {
        Self(n)
    }
}
