//! Type-safe wrappers for domain primitives.
//!
//! This module provides newtype wrappers around primitive types
//! to enforce domain constraints and improve type safety.

use crate::error::{LottoError, Result};
use std::fmt;

/// Represents a ball number in the lottery.
///
/// Ensures that ball numbers are always within valid range (1-255).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BallNumber(u8);

impl BallNumber {
    /// Create a new BallNumber.
    ///
    /// # Arguments
    ///
    /// * `value` - The ball number value
    ///
    /// # Returns
    ///
    /// A BallNumber instance.
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    /// Get the inner value.
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl fmt::Display for BallNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

/// Represents the range of ball numbers for a lottery game.
///
/// Ensures that the start value is less than the end value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BallRange {
    start: BallNumber,
    end: BallNumber,
}

impl BallRange {
    /// Create a new BallRange.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting ball number (inclusive)
    /// * `end` - The ending ball number (inclusive)
    ///
    /// # Returns
    ///
    /// A BallRange if start < end, otherwise an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use lotto_quick_pick::newtypes::{BallRange, BallNumber};
    ///
    /// let range = BallRange::new(BallNumber::new(1), BallNumber::new(60)).unwrap();
    /// assert_eq!(range.size(), 60);
    /// ```
    pub fn new(start: BallNumber, end: BallNumber) -> Result<Self> {
        if start.value() >= end.value() {
            return Err(LottoError::InvalidRange {
                start: start.value(),
                end: end.value(),
            });
        }
        Ok(Self { start, end })
    }

    /// Get the starting ball number.
    pub fn start(&self) -> BallNumber {
        self.start
    }

    /// Get the ending ball number.
    pub fn end(&self) -> BallNumber {
        self.end
    }

    /// Get the size of the range (number of possible values).
    pub fn size(&self) -> usize {
        (self.end.value() - self.start.value() + 1) as usize
    }
}

/// Represents the number of balls to pick per game.
///
/// Ensures that the pick count is valid for the given range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PickCount(usize);

impl PickCount {
    /// Create a new PickCount.
    ///
    /// # Arguments
    ///
    /// * `value` - The number of balls to pick
    /// * `range` - The range of available balls
    ///
    /// # Returns
    ///
    /// A PickCount if the value is valid for the range, otherwise an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use lotto_quick_pick::newtypes::{PickCount, BallRange, BallNumber};
    ///
    /// let range = BallRange::new(BallNumber::new(1), BallNumber::new(60)).unwrap();
    /// let pick = PickCount::new(6, &range).unwrap();
    /// assert_eq!(pick.value(), 6);
    /// ```
    pub fn new(value: usize, range: &BallRange) -> Result<Self> {
        if value > range.size() {
            return Err(LottoError::PickExceedsRange {
                pick: value,
                available: range.size(),
            });
        }
        if value == 0 {
            return Err(LottoError::PickExceedsRange {
                pick: 0,
                available: range.size(),
            });
        }
        Ok(Self(value))
    }

    /// Get the inner value.
    pub fn value(&self) -> usize {
        self.0
    }
}

/// Represents the number of games to generate.
///
/// Ensures that at least one game is generated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameCount(usize);

impl GameCount {
    /// Create a new GameCount.
    ///
    /// # Arguments
    ///
    /// * `value` - The number of games to generate
    ///
    /// # Returns
    ///
    /// A GameCount if the value is at least 1, otherwise an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use lotto_quick_pick::newtypes::GameCount;
    ///
    /// let count = GameCount::new(10).unwrap();
    /// assert_eq!(count.value(), 10);
    ///
    /// assert!(GameCount::new(0).is_err());
    /// ```
    pub fn new(value: usize) -> Result<Self> {
        if value == 0 {
            return Err(LottoError::ZeroGames);
        }
        Ok(Self(value))
    }

    /// Get the inner value.
    pub fn value(&self) -> usize {
        self.0
    }
}

/// Represents a single lottery ticket containing unique ball numbers.
///
/// The balls are stored in sorted order for consistency.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ticket {
    balls: Vec<BallNumber>,
}

impl Ticket {
    /// Create a new Ticket from a vector of ball numbers.
    ///
    /// The balls will be automatically sorted.
    ///
    /// # Arguments
    ///
    /// * `balls` - Vector of ball numbers
    ///
    /// # Examples
    ///
    /// ```
    /// use lotto_quick_pick::newtypes::{Ticket, BallNumber};
    ///
    /// let ticket = Ticket::new(vec![
    ///     BallNumber::new(5),
    ///     BallNumber::new(10),
    ///     BallNumber::new(15),
    /// ]);
    /// assert_eq!(ticket.balls().len(), 3);
    /// ```
    pub fn new(mut balls: Vec<BallNumber>) -> Self {
        balls.sort_unstable();
        Self { balls }
    }

    /// Get a reference to the ball numbers.
    pub fn balls(&self) -> &[BallNumber] {
        &self.balls
    }
}

impl fmt::Display for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted: Vec<String> = self.balls.iter().map(|b| b.to_string()).collect();
        write!(f, "{}", formatted.join(" "))
    }
}

// Trait implementations for better ergonomics

impl From<BallNumber> for u8 {
    fn from(ball: BallNumber) -> u8 {
        ball.value()
    }
}

impl TryFrom<u8> for BallNumber {
    type Error = crate::error::LottoError;

    fn try_from(value: u8) -> crate::error::Result<Self> {
        // Accept all u8 values including 0 (valid for Lotomania 0-99)
        Ok(BallNumber::new(value))
    }
}

// Additional BallRange methods

impl BallRange {
    /// Iterate over all ball numbers in this range.
    ///
    /// # Examples
    ///
    /// ```
    /// use lotto_quick_pick::newtypes::{BallRange, BallNumber};
    ///
    /// let range = BallRange::new(BallNumber::new(1), BallNumber::new(5)).unwrap();
    /// let balls: Vec<_> = range.iter().collect();
    /// assert_eq!(balls.len(), 5);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = BallNumber> + '_ {
        (self.start.value()..=self.end.value()).map(BallNumber::new)
    }

    /// Check if a ball number is within this range.
    ///
    /// # Examples
    ///
    /// ```
    /// use lotto_quick_pick::newtypes::{BallRange, BallNumber};
    ///
    /// let range = BallRange::new(BallNumber::new(1), BallNumber::new(60)).unwrap();
    /// assert!(range.contains(BallNumber::new(30)));
    /// assert!(!range.contains(BallNumber::new(61)));
    /// ```
    pub fn contains(&self, ball: BallNumber) -> bool {
        ball >= self.start && ball <= self.end
    }

    /// Create a range for Brazilian Mega-Sena lottery (1-60, pick 6).
    ///
    /// # Examples
    ///
    /// ```
    /// use lotto_quick_pick::newtypes::BallRange;
    ///
    /// let range = BallRange::mega_sena();
    /// assert_eq!(range.size(), 60);
    /// ```
    pub fn mega_sena() -> Self {
        Self {
            start: BallNumber::new(1),
            end: BallNumber::new(60),
        }
    }

    /// Create a range for Brazilian Lotomania lottery (0-99, pick 50).
    ///
    /// # Examples
    ///
    /// ```
    /// use lotto_quick_pick::newtypes::BallRange;
    ///
    /// let range = BallRange::lotomania();
    /// assert_eq!(range.size(), 100);
    /// ```
    pub fn lotomania() -> Self {
        Self {
            start: BallNumber::new(0),
            end: BallNumber::new(99),
        }
    }

    /// Create a range for US Powerball lottery (1-69).
    ///
    /// # Examples
    ///
    /// ```
    /// use lotto_quick_pick::newtypes::BallRange;
    ///
    /// let range = BallRange::powerball();
    /// assert_eq!(range.size(), 69);
    /// ```
    pub fn powerball() -> Self {
        Self {
            start: BallNumber::new(1),
            end: BallNumber::new(69),
        }
    }
}

// Additional Ticket methods

impl Ticket {
    /// Get the number of balls in this ticket.
    ///
    /// # Examples
    ///
    /// ```
    /// use lotto_quick_pick::newtypes::{Ticket, BallNumber};
    ///
    /// let ticket = Ticket::new(vec![
    ///     BallNumber::new(5),
    ///     BallNumber::new(10),
    ///     BallNumber::new(15),
    /// ]);
    /// assert_eq!(ticket.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.balls.len()
    }

    /// Check if the ticket is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use lotto_quick_pick::newtypes::{Ticket, BallNumber};
    ///
    /// let ticket = Ticket::new(vec![]);
    /// assert!(ticket.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.balls.is_empty()
    }

    /// Check if this ticket contains a specific ball number.
    ///
    /// # Examples
    ///
    /// ```
    /// use lotto_quick_pick::newtypes::{Ticket, BallNumber};
    ///
    /// let ticket = Ticket::new(vec![
    ///     BallNumber::new(5),
    ///     BallNumber::new(10),
    ///     BallNumber::new(15),
    /// ]);
    /// assert!(ticket.contains(&BallNumber::new(10)));
    /// assert!(!ticket.contains(&BallNumber::new(20)));
    /// ```
    pub fn contains(&self, ball: &BallNumber) -> bool {
        self.balls.contains(ball)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ball_number_creation() {
        let ball = BallNumber::new(42);
        assert_eq!(ball.value(), 42);
    }

    #[test]
    fn test_ball_number_display() {
        assert_eq!(format!("{}", BallNumber::new(5)), "05");
        assert_eq!(format!("{}", BallNumber::new(42)), "42");
    }

    #[test]
    fn test_ball_range_valid() {
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(60)).unwrap();
        assert_eq!(range.start().value(), 1);
        assert_eq!(range.end().value(), 60);
        assert_eq!(range.size(), 60);
    }

    #[test]
    fn test_ball_range_invalid() {
        let result = BallRange::new(BallNumber::new(60), BallNumber::new(1));
        assert!(matches!(result, Err(LottoError::InvalidRange { .. })));
    }

    #[test]
    fn test_ball_range_equal() {
        let result = BallRange::new(BallNumber::new(10), BallNumber::new(10));
        assert!(matches!(result, Err(LottoError::InvalidRange { .. })));
    }

    #[test]
    fn test_pick_count_valid() {
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(60)).unwrap();
        let pick = PickCount::new(6, &range).unwrap();
        assert_eq!(pick.value(), 6);
    }

    #[test]
    fn test_pick_count_exceeds_range() {
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(10)).unwrap();
        let result = PickCount::new(11, &range);
        assert!(matches!(result, Err(LottoError::PickExceedsRange { .. })));
    }

    #[test]
    fn test_pick_count_zero() {
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(60)).unwrap();
        let result = PickCount::new(0, &range);
        assert!(matches!(result, Err(LottoError::PickExceedsRange { .. })));
    }

    #[test]
    fn test_game_count_valid() {
        let count = GameCount::new(10).unwrap();
        assert_eq!(count.value(), 10);
    }

    #[test]
    fn test_game_count_zero() {
        let result = GameCount::new(0);
        assert!(matches!(result, Err(LottoError::ZeroGames)));
    }

    #[test]
    fn test_ticket_creation() {
        let ticket = Ticket::new(vec![
            BallNumber::new(15),
            BallNumber::new(5),
            BallNumber::new(10),
        ]);

        // Should be sorted
        assert_eq!(ticket.balls()[0].value(), 5);
        assert_eq!(ticket.balls()[1].value(), 10);
        assert_eq!(ticket.balls()[2].value(), 15);
    }

    #[test]
    fn test_ticket_display() {
        let ticket = Ticket::new(vec![
            BallNumber::new(5),
            BallNumber::new(10),
            BallNumber::new(15),
        ]);
        assert_eq!(format!("{}", ticket), "05 10 15");
    }

    #[test]
    fn test_ticket_equality() {
        let ticket1 = Ticket::new(vec![BallNumber::new(5), BallNumber::new(10)]);
        let ticket2 = Ticket::new(vec![BallNumber::new(10), BallNumber::new(5)]);
        // Should be equal even if created in different order
        assert_eq!(ticket1, ticket2);
    }
}
