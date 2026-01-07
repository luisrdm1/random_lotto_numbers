//! Lotto Quick Pick - A library for generating lottery tickets.
//!
//! This library provides functionality to generate unique lottery tickets
//! with configurable parameters and calculate winning probabilities.
//!
//! # Features
//!
//! - Type-safe configuration using newtypes
//! - Pluggable random number generators
//! - Efficient ticket generation (uses optimal strategy based on pick size)
//! - Probability calculation without factorial (no overflow for practical lotteries)
//! - Comprehensive error handling
//!
//! # Examples
//!
//! ```
//! use lotto_quick_pick::{Config, generate_tickets};
//! use rand::rng;
//!
//! let config = Config::new(10, 1, 60, 6).unwrap();
//! let mut rng = rand::rng();
//! let tickets = generate_tickets(&mut rng, &config).unwrap();
//!
//! assert_eq!(tickets.len(), 10);
//! ```

pub mod error;
pub mod newtypes;
pub mod probability;
pub mod rng;
pub mod ticket_bitwise;
pub mod ticket;

pub use error::{LottoError, Result};
pub use newtypes::{BallNumber, BallRange, GameCount, PickCount, Ticket};
pub use probability::{calculate_probability, combination};
pub use rng::RandomNumberGenerator;
pub use ticket::{generate_ticket, generate_unique_tickets};

/// Configuration for lottery ticket generation.
///
/// This struct holds all necessary parameters for generating lottery tickets
/// in a type-safe manner.
#[derive(Debug, Clone)]
pub struct Config {
    game_count: GameCount,
    range: BallRange,
    pick: PickCount,
}

impl Config {
    /// Create a new Config from raw values.
    ///
    /// # Arguments
    ///
    /// * `games` - Number of unique tickets to generate
    /// * `start` - Starting ball number (inclusive)
    /// * `end` - Ending ball number (inclusive)
    /// * `pick` - Number of balls to pick per ticket
    ///
    /// # Returns
    ///
    /// A Config if all parameters are valid, otherwise a LottoError.
    ///
    /// # Examples
    ///
    /// ```
    /// use lotto_quick_pick::Config;
    ///
    /// // Mega-Sena configuration: 6 from 60, generate 10 tickets
    /// let config = Config::new(10, 1, 60, 6).unwrap();
    /// ```
    pub fn new(games: usize, start: u8, end: u8, pick: usize) -> Result<Self> {
        let game_count = GameCount::new(games)?;
        
        // Ensure start <= end by swapping if necessary
        let (start, end) = if start <= end {
            (start, end)
        } else {
            (end, start)
        };
        
        let range = BallRange::new(BallNumber::new(start), BallNumber::new(end))?;
        let pick_count = PickCount::new(pick, &range)?;

        Ok(Self {
            game_count,
            range,
            pick: pick_count,
        })
    }

    /// Get the number of games to generate.
    pub fn game_count(&self) -> &GameCount {
        &self.game_count
    }

    /// Get the ball range.
    pub fn range(&self) -> &BallRange {
        &self.range
    }

    /// Get the pick count.
    pub fn pick(&self) -> &PickCount {
        &self.pick
    }
}

/// Generate lottery tickets using the provided configuration.
///
/// This is a convenience function that uses the configuration to generate
/// unique lottery tickets.
///
/// # Arguments
///
/// * `rng` - Random number generator
/// * `config` - Lottery configuration
///
/// # Returns
///
/// A Result containing a vector of unique tickets, or an error.
///
/// # Examples
///
/// ```
/// use lotto_quick_pick::{Config, generate_tickets};
/// use rand::rng;
///
/// let config = Config::new(5, 1, 60, 6).unwrap();
/// let mut rng = rand::rng();
/// let tickets = generate_tickets(&mut rng, &config).unwrap();
///
/// assert_eq!(tickets.len(), 5);
/// ```
pub fn generate_tickets<R: RandomNumberGenerator>(
    rng: &mut R,
    config: &Config,
) -> Result<Vec<Ticket>> {
    generate_unique_tickets(rng, config.range(), config.pick(), config.game_count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new_valid() {
        let config = Config::new(10, 1, 60, 6).unwrap();
        assert_eq!(config.game_count().value(), 10);
        assert_eq!(config.range().start().value(), 1);
        assert_eq!(config.range().end().value(), 60);
        assert_eq!(config.pick().value(), 6);
    }

    #[test]
    fn test_config_new_swaps_start_end() {
        let config = Config::new(1, 60, 1, 6).unwrap();
        assert_eq!(config.range().start().value(), 1);
        assert_eq!(config.range().end().value(), 60);
    }

    #[test]
    fn test_config_new_invalid_zero_games() {
        let result = Config::new(0, 1, 60, 6);
        assert!(matches!(result, Err(LottoError::ZeroGames)));
    }

    #[test]
    fn test_config_new_invalid_pick_exceeds_range() {
        let result = Config::new(1, 1, 10, 15);
        assert!(matches!(result, Err(LottoError::PickExceedsRange { .. })));
    }

    #[test]
    fn test_config_new_invalid_equal_start_end() {
        let result = Config::new(1, 10, 10, 1);
        assert!(matches!(result, Err(LottoError::InvalidRange { .. })));
    }

    #[test]
    fn test_generate_tickets_returns_correct_count() {
        let mut rng = rand::rng();
        let config = Config::new(5, 1, 60, 6).unwrap();
        let tickets = generate_tickets(&mut rng, &config).unwrap();
        assert_eq!(tickets.len(), 5);
    }

    #[test]
    fn test_generate_tickets_all_unique() {
        let mut rng = rand::rng();
        let config = Config::new(10, 1, 60, 6).unwrap();
        let tickets = generate_tickets(&mut rng, &config).unwrap();
        
        use std::collections::HashSet;
        let unique_tickets: HashSet<_> = tickets.iter().collect();
        assert_eq!(unique_tickets.len(), 10);
    }

    #[test]
    fn test_generate_tickets_too_many() {
        let mut rng = rand::rng();
        let config = Config::new(11, 1, 5, 3).unwrap();
        // C(5,3) = 10, so requesting 11 should fail
        let result = generate_tickets(&mut rng, &config);
        assert!(matches!(result, Err(LottoError::TooManyUniqueGames { .. })));
    }
}
