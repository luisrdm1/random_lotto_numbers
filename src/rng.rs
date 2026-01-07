//! Random number generator abstraction.
//!
//! This module provides a trait for random number generation,
//! allowing different RNG implementations to be plugged in
//! (e.g., simple random, Sobol sampling, quasi-random sequences).

use rand::Rng;

/// Trait for random number generation abstraction.
///
/// This trait allows different random number generators to be used
/// interchangeably in the lottery ticket generation system.
///
/// # Examples
///
/// ```
/// use rand::rng;
/// use lotto_quick_pick::rng::RandomNumberGenerator;
///
/// let mut rng = rand::rng();
/// let random_number = rng.gen_range_u8(1, 60);
/// assert!(random_number >= 1 && random_number <= 60);
/// ```
pub trait RandomNumberGenerator {
    /// Generate a random u8 value within the specified range [low, high].
    ///
    /// # Arguments
    ///
    /// * `low` - The lower bound (inclusive)
    /// * `high` - The upper bound (inclusive)
    ///
    /// # Returns
    ///
    /// A random u8 value between low and high (inclusive)
    fn gen_range_u8(&mut self, low: u8, high: u8) -> u8;
}

/// Implementation of RandomNumberGenerator for rand's ThreadRng.
impl<R: Rng> RandomNumberGenerator for R {
    fn gen_range_u8(&mut self, low: u8, high: u8) -> u8 {
        self.random_range(low..=high)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock RNG for testing that returns predictable values.
    struct MockRng {
        values: Vec<u8>,
        index: usize,
    }

    impl MockRng {
        fn new(values: Vec<u8>) -> Self {
            Self { values, index: 0 }
        }
    }

    impl RandomNumberGenerator for MockRng {
        fn gen_range_u8(&mut self, _low: u8, _high: u8) -> u8 {
            let value = self.values[self.index % self.values.len()];
            self.index += 1;
            value
        }
    }

    #[test]
    fn test_mock_rng_returns_predictable_values() {
        let mut rng = MockRng::new(vec![5, 10, 15]);

        assert_eq!(rng.gen_range_u8(1, 60), 5);
        assert_eq!(rng.gen_range_u8(1, 60), 10);
        assert_eq!(rng.gen_range_u8(1, 60), 15);
        assert_eq!(rng.gen_range_u8(1, 60), 5); // Wraps around
    }

    #[test]
    fn test_thread_rng_generates_within_range() {
        let mut rng = rand::rng();

        for _ in 0..100 {
            let value = rng.gen_range_u8(1, 60);
            assert!((1..=60).contains(&value));
        }
    }
}
