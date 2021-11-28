use super::Point;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct PointSet<T: Hash + Eq = u32> {
    points: HashSet<Point<T>>,
}

impl<T: Hash + Eq> PointSet<T> {
    pub fn insert(&mut self, point: Point<T>) {
        self.points.insert(point);
    }

    pub fn contains(&self, point: &Point<T>) -> bool {
        self.points.contains(point)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }
}
