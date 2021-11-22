use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

pub type Position = (u8, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Dimensions {
    pub width: u8,
    pub height: u8,
}

impl Dimensions {
    pub fn number_of_squares(&self) -> u16 {
        (self.width as u16) * (self.height as u16)
    }

    pub fn contains(&self, (x, y): Position) -> bool {
        x <= self.width && y <= self.height
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Area {
    pub position: Position,
    pub dimensions: Dimensions,
}

impl Area {
    pub fn covered_positions(&self) -> impl Iterator<Item = Position> + '_ {
        (0..self.dimensions.width).flat_map(|x| (0..self.dimensions.height).map(move |y| (x, y)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Fish(Dimensions);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PositionedFish(pub Position, pub Fish);

impl PositionedFish {
    fn area(&self) -> Area {
        Area {
            position: self.0,
            dimensions: self.1 .0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerBoards {
    pub hits: BoardMarkers,
    pub misses: BoardMarkers,
}

impl From<Dimensions> for PlayerBoards {
    fn from(dimensions: Dimensions) -> Self {
        Self {
            hits: dimensions.into(),
            misses: dimensions.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BoardMarkers {
    dimensions: Dimensions,
    markers: BitVec,
}

impl From<Dimensions> for BoardMarkers {
    fn from(dimensions: Dimensions) -> Self {
        Self {
            dimensions,
            markers: BitVec::from_elem(dimensions.number_of_squares() as usize, false),
        }
    }
}

impl BoardMarkers {
    /// Number of spaces marked as `True`
    pub fn count(&self) -> usize {
        self.markers
            .blocks()
            .map(|block| block.count_ones())
            .sum::<u32>()
            .try_into()
            .unwrap()
    }

    pub fn dimensions(&self) -> Dimensions {
        self.dimensions
    }

    pub fn contains(&self, position: Position) -> bool {
        let offset = self.position_offset(position);
        self.markers.get(offset).unwrap_or(false)
    }

    pub fn insert(&mut self, position: Position)  {
        self.set(position, true);
    }

    pub fn remove(&mut self, position: Position) {
        self.set(position, false);
    }

    pub fn any_in_area(&self, area: Area) -> bool {
        area.covered_positions().any(|pos| self.contains(pos))
    }

    fn set(&mut self, position: Position, x: bool) {
        let offset = self.position_offset(position);
        self.markers.set(offset, x);
    }

    fn position_offset(&self, (x, y): Position) -> usize {
        (x as usize) + ((self.dimensions.width as usize) * (y as usize))
    }
}
