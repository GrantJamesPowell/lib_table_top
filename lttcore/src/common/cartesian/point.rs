use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

macro_rules! coord_component {
    ($id:ident) => {
        #[derive(
            Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize,
        )]
        pub struct $id<T = u32>(pub T);

        impl<T> From<T> for $id<T> {
            fn from(n: T) -> Self {
                Self(n)
            }
        }

        impl<T> Add<$id<T>> for $id<T>
        where
            T: Add<Output = T> + Copy,
        {
            type Output = $id<T>;

            fn add(self, rhs: $id<T>) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl<T> Add<T> for $id<T>
        where
            T: Add<Output = T> + Copy,
        {
            type Output = $id<T>;

            fn add(self, n: T) -> Self::Output {
                Self(self.0 + n)
            }
        }

        impl<T> Sub<$id<T>> for $id<T>
        where
            T: Sub<Output = T> + Copy,
        {
            type Output = $id<T>;

            fn sub(self, rhs: $id<T>) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }

        impl<T> Sub<T> for $id<T>
        where
            T: Sub<Output = T> + Copy,
        {
            type Output = $id<T>;

            fn sub(self, n: T) -> Self::Output {
                Self(self.0 - n)
            }
        }

        impl<T> $id<T>
        where
            T: Copy,
        {
            pub fn inner(&self) -> T {
                self.0
            }
        }
    };
}

coord_component!(X);
coord_component!(Y);

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point<T = u32> {
    pub x: X<T>,
    pub y: Y<T>,
}

impl<T> From<(T, T)> for Point<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x: X(x), y: Y(y) }
    }
}

impl<T> From<(X<T>, Y<T>)> for Point<T> {
    fn from((x, y): (X<T>, Y<T>)) -> Self {
        Self { x, y }
    }
}

impl<T> Point<T>
where
    T: Zero,
{
    pub fn origin() -> Self {
        Self {
            x: X(T::zero()),
            y: Y(T::zero()),
        }
    }
}

impl<T> Add<X<T>> for Point<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Point<T>;

    fn add(self, rhs: X<T>) -> Self::Output {
        Point {
            x: self.x + rhs,
            y: self.y,
        }
    }
}

impl<T> Add<Y<T>> for Point<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Point<T>;

    fn add(self, rhs: Y<T>) -> Self::Output {
        Point {
            x: self.x,
            y: self.y + rhs,
        }
    }
}

impl<T> Add<Point<T>> for Point<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Point<T>;

    fn add(self, rhs: Point<T>) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub<X<T>> for Point<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Point<T>;

    fn sub(self, rhs: X<T>) -> Self::Output {
        Point {
            x: self.x - rhs,
            y: self.y,
        }
    }
}

impl<T> Sub<Y<T>> for Point<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Point<T>;

    fn sub(self, rhs: Y<T>) -> Self::Output {
        Point {
            x: self.x,
            y: self.y - rhs,
        }
    }
}

impl<T> Sub<Point<T>> for Point<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Point<T>;

    fn sub(self, rhs: Point<T>) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
