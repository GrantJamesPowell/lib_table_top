use futures_util::Stream;
use lttcore::play::{ActionResponse, GameAdvance};
use lttcore::utilities::PlayerItemCollector;
use lttcore::{GameProgression, Play};

pub trait GameHostRuntime {
    fn send_updates(&mut self, game_advance: GameAdvance);
}

struct GameHostConfig<T> {
    progression: GameProgression<T>,
}

pub async fn host_game<T, Runtime>(
    actions: impl Stream<Item = (Player, ActionResponse<T>)>,
    progression: GameProgression<T>,
) -> GameProgression<T>
where
    T: Play,
    Runtime: GameHostRuntime,
{
}
