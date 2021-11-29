use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Seed(#[serde(with = "hex")] [u8; 32]);

pub const SEED_0: Seed = Seed([0; 32]);
pub const SEED_42: Seed = Seed([42; 32]);

impl From<[u8; 32]> for Seed {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

impl Seed {
    /// Create a random seed using the `rand::thread_rng` generator
    pub fn random() -> Self {
        rand::thread_rng().gen::<[u8; 32]>().into()
    }

    pub fn rng(&self) -> impl rand::Rng {
        ChaCha20Rng::from_seed(self.0)
    }

    pub fn bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn rng_for_init(&self) -> impl rand::Rng {
        self.rng_for_turn(u64::MAX)
    }

    pub fn rng_for_turn(&self, turn: impl Into<u64>) -> impl rand::Rng {
        let mut rng = ChaCha20Rng::from_seed(self.0);
        rng.set_stream(turn.into());
        rng
    }
}
