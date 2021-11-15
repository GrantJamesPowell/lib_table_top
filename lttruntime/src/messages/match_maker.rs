use lttcore::id::UserId;
use lttcore::play::Mode;
use lttcore::Play;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModeSelection<T: Play> {
    All,
    Specific(HashSet<Mode<T>>),
}

impl<T: Play> Default for ModeSelection<T> {
    fn default() -> Self {
        ModeSelection::All
    }
}

impl<T: Play> ModeSelection<T> {
    pub fn contains(&self, mode: &Mode<T>) -> bool {
        match self {
            ModeSelection::All => true,
            ModeSelection::Specific(set) => set.contains(mode)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchMakerRequest<T: Play> {
    mode_selection: ModeSelection<T>,
    user_id: UserId,
}
