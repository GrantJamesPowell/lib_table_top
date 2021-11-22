use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BitVec;

impl BitVec {
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn count_ones(&self) -> u64 {
        todo!()
    }

    pub fn set(&mut self, _val: bool) {
        todo!()
    }

    pub fn get(&self, _i: usize) -> bool {
        todo!()
    }

    pub fn intersection(&self, _other: Self) -> Self {
        todo!()
    }
}

pub type Position = (u8, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Dimensions {
    pub width: u8,
    pub height: u8,
}

impl Dimensions {
    fn number_of_squares(&self) -> u16 {
        (self.width as u16) * (self.height as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Area {
    pub position: Position,
    pub dimensions: Dimensions,
}

impl Area {
    pub fn intersects_with(&self, other: impl Into<BitVec>) -> bool {
        let other = other.into();
        !self.bit_vec().intersection(other).is_empty()
    }

    pub fn bit_vec(&self) -> BitVec {
        todo!()
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
            markers: Default::default(),
        }
    }
}

impl BoardMarkers {
    pub fn count(&self) -> usize {
        todo!()
    }

    pub fn dimensions(&self) -> Dimensions {
        self.dimensions
    }

    pub fn contains(&self, position: Position) -> bool {
        todo!()
    }

    pub fn insert(&mut self, position: Position) {
        todo!()
    }

    pub fn remove(&mut self, position: Position) {
        todo!()
    }

    pub fn intersects(&self, other: &Self) -> bool {
        todo!()
    }
}
