use crate::utilities::PlayerIndexedData as PID;
use crate::Player;
use smallvec::SmallVec;

pub trait Score {
    fn score(&self) -> Option<PID<u64>> {
        None
    }

    fn rank(&self) -> Option<SmallVec<[SmallVec<[Player; 2]>; 4]>> {
        None
    }
}
