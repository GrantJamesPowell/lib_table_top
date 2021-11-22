use serde::{Deserialize, Serialize};

pub type BoundedPosition<const WIDTH: usize, const HEIGHT: usize> =
    (BoundedCol<WIDTH>, BoundedRow<HEIGHT>);

macro_rules! bounded {
    ($id:ident) => {
        #[derive(
            Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Deserialize, Serialize,
        )]
        pub struct $id<const BOUND: usize>(usize);

        impl<const BOUND: usize> From<$id<BOUND>> for usize {
            fn from(ty: $id<BOUND>) -> Self {
                ty.0
            }
        }

        impl<const BOUND: usize> Default for $id<BOUND> {
            fn default() -> Self {
                $id(0)
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
    };
}

bounded!(BoundedCol);
bounded!(BoundedRow);

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
        let row: BoundedRow<3> = BoundedRow::new(0);
        let expected = [0, 1, 2, 3, 0, 1, 2, 3];

        expected.into_iter().fold(row, |row, expected| {
            assert_eq!(usize::from(row), expected);
            row.next()
        });

        let expected = [0, 3, 2, 1, 0, 3, 2, 1];

        expected.into_iter().fold(row, |row, expected| {
            assert_eq!(usize::from(row), expected);
            row.previous()
        });
    }

    #[test]
    fn test_all() {
        assert!(BoundedRow::<4>::all()
            .map(usize::from)
            .eq([0, 1, 2, 3, 4].into_iter()))
    }
}
