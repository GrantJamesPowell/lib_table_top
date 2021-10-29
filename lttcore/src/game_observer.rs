use crate::{Observe, Observer, Play, PlayerSet};

pub struct GameObserver<T: Play> {
    turn_num: u64,
    action_requests: PlayerSet,
    settings: <T as Play>::Settings,
    public_info: <T as Play>::PublicInfo,
}

impl<T: Play> Observe<T> for GameObserver<T> {
    fn observe(&self) -> Observer<'_, T> {
        Observer {
            turn_num: self.turn_num,
            action_requests: self.action_requests,
            settings: &self.settings,
            public_info: &self.public_info,
        }
    }
}
