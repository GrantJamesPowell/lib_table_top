use super::game_runner::GameRunner;
use lttcore::encoder::Encoder;
use lttcore::Play;

pub struct Runtime<T: Play, E: Encoder> {
    game_runner: GameRunner<T, E>,
}

impl<T: Play, E: Encoder> Runtime<T, E> {
    fn start() -> Self {
        Self {
            game_runner: GameRunner::new(),
        }
    }
}
