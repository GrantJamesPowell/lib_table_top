use super::messages::MatchMakerRequestReceiver;
use crate::runtime::game_runner::GameRunner;
use lttcore::encoder::Encoder;
use lttcore::{id::GameId, play::Mode, Play, Player};
use smallvec::SmallVec;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::oneshot::Sender;

pub async fn run_match_maker<T: Play, E: Encoder>(
    mailbox: MatchMakerRequestReceiver<T>,
    game_runner: Arc<GameRunner<T, E>>,
) {
    let queue: HashMap<Mode<T>, SmallVec<[Sender<(GameId, Player)>; 4]>> = Default::default();
    todo!()
}
