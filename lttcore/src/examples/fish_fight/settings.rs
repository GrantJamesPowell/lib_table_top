use crate::play::LttSettings;
use crate::utilities::number_of_players::TWO_PLAYER;
use crate::NumberOfPlayers;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::collections::HashMap;

use super::{Dimensions, Fish};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Settings {
    board_dimensions: Dimensions,
    fish: SmallVec<[Fish; 8]>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            board_dimensions: Dimensions {
                width: 16,
                height: 16,
            },
            fish: Default::default(),
        }
    }
}

impl LttSettings for Settings {
    fn number_of_players(&self) -> NumberOfPlayers {
        TWO_PLAYER
    }

    fn game_modes() -> &'static HashMap<&'static str, std::sync::Arc<Self>> {
        todo!()
    }
}
