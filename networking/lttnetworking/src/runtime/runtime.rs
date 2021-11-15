use super::game_runner::GameRunner;
use lttcore::encoder::Encoder;
use lttcore::Play;
use std::sync::Arc;

pub struct Runtime<T: Play, E: Encoder> {
    game_runner: Arc<GameRunner<T, E>>,
}

impl<T: Play, E: Encoder> Runtime<T, E> {
    fn start() -> Self {
        let game_runner = Arc::new(GameRunner::new());

        Self { game_runner }
    }
}
