use crate::{Play, View};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct Spectator<T: Play> {
    turn_num: u64,
    settings: Arc<<T as Play>::Settings>,
    public_info: <T as Play>::PublicInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct SpectatorUpdate<T: Play> {
    pub turn_num: u64,
    pub public_info_update: <<T as Play>::PublicInfo as View>::Update,
}

impl<T: Play> Spectator<T> {
    fn update(&mut self, update: SpectatorUpdate<T>) -> Result<(), Box<dyn Error>> {
        self.public_info.update(&update.public_info_update)?;
        self.turn_num = update.turn_num;
        Ok(())
    }
}
