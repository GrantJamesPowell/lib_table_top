use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Seed([u8; 32]);

impl From<[u8; 32]> for Seed {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

impl Seed {
    pub fn rng_for_init(&self) -> impl rand::Rng {
        self.rng_for_turn(u64::MAX)
    }

    pub fn rng_for_turn(&self, turn: u64) -> impl rand::Rng {
        let mut rng = ChaCha20Rng::from_seed(self.0);
        rng.set_stream(turn);
        rng
    }
}
