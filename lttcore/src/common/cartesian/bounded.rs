use super::{Point, X, Y};
use paste::paste;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::fmt;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoundsError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BoundedPoint<const WIDTH: usize, const HEIGHT: usize> {
    x: BoundedX<WIDTH>,
    y: BoundedY<HEIGHT>,
}

impl<const WIDTH: usize, const HEIGHT: usize> TryFrom<Point<usize>>
    for BoundedPoint<WIDTH, HEIGHT>
{
    type Error = BoundsError;

    fn try_from(Point { x, y }: Point<usize>) -> Result<Self, Self::Error> {
        let x: BoundedX<WIDTH> = BoundedX::try_from(x)?;
        let y: BoundedY<HEIGHT> = BoundedY::try_from(y)?;

        Ok(Self { x, y })
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> TryFrom<(usize, usize)>
    for BoundedPoint<WIDTH, HEIGHT>
{
    type Error = BoundsError;

    fn try_from((x, y): (usize, usize)) -> Result<Self, Self::Error> {
        let point = Point::<usize>::from((x, y));
        Self::try_from(point)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> From<(BoundedX<WIDTH>, BoundedY<HEIGHT>)>
    for BoundedPoint<WIDTH, HEIGHT>
{
    fn from((x, y): (BoundedX<WIDTH>, BoundedY<HEIGHT>)) -> Self {
        Self { x, y }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> BoundedPoint<WIDTH, HEIGHT> {
    pub fn x(&self) -> BoundedX<WIDTH> {
        self.x
    }

    pub fn y(&self) -> BoundedY<HEIGHT> {
        self.y
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> fmt::Display for BoundedPoint<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

macro_rules! bounded_coord_component {
    ($id:ident, $counterpart:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
        pub struct $id<const BOUND: usize>(usize);

        impl<const BOUND: usize> fmt::Display for $id<BOUND> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
                write!(f, "{}", self.0)
            }
        }

        impl<const BOUND: usize> From<$id<BOUND>> for usize {
            fn from(bounded: $id<BOUND>) -> Self {
                bounded.0
            }
        }

        impl<const BOUND: usize> TryFrom<$counterpart<usize>> for $id<BOUND> {
            type Error = BoundsError;

            fn try_from(non_bounded: $counterpart<usize>) -> Result<Self, Self::Error> {
                if non_bounded.0 <= BOUND {
                    Ok(Self(non_bounded.0))
                } else {
                    Err(BoundsError {})
                }
            }
        }

        impl<const BOUND: usize> From<$id<BOUND>> for $counterpart<usize> {
            fn from(bounded: $id<BOUND>) -> Self {
                Self(bounded.0)
            }
        }

        impl<const BOUND: usize> $id<BOUND> {
            #[doc = "Create a new `"]
            #[doc = stringify!($id)]
            #[doc = "`, panicking if `n` is larger than `BOUND`"]
            pub fn new(n: usize) -> Self {
                if n > BOUND {
                    panic!(
                        "can't create {} because {} is larger than {}",
                        std::any::type_name::<$id<BOUND>>(),
                        n,
                        BOUND
                    );
                } else {
                    Self(n)
                }
            }

            /// Creates a new instance without doing any bounds checking
            pub const unsafe fn new_unchecked(n: usize) -> Self {
                Self(n)
            }

            #[doc = "Create a new `"]
            #[doc = stringify!($id)]
            #[doc = "`, returning `None` if `n` is larger than `BOUND`"]
            pub fn try_new(n: usize) -> Option<Self> {
                if n > BOUND {
                    None
                } else {
                    Some($id(n))
                }
            }

            /// Produce the next value, wrapping from `BOUND` to 0
            pub fn next(&self) -> Self {
                if self.0 == BOUND {
                    Self(0)
                } else {
                    Self(self.0 + 1)
                }
            }

            /// Produce the previous value, wrapping from 0 to `BOUND`
            pub fn previous(&self) -> Self {
                if self.0 == 0 {
                    Self(BOUND)
                } else {
                    Self(self.0 - 1)
                }
            }

            /// An iterator over all values between 0..=BOUND
            pub fn all() -> impl Iterator<Item = Self> {
                (0..=BOUND).map(Self)
            }
        }

        paste! {
            bounded_serde!($id, [<$id Visitor>]);
        }
    };
}

macro_rules! bounded_serde {
    ($id:ident, $visitor:ident) => {
        struct $visitor<const BOUND: usize>;

        impl<'de, const BOUND: usize> Deserialize<'de> for $id<BOUND> {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_u64($visitor)
            }
        }

        impl<'de, const BOUND: usize> Visitor<'de> for $visitor<BOUND> {
            type Value = $id<BOUND>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "an integer between 0 and {}", BOUND)
            }

            fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
                let err = move || {
                    let msg = format!(
                        "{} out of range 0..={}, required by {}",
                        value,
                        BOUND,
                        stringify!($id)
                    );
                    E::custom(msg)
                };

                let num = value.try_into().map_err(|_| err())?;
                let bounded = Self::Value::try_new(num).ok_or_else(err)?;
                Ok(bounded)
            }
        }
    };
}

bounded_coord_component!(BoundedX, X);
bounded_coord_component!(BoundedY, Y);

#[cfg(test)]
mod tests {
    use super::*;

    /// This pollutes test output so I'm turning it off.
    // #[test]
    // #[should_panic(expected = "can't create lttcore::common::cartesian::bounded::BoundedRow<3> because 5 is larger than 3")]
    // fn test_new() {
    //     let _: BoundedRow<3> = BoundedRow::new(5);
    // }

    #[test]
    fn test_next_and_previous() {
        let x: BoundedX<3> = BoundedX::new(0);
        let expected = [0, 1, 2, 3, 0, 1, 2, 3];

        expected.into_iter().fold(x, |x, expected| {
            assert_eq!(usize::from(x), expected);
            x.next()
        });

        let expected = [0, 3, 2, 1, 0, 3, 2, 1];

        expected.into_iter().fold(x, |x, expected| {
            assert_eq!(usize::from(x), expected);
            x.previous()
        });
    }

    #[test]
    fn test_all() {
        assert!(BoundedX::<4>::all()
            .map(usize::from)
            .eq([0, 1, 2, 3, 4].into_iter()))
    }

    #[test]
    fn test_bounded_serde() {
        let row: BoundedX<3> = BoundedX::new(3);
        let serialized = serde_json::to_string(&row).unwrap();
        assert_eq!(serialized, "3");
        let deserialized: BoundedX<3> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(row, deserialized);

        let invalid: Result<BoundedX<3>, _> = serde_json::from_str("42");
        assert_eq!(
            &invalid.unwrap_err().to_string(),
            "42_u64 out of range 0..=3 required by BoundedX at line 1 column 2"
        );
        let invalid: Result<BoundedX<3>, _> = serde_json::from_str("-12");
        assert_eq!(
            &invalid.unwrap_err().to_string(),
            "-12_i64 out of range 0..=3 required by BoundedX at line 1 column 3"
        );
    }
}
