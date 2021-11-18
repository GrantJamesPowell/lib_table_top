use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Col(u8);

impl From<Col> for u8 {
    fn from(Col(n): Col) -> Self {
        n
    }
}

impl Col {
    /// Returns the column as a usize
    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }

    /// Returns the next column, wrapping back to 0 after 2
    ///
    /// ```
    /// use tic_tac_toe::Col;
    ///
    /// assert_eq!(Col::new(0).next(), Col::new(1));
    /// assert_eq!(Col::new(1).next(), Col::new(2));
    /// assert_eq!(Col::new(2).next(), Col::new(0));
    /// ```
    pub fn next(&self) -> Self {
        Self(match self.0 {
            0 => 1,
            1 => 2,
            2 => 0,
            _ => panic!("invalid index"),
        })
    }

    /// Returns the previous column, wrapping back to 2 after 0
    ///
    /// ```
    /// use tic_tac_toe::Col;
    ///
    /// assert_eq!(Col::new(2).previous(), Col::new(1));
    /// assert_eq!(Col::new(1).previous(), Col::new(0));
    /// assert_eq!(Col::new(0).previous(), Col::new(2));
    /// ```
    pub fn previous(&self) -> Self {
        Self(match self.0 {
            2 => 1,
            1 => 0,
            0 => 2,
            _ => panic!("invalid index"),
        })
    }

    /// Construct a new `Col` (see `Col::try_new` for a non panicking version)
    ///
    /// # Panics
    ///
    /// panics if n is outside of 0..=2
    ///
    /// ```should_panic
    /// use tic_tac_toe::Col;
    /// Col::new(1000);
    /// ```
    pub fn new(n: usize) -> Self {
        Self::try_new(n).expect("Invalid index, n must be within 0..=2")
    }

    /// Try to construct a `Col`, returning None if n is out of bounds
    ///
    /// ```
    /// use tic_tac_toe::Col;
    /// assert!(Col::try_new(1).is_some());
    /// assert!(Col::try_new(1000).is_none());
    /// ```
    pub fn try_new(n: usize) -> Option<Self> {
        match n {
            0 | 1 | 2 => Some(Self(n.try_into().unwrap())),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Row(u8);

impl From<Row> for u8 {
    fn from(Row(n): Row) -> Self {
        n
    }
}

impl Row {
    /// Returns the column as a usize
    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }

    /// Returns the next column, wrapping back to 0 after 2
    ///
    /// ```
    /// use tic_tac_toe::Row;
    ///
    /// assert_eq!(Row::new(0).next(), Row::new(1));
    /// assert_eq!(Row::new(1).next(), Row::new(2));
    /// assert_eq!(Row::new(2).next(), Row::new(0));
    /// ```
    pub fn next(&self) -> Self {
        Self(match self.0 {
            0 => 1,
            1 => 2,
            2 => 0,
            _ => panic!("invalid index"),
        })
    }

    /// Returns the previous column, wrapping back to 2 after 0
    ///
    /// ```
    /// use tic_tac_toe::Row;
    ///
    /// assert_eq!(Row::new(2).previous(), Row::new(1));
    /// assert_eq!(Row::new(1).previous(), Row::new(0));
    /// assert_eq!(Row::new(0).previous(), Row::new(2));
    /// ```
    pub fn previous(&self) -> Self {
        Self(match self.0 {
            2 => 1,
            1 => 0,
            0 => 2,
            _ => panic!("invalid index"),
        })
    }

    /// Construct a new Row (see `Row::try_new` for a non panicking version)
    ///
    /// # Panics
    ///
    /// panics if n is outside of 0..=2
    ///
    /// ```should_panic
    /// use tic_tac_toe::Row;
    /// Row::new(1000);
    /// ```
    pub fn new(n: usize) -> Self {
        Self::try_new(n).expect("Invalid index, n must be within 0..=2")
    }

    /// Try to construct a `Row`, returning None if n is out of bounds
    ///
    /// ```
    /// use tic_tac_toe::Row;
    /// assert!(Row::try_new(1).is_some());
    /// assert!(Row::try_new(1000).is_none());
    /// ```
    pub fn try_new(n: usize) -> Option<Self> {
        match n {
            0 | 1 | 2 => Some(Self(n.try_into().unwrap())),
            _ => None,
        }
    }
}

pub type Position = (Col, Row);

pub const POSSIBLE_WINS: [[(Col, Row); 3]; 8] = [
    // Fill up a row
    [(Col(0), Row(0)), (Col(0), Row(1)), (Col(0), Row(2))],
    [(Col(1), Row(0)), (Col(1), Row(1)), (Col(1), Row(2))],
    [(Col(2), Row(0)), (Col(2), Row(1)), (Col(2), Row(2))],
    // Fill up a col
    [(Col(0), Row(0)), (Col(1), Row(0)), (Col(2), Row(0))],
    [(Col(0), Row(1)), (Col(1), Row(1)), (Col(2), Row(1))],
    [(Col(0), Row(2)), (Col(1), Row(2)), (Col(2), Row(2))],
    // Diagonal
    [(Col(0), Row(0)), (Col(1), Row(1)), (Col(2), Row(2))],
    [(Col(2), Row(0)), (Col(1), Row(1)), (Col(0), Row(2))],
];
