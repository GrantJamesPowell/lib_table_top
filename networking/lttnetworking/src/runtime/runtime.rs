use super::async_game_runner::AsyncGameRunner;
use lttcore::encoder::Encoder;
use lttcore::Play;

pub struct Runtime<T: Play, E: Encoder> {
    game_runner: AsyncGameRunner<T, E>,
}

impl<T: Play, E: Encoder> Runtime<T, E> {
    fn start() -> Self {
        Self {
            game_runner: AsyncGameRunner::new(),
        }
    }
}
