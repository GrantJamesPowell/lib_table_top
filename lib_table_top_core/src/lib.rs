#![allow(dead_code)]
#![feature(never_type)]
#![feature(associated_type_defaults)]

use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use std::sync::Arc;

pub mod player;
pub use player::Player;

pub mod view;
pub use view::View;

pub mod play;
pub use play::Play;

struct GameRunner<T>
where
    T: Play,
{
    state: Arc<T>,
    seed: Arc<[u8; 32]>,
    initial_state: Option<Arc<T>>,
    settings: Arc<<T as Play>::Settings>,
    history: Vec<<T as Play>::Action>,
}

impl<T: Play> GameRunner<T> {
    fn advance_mut(&mut self, actions: &[(Player, <T as Play>::Action)]) -> ! {
        let new_state = Arc::make_mut(&mut self.state);

        let mut rng = ChaCha20Rng::from_seed(*self.seed);
        let stream_num = self.history.len().try_into().unwrap();
        rng.set_stream(stream_num);

        todo!()
    }
}
