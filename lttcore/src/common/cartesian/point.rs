use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

macro_rules! coord {
    ($id:ident) => {
        #[derive(
            Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize,
        )]
        pub struct $id(pub usize);

        impl From<usize> for $id {
            fn from(n: usize) -> Self {
                Self(n)
            }
        }

        impl From<$id> for usize {
            fn from(coord: $id) -> Self {
                coord.0
            }
        }

        impl Add<$id> for $id {
            type Output = $id;
            fn add(self, rhs: $id) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl Add<usize> for $id {
            type Output = $id;
            fn add(self, n: usize) -> Self::Output {
                Self(self.0 + n)
            }
        }

        impl Sub<$id> for $id {
            type Output = $id;

            fn sub(self, rhs: $id) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }

        impl Sub<usize> for $id {
            type Output = $id;

            fn sub(self, n: usize) -> Self::Output {
                Self(self.0 - n)
            }
        }
    };
}

coord!(X);
coord!(Y);

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point {
    pub x: X,
    pub y: Y,
}

pub const ORIGIN: Point = Point { x: X(0), y: Y(0) };

impl Add<X> for Point {
    type Output = Point;

    fn add(self, rhs: X) -> Self::Output {
        Point {
            x: self.x + rhs,
            y: self.y,
        }
    }
}

impl Add<Y> for Point {
    type Output = Point;

    fn add(self, rhs: Y) -> Self::Output {
        Point {
            x: self.x,
            y: self.y + rhs,
        }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<X> for Point {
    type Output = Point;

    fn sub(self, rhs: X) -> Self::Output {
        Point {
            x: self.x - rhs,
            y: self.y,
        }
    }
}

impl Sub<Y> for Point {
    type Output = Point;

    fn sub(self, rhs: Y) -> Self::Output {
        Point {
            x: self.x,
            y: self.y - rhs,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
