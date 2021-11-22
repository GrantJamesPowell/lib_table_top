use itertools::Position;
use super::{Dimensions, Fish, Position, PositionedFish, Settings};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    Setup(FishPositions),
    Guess(Position),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionError {
    // GuessOutOfBounds { board_dimensions: Dimensions, guess: Position },
    // GuessAlreadyAttempted { guess: Position },
    // FishPlacedOutOfBounds { fish: PositionedFish, board_dimensions: board_dimensions },
    // OverlappingFish { fish_1: PositionedFish, fish_2: PositionedFish },
    // FishPlacedOnRemovedSquare { fish: PositionedFish, removed: Position },
    IncorrectNumberOfFish { number_sent: usize, number_expected: usize },
}

pub fn validate_placement(
    settings: &Settings,
    fish_to_place: &[Fish],
    placements: &[Position],
) -> Result<(), Vec<ActionError>> {
    let mut errors: Vec<ActionError> = Vec::new();

    if fish_positions.len() != placements.len() {
        errors.push(IncorrectNumberOfFish {
            number_sent: placements.len(),
            number_expected: fish_to_place.len()
        });
        return Err(errors);
    }

    fish_to_place
        .zip(placements.iter())
        .for_each(|fish, placement| {

        });

    Ok(())
}
