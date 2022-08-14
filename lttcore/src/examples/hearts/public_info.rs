use crate::utilities::PlayerIndexedData as PID;
use serde::{Deserialize, Serialize};

use crate::play::{Score, View};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicInfoUpdate;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicInfo {}

impl View for PublicInfo {
    type Update = PublicInfoUpdate;
}

impl Score for PublicInfo {
    fn score(&self) -> Option<PID<i64>> {
        todo!()
    }
}
