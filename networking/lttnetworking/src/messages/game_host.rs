use lttcore::play::ActionResponse;
use lttcore::{Play, Player};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToGameHostMsg<T: Play> {
    RequestObserverState,
    RequestPlayerState {
        player: Player,
    },
    SubmitActionResponse {
        player: Player,
        response: ActionResponse<T>,
    },
}
