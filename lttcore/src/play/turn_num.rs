use serde::{Deserialize, Serialize};

/// New type wrapper around the current turn number
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct TurnNum(u64);

impl From<u64> for TurnNum {
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl From<TurnNum> for u64 {
    fn from(turn_num: TurnNum) -> u64 {
        turn_num.0
    }
}

impl TurnNum {
    /// Increment the [`TurnNum`]
    ///
    /// ```
    /// use lttcore::play::TurnNum;
    ///
    /// let mut turn_num = TurnNum::from(10);
    /// turn_num.increment();
    /// turn_num.increment();
    /// turn_num.increment();
    /// assert_eq!(u64::from(turn_num), 13)
    /// ```
    ///
    /// # Panics
    ///
    /// This panics on overflow
    ///
    /// ```should_panic
    /// use lttcore::play::TurnNum;
    ///
    /// let mut turn_num = TurnNum::from(u64::MAX);
    /// turn_num.increment();
    /// ```
    pub fn increment(&mut self) {
        self.0 = self
            .0
            .checked_add(1)
            .expect("turn number does not overflow");
    }

    /// Returns the next [`TurnNum`]
    ///
    /// ```
    /// use lttcore::play::TurnNum;
    ///
    /// let turn_num = TurnNum::from(10);
    /// let next = turn_num.next();
    /// assert_eq!(u64::from(next), 11)
    /// ```
    ///
    /// # Panics
    ///
    /// This panics on overflow
    ///
    /// ```should_panic
    /// use lttcore::play::TurnNum;
    ///
    /// let turn_num = TurnNum::from(u64::MAX);
    /// let _ = turn_num.next();
    /// ```
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}
