use super::Area;
use crate::play::LttSettings;
use crate::utilities::number_of_players::TWO_PLAYER;
use crate::NumberOfPlayers;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::collections::HashMap;

use super::{BoardMarkers, Dimensions, Fish};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Settings {
    dimensions: Dimensions,
    out_of_player_squares: BoardMarkers,
    fish: SmallVec<[Fish; 8]>,
}

impl Settings {
    pub fn dimensions(&self) -> Dimensions {
        self.dimensions
    }

    pub fn board_area(&self) -> Area {
        Area {
            position: (0, 0),
            dimensions: self.dimensions,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        let dimensions = Dimensions {
            width: 16,
            height: 16,
        };

        Self {
            dimensions,
            fish: Default::default(),
            out_of_player_squares: dimensions.into(),
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
