mod bounded;
pub use bounded::{BoundedCol, BoundedPosition, BoundedRow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
const ORIGIN: Point = Point { x: 0, y: 0 };

impl From<Point> for (usize, usize) {
    fn from(Point { x, y }: Point) -> Self {
        (x, y)
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

impl Dimensions {
    pub fn number_of_squares(&self) -> usize {
        self.width
            .checked_mul(self.height)
            .expect("width and height overflowed number_of_squares")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Area {
    pub origin: Point,
    pub dimensions: Dimensions,
}

impl Area {
    pub fn from_origin(dimensions: Dimensions) -> Self {
        Self {
            dimensions,
            origin: ORIGIN,
        }
    }

    pub fn positions(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.dimensions.width)
            .flat_map(|x| (0..self.dimensions.height).map(move |y| Point { x, y }))
    }

    pub fn number_of_squares(&self) -> usize {
        self.dimensions.number_of_squares()
    }

    pub fn top_right_point(&self) -> Point {
        let Point { x, y } = self.origin;
        Point {
            x: x + self.dimensions.width,
            y: y + self.dimensions.height,
        }
    }

    pub fn top_left_point(&self) -> Point {
        let Point { x, y } = self.origin;

        Point {
            x,
            y: y + self.dimensions.height,
        }
    }

    pub fn bottom_left_point(&self) -> Point {
        self.origin
    }

    pub fn bottom_right_point(&self) -> Point {
        let Point { x, y } = self.origin;

        Point {
            x: x + self.dimensions.width,
            y,
        }
    }

    pub fn top_y(&self) -> usize {
        self.bottom_y() + self.dimensions.height
    }

    pub fn bottom_y(&self) -> usize {
        let Point { x: _, y } = self.origin;
        y
    }

    pub fn left_x(&self) -> usize {
        let Point { x, y: _ } = self.origin;
        x
    }

    pub fn right_x(&self) -> usize {
        self.left_x() + self.dimensions.width
    }

    pub fn has_overlap_with(&self, other: Self) -> bool {
        self.overlaping_area(other).is_some()
    }

    pub fn contains_point(&self, Point { x, y }: Point) -> bool {
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
                    width: right_x - left_x,
                    height: top_y - bottom_y,
                },
            })
        } else {
            None
        }
    }

    pub fn encloses_area(&self, other: Self) -> bool {
        self.overlaping_area(other) == Some(other)
    }
}
