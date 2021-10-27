use crate::{Play, View};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct Spectator<T: Play> {
    pub turn_num: u64,
    pub settings: Arc<<T as Play>::Settings>,
    pub view: <T as Play>::SpectatorView,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct SpectatorUpdate<T: Play> {
    pub turn_num: u64,
    pub update: <<T as Play>::SpectatorView as View>::Update,
}

impl<T: Play> Spectator<T> {
    fn update(&mut self, update: SpectatorUpdate<T>) -> Result<(), Box<dyn Error>> {
        assert!(
            update.turn_num == (self.turn_num + 1),
            "tried to apply update for turn {:?} to a Spectator currently on turn {:?}",
            update.turn_num,
            self.turn_num,
        );

        self.view.update(&update.update)?;
        self.turn_num = update.turn_num;
        Ok(())
    }
}
