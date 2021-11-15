use lttcore::id::UserId;
use lttcore::play::Mode;
use lttcore::Play;
use std::collections::HashSet;

pub enum ModeSelection<T: Play> {
    All,
    Specific(HashSet<Mode<T>>),
}

pub struct MatchMakerRequest<T: Play> {
    mode_selection: ModeSelection<T>,
    user_id: UserId,
}
