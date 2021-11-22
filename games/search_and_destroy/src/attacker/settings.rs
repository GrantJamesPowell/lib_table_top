use lttcore::{
    common::cartesian::{Area, Dimensions},
    play::LttSettings,
    utilities::number_of_players::ONE_PLAYER,
    NumberOfPlayers,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Settings {
    board_dimensions: Dimensions,
    target_dimensions: Vec<Dimensions>,
    removed_areas: Vec<Area>,
}

impl Settings {
    pub fn board_dimensions(&self) -> Dimensions {
        self.board_dimensions
    }

    pub fn removed_areas(&self) -> &[Area] {
        &self.removed_areas
    }

    pub fn target_dimensions(&self) -> &[Dimensions] {
        &self.target_dimensions
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            board_dimensions: Dimensions {
                width: 20,
                height: 20,
            },
            removed_areas: Vec::new(),
            target_dimensions: Vec::new(),
        }
    }
}

impl LttSettings for Settings {
    fn number_of_players(&self) -> NumberOfPlayers {
        ONE_PLAYER
    }

    fn game_modes() -> &'static HashMap<&'static str, Arc<Self>> {
        todo!()
    }
}
