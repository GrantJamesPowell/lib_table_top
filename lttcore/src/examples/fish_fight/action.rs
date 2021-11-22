use super::{Dimensions, Fish, FishPositions, PositionedFish, Settings};
use crate::common::cartesian::Point;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    Setup(FishPositions),
    Guess(Point),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionError {
    // GuessOutOfBounds { board_dimensions: Dimensions, guess: Position },
    // GuessAlreadyAttempted { guess: Position },
    // FishPlacedOutOfBounds { fish: PositionedFish, board_dimensions: board_dimensions },
    // OverlappingFish { fish_1: PositionedFish, fish_2: PositionedFish },
    // FishPlacedOnRemovedSquare { fish: PositionedFish, removed: Position },
    IncorrectNumberOfFish {
        number_sent: usize,
        number_expected: usize,
    },
}

pub fn validate_placement(
    settings: &Settings,
    fish_to_place: &[Fish],
    placements: &[Point],
) -> Result<(), Vec<ActionError>> {
    let mut errors: Vec<ActionError> = Vec::new();

    if fish_to_place.len() != placements.len() {
        errors.push(ActionError::IncorrectNumberOfFish {
            number_sent: placements.len(),
            number_expected: fish_to_place.len(),
        });
        return Err(errors);
    }

    Ok(())
}
