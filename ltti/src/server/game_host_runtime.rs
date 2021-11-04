use async_trait::async_trait;
use lttcore::play::EnumeratedGameAdvance;
use lttcore::utilities::PlayerIndexedData as PID;
use lttcore::{GameObserver, GamePlayer, Play};
use lttnetworking::host_game::GameHostRuntime;
use smallvec::SmallVec;
use tokio::sync::mpsc::Sender;
use std::time::Instant;

type Connections<Values> = SmallVec<[Sender<Values>; 8]>;

struct Runtime<T: Play> {
    players: PID<Connections<()>>,
    observers: PID<Connections<()>>,
    phantom: std::marker::PhantomData<T>,
}

#[async_trait(?Send)]
impl<T: Play> GameHostRuntime<T> for Runtime<T> {
    async fn send_game_player(&mut self, _game_player: GamePlayer<T>) {}
    async fn send_observer(&mut self, _observer: GameObserver<T>) {}
    async fn send_updates(&mut self, _game_advance: EnumeratedGameAdvance<T>) {}
    async fn tick(&mut self, _time: Instant) {}
}
