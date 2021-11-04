use async_trait::async_trait;
use lttcore::play::EnumeratedGameAdvance;
use lttcore::utilities::PlayerIndexedData as PID;
use lttcore::{GameObserver, GamePlayer, Play};
use lttnetworking::host_game::GameHostRuntime;
use smallvec::SmallVec;
use std::pin::Pin;
use tokio::sync::mpsc::Sender;

type Connections<Values> = SmallVec<[Sender<Values>; 8]>;

struct Runtime<T: Play> {
    players: PID<Connections<()>>,
    observers: PID<Connections<()>>,
    phantom: std::marker::PhantomData<T>,
}

#[async_trait]
impl<T: Play> GameHostRuntime<T> for Runtime<T> {
    fn send_game_player<'async_trait>(
        &'async_trait mut self,
        _: GamePlayer<T>,
    ) -> Pin<Box<(dyn futures_util::Future<Output = ()> + std::marker::Send + 'async_trait)>> {
        todo!()
    }

    fn send_observer<'async_trait>(
        &'async_trait mut self,
        _: GameObserver<T>,
    ) -> Pin<Box<(dyn futures_util::Future<Output = ()> + std::marker::Send + 'async_trait)>> {
        todo!()
    }
    fn send_updates<'async_trait>(
        &'async_trait mut self,
        _: EnumeratedGameAdvance<T>,
    ) -> Pin<Box<(dyn futures_util::Future<Output = ()> + std::marker::Send + 'async_trait)>> {
        todo!()
    }
}
