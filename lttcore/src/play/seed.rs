//! Utilites around seeding things with a random state

use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};

/// 256 bits of sweet, sweet entropy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Seed(#[serde(with = "hex")] [u8; 32]);

/// Seed made of all 0s
pub const SEED_0: Seed = Seed([0; 32]);
/// Seed made of all
/// [42](https://en.wikipedia.org/wiki/42_(number)#The_Hitchhiker's_Guide_to_the_Galaxy)s
pub const SEED_42: Seed = Seed([42; 32]);

impl From<[u8; 32]> for Seed {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

impl Default for Seed {
    fn default() -> Self {
        Self::random()
    }
}

impl Seed {
    /// Alias for [`Seed::random`]
    pub fn new() -> Self {
        Self::random()
    }

    /// Create a random seed using the `rand::thread_rng` generator
    pub fn random() -> Self {
        rand::thread_rng().gen::<[u8; 32]>().into()
    }

    /// Build a [`ChaCha20Rng`] rng from the seed
    pub fn rng(&self) -> impl rand::Rng {
        ChaCha20Rng::from_seed(self.0)
    }

    /// Return a reference to the underlying bytes of the [`Seed`]
    pub fn bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Return the `Rng` stream used to init games
    ///
    /// # Implementation notes:
    ///
    /// We use `u64::MAX` as the stream
    pub fn rng_for_init(&self) -> impl rand::Rng {
        self.rng_for_turn(u64::MAX)
    }

    /// Return the `Rng` for a specific turn
    ///
    /// # Implementation notes:
    ///
    /// We use the stream of the [`TurnNum`](super::TurnNum) converted to a [`u64`]
    pub fn rng_for_turn(&self, turn: impl Into<u64>) -> impl rand::Rng {
        let mut rng = ChaCha20Rng::from_seed(self.0);
        rng.set_stream(turn.into());
        rng
    }
}
