use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Seed([u8; 32]);

impl From<[u8; 32]> for Seed {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

pub fn for_init(seed: Seed) -> impl rand::Rng {
    let mut rng = ChaCha20Rng::from_seed(seed.0);
    rng.set_stream(u64::MAX);
    rng
}
