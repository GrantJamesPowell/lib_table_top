use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BitVec;

impl BitVec {
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn num_true(&self) -> u16 {
        todo!()
    }

    pub fn num_false(&self) -> u16 {
        todo!()
    }

    pub fn set(&mut self, _val: bool) {
        todo!()
    }

    pub fn get(&self, _i: usize) -> bool {
        todo!()
    }

    pub fn intersection(&self, other: Self) -> Self {
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
            position: self.0.position,
            dimensions: self.1 .0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Board {
    pub misses: BitVec,
    pub hits: BitVec,
}
