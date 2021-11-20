use crate::utilities::PlayerIndexedData as PID;
use crate::Player;
use smallvec::SmallVec;
use std::borrow::Cow;

pub trait Score {
    fn score(&self) -> Cow<'_, Option<PID<u64>>> {
        Cow::Owned(None)
    }

    fn rank(&self) -> Cow<'_, Option<SmallVec<[SmallVec<[Player; 2]>; 4]>>> {
        Cow::Owned(None)
    }
}
