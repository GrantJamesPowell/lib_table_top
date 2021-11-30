//! Statistics calculations
#![allow(missing_docs)]
#![allow(clippy::cast_precision_loss)]

use serde::{Deserialize, Serialize};

/// A representation of a single statistic.
///
/// Uses a [`usize`] and 4 [`u64`]s to keep track of sample:
/// * Count
/// * Min
/// * Max
/// * Mean (x̄)
/// * Standard Deviation (σ)
/// * Variance (σ^2)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Stat {
    count: usize,
    mean: f64,
    sum_of_squares: f64,
    max_value: f64,
    min_value: f64,
}

impl Default for Stat {
    fn default() -> Self {
        Self {
            count: 0,
            mean: 0.,
            sum_of_squares: 0.,
            max_value: 0.,
            min_value: f64::MAX,
        }
    }
}

impl FromIterator<f64> for Stat {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        let mut stat = Stat::new();

        for sample in iter {
            stat.add_sample(sample);
        }

        stat
    }
}

impl Stat {
    /// Create a new empty [`Stat`]
    ///
    /// ```
    /// use lttcore::utilities::stat::Stat;
    ///
    /// let stat = Stat::new();
    /// assert_eq!(stat, Stat::default());
    /// assert_eq!(stat.sample_count(), 0);
    /// assert_eq!(stat.min_value(), None);
    /// assert_eq!(stat.max_value(), None);
    /// assert_eq!(stat.mean(), 0.);
    /// assert_eq!(stat.variance(), 0.);
    /// assert_eq!(stat.standard_deviation(), 0.);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a [`Stat`] from an iterator of [`f64`]
    ///
    /// ```
    /// use lttcore::utilities::stat::Stat;
    ///
    /// // 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 (note: it's 11 numbers)
    /// let stat = Stat::from_samples((0..=10).map(|i| i as f64));
    /// assert_eq!(stat.sample_count(), 11);
    /// assert_eq!(stat.min_value(), Some(0.));
    /// assert_eq!(stat.max_value(), Some(10.));
    /// assert_eq!(stat.mean(), 5.);
    /// assert_eq!(stat.variance(), 10.0);
    ///
    /// assert_eq!(stat.standard_deviation(), 10.0_f64.sqrt());
    /// ```
    pub fn from_samples(samples: impl IntoIterator<Item = f64>) -> Self {
        samples.into_iter().collect()
    }

    /// Add a sample to the [`Stat`]
    ///
    /// This uses [Welford's online
    /// algorithm](https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Welford's_online_algorithm)
    /// to keep track of mean and variance.
    pub fn add_sample(&mut self, sample: f64) {
        self.count += 1;
        let delta = sample - self.mean;
        self.mean += delta / (self.count as f64);
        let delta2 = sample - self.mean;
        self.sum_of_squares += delta * delta2;

        // Update Min/Max values
        if sample <= self.min_value {
            self.min_value = sample;
        }

        if sample >= self.max_value {
            self.max_value = sample;
        }
    }

    /// Add a [`Stat`] to the [`Stat`]
    ///
    /// Uses the parallel version of [Welford's online
    /// algorithm](https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Parallel_algorithm)
    ///
    /// ```
    /// use lttcore::utilities::stat::Stat;
    ///
    /// let mut stat1: Stat = (0..5).map(|i| i as f64).collect();
    /// let stat2: Stat = (5..10).map(|i| i as f64).collect();
    ///
    /// let expected_combined: Stat = (0..10).map(|i| i as f64).collect();
    ///
    /// stat1.add_stat(stat2);
    ///
    /// assert_eq!(stat1, expected_combined);
    /// ```
    pub fn add_stat(&mut self, other: Self) {
        let combined_count = self.count + other.count;
        let delta = other.mean - self.mean;
        let combined_mean = self.mean + (delta * (other.count as f64 / combined_count as f64));
        let combined_sum_of_squares = (self.sum_of_squares + other.sum_of_squares)
            + ((delta.powi(2) * (self.count * other.count) as f64) / combined_count as f64);

        self.count = combined_count;
        self.mean = combined_mean;
        self.sum_of_squares = combined_sum_of_squares;
        self.min_value = self.min_value.min(other.min_value);
        self.max_value = self.max_value.max(other.max_value);
    }

    /// Combine two [`Stat`] into a single [`Stat`]
    ///
    /// Uses the parallel version of [Welford's online
    /// algorithm](https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Parallel_algorithm)
    ///
    /// ```
    /// use lttcore::utilities::stat::Stat;
    ///
    /// let stat1: Stat = (0..5).map(|i| i as f64).collect();
    /// let stat2: Stat = (5..10).map(|i| i as f64).collect();
    ///
    /// let expected_combined: Stat = (0..10).map(|i| i as f64).collect();
    ///
    /// assert_eq!(stat1.combine_with(&stat2), expected_combined);
    /// ```
    pub fn combine_with(&self, other: &Self) -> Self {
        let mut new = *self;
        new.add_stat(*other);
        new
    }

    /// Returns the mean of the samples this [`Stat`] has seen
    ///
    /// ```
    /// use lttcore::utilities::stat::Stat;
    ///
    /// let mut stat = Stat::new();
    /// assert_eq!(stat.mean(), 0.);
    /// stat.add_sample(10.);
    /// assert_eq!(stat.mean(), 10.);
    /// stat.add_sample(0.);
    /// assert_eq!(stat.mean(), 5.);
    /// ```
    pub fn mean(&self) -> f64 {
        self.mean
    }

    pub fn standard_deviation(&self) -> f64 {
        self.variance().sqrt()
    }

    pub fn variance(&self) -> f64 {
        match self.count {
            0 => 0.,
            _ => self.sum_of_squares / (self.count as f64),
        }
    }

    /// Return the number of samples this [`Stat`] has received
    ///
    /// ```
    /// use lttcore::utilities::stat::Stat;
    ///
    /// let mut stat = Stat::new();
    /// assert_eq!(stat.sample_count(), 0);
    /// stat.add_sample(1.0);
    /// assert_eq!(stat.sample_count(), 1);
    /// ```
    pub fn sample_count(&self) -> usize {
        self.count
    }

    /// Return the maximum sample value this [`Stat`] has seen
    ///
    /// This function returns `None` if there haven't been any stats added yet
    ///
    /// ```
    /// use lttcore::utilities::stat::Stat;
    ///
    /// let mut stat = Stat::new();
    /// assert_eq!(stat.max_value(), None);
    /// stat.add_sample(1.0);
    /// assert_eq!(stat.max_value(), Some(1.0));
    /// stat.add_sample(0.0);
    /// assert_eq!(stat.max_value(), Some(1.0));
    /// ```
    pub fn max_value(&self) -> Option<f64> {
        match self.count {
            0 => None,
            _ => Some(self.max_value),
        }
    }

    /// Return the minimum sample value this [`Stat`] has seen
    ///
    /// This function returns `None` if there haven't been any stats added yet
    ///
    /// ```
    /// use lttcore::utilities::stat::Stat;
    ///
    /// let mut stat = Stat::new();
    /// assert_eq!(stat.min_value(), None);
    /// stat.add_sample(1.0);
    /// assert_eq!(stat.min_value(), Some(1.0));
    /// stat.add_sample(0.0);
    /// assert_eq!(stat.min_value(), Some(0.0));
    /// ```
    pub fn min_value(&self) -> Option<f64> {
        match self.count {
            0 => None,
            _ => Some(self.min_value),
        }
    }
}
