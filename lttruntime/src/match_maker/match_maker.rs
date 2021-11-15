use super::messages::MatchMakerRequestReceiver;
use crate::game_runner::GameRunner;
use lttcore::encoder::Encoder;
use lttcore::play::LttSettings;
use lttcore::{
    id::{GameId, UserId},
    play::Mode,
    Play, Player,
};
use smallvec::SmallVec;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub(crate) struct PendingRequest {
    user_id: UserId,
    sender: Sender<(GameId, Player)>,
}

type PendingRequests = SmallVec<[PendingRequest; 4]>;

pub struct MatchMakerQueues<T: Play> {
    queues: HashMap<Mode<T>, PendingRequests>,
}

impl<T: Play> MatchMakerQueues<T> {
    pub fn new() -> Self {
        let queues = <T::Settings as LttSettings>::game_modes()
            .keys()
            .map(|name| {
                let mode = Mode::try_new(name).expect("modes from `game_modes` are valid");
                (mode, Default::default())
            })
            .collect();

        MatchMakerQueues { queues }
    }
}

pub async fn run_match_maker<T: Play, E: Encoder>(
    _mailbox: MatchMakerRequestReceiver<T>,
    _game_runner: Arc<GameRunner<T, E>>,
    _queues: MatchMakerQueues<T>,
) {
    todo!()
}
