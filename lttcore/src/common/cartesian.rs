pub mod bounded;
mod point;
mod point_set;

pub use point::{Point, X, Y};
pub use point_set::PointSet;

use num_traits::{CheckedMul, Zero};
use serde::{Deserialize, Serialize};
use std::cmp::Ord;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Dimensions<T = u32> {
    pub width: T,
    pub height: T,
}

impl<T> Dimensions<T>
where
    T: CheckedMul,
{
    pub fn number_of_squares(&self) -> T {
        self.width
            .checked_mul(&self.height)
            .expect("width and height multiplication overflowed")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Area<T = u32> {
    pub origin: Point<T>,
    pub dimensions: Dimensions<T>,
}

impl<T: Zero> Area<T> {
    pub fn from_origin(dimensions: Dimensions<T>) -> Self {
        Self {
            dimensions,
            origin: Point::origin(),
        }
    }
}

impl<T: Copy + CheckedMul> Area<T> {
    pub fn number_of_squares(&self) -> T {
        self.dimensions.number_of_squares()
    }
}

impl<T: Add<T, Output = T> + Copy> Area<T> {
    pub fn top_right_point(&self) -> Point<T> {
        let Point { x, y } = self.origin;
        Point {
            x: x + self.dimensions.width,
            y: y + self.dimensions.height,
        }
    }

    pub fn top_left_point(&self) -> Point<T> {
        let Point { x, y } = self.origin;

        Point {
            x,
            y: y + self.dimensions.height,
        }
    }

    pub fn bottom_left_point(&self) -> Point<T> {
        self.origin
    }

    pub fn bottom_right_point(&self) -> Point<T> {
        let Point { x, y } = self.origin;

        Point {
            x: x + self.dimensions.width,
            y,
        }
    }

    pub fn top_y(&self) -> Y<T> {
        self.origin.y + self.dimensions.height
    }

    pub fn bottom_y(&self) -> Y<T> {
        self.origin.y
    }

    pub fn left_x(&self) -> X<T> {
        self.origin.x
    }

    pub fn right_x(&self) -> X<T> {
        self.origin.x + self.dimensions.width
    }
}

impl<T: Add<T, Output = T> + Sub<T, Output = T> + Ord + Copy> Area<T> {
    pub fn contains_point(&self, Point { x, y }: Point<T>) -> bool {
        self.left_x() <= x && x <= self.right_x() && self.bottom_y() <= y && y <= self.top_y()
    }

    pub fn overlaping_area(&self, other: Self) -> Option<Self> {
        let top_y = self.top_y().min(other.top_y());
        let bottom_y = self.top_y().max(other.top_y());
        let left_x = self.left_x().max(other.left_x());
        let right_x = self.right_x().min(other.right_x());

        if top_y >= bottom_y && right_x >= left_x {
            Some(Area {
                origin: Point {
                    x: left_x,
                    y: bottom_y,
                },
                dimensions: Dimensions {
                    width: (right_x - left_x).inner(),
                    height: (top_y - bottom_y).inner(),
                },
            })
        } else {
            None
        }
    }

    pub fn has_overlap_with(&self, other: Self) -> bool {
        self.overlaping_area(other).is_some()
    }

    pub fn encloses_area(&self, other: Self) -> bool {
        self.overlaping_area(other) == Some(other)
    }
}
