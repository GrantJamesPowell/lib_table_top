use crate::common::cartesian::{Area, Dimensions, Point};
use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Fish(Dimensions);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PositionedFish(pub Point, pub Fish);

impl PositionedFish {
    fn area(&self) -> Area {
        Area {
            origin: self.0,
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
            markers: BitVec::from_elem(dimensions.number_of_squares(), false),
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

    pub fn contains(&self, point: Point) -> bool {
        let offset = self.point_offset(point);
        self.markers.get(offset).unwrap_or(false)
    }

    pub fn insert(&mut self, point: Point) {
        self.set(point, true);
    }

    pub fn remove(&mut self, point: Point) {
        self.set(point, false);
    }

    pub fn any_in_area(&self, area: Area) -> bool {
        area.positions().any(|pos| self.contains(pos))
    }

    fn set(&mut self, point: Point, x: bool) {
        let offset = self.point_offset(point);
        self.markers.set(offset, x);
    }

    fn point_offset(&self, Point { x, y }: Point) -> usize {
        (usize::from(x)) + (usize::from(self.dimensions.width) * usize::from(y))
    }
}
