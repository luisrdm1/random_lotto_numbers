//! Bitwise/bitmask optimization strategies for lottery ticket generation.
//!
//! This module implements alternative ticket generation algorithms using bitwise operations
//! instead of HashSet. The goal is to measure if bitmap-based duplicate checking provides
//! measurable performance improvements over hash-based approaches.
//!
//! # Strategies
//!
//! - **u64 bitmap**: For ranges ≤ 64 (e.g., Mega-Sena with range 1-60)
//! - **u128 bitmap**: For ranges ≤ 128 (e.g., Lotomania with range 0-99)
//! - **Vec<u64> bitmap**: For ranges ≤ 512 (supporting larger lottery systems)
//!
//! # Expected Performance
//!
//! Bitwise AND operations for duplicate checking should be O(1) vs HashSet's O(1) amortized,
//! but with better cache locality and no hashing overhead. Expected gains: 30-50% for small ranges.

use crate::error::LottoError;
use crate::newtypes::{BallNumber, BallRange, PickCount};
use crate::rng::RandomNumberGenerator;
use std::vec::Vec;

/// Determines which bitmap strategy to use based on the range size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitwiseStrategy {
    /// Use u64 bitmap for ranges ≤ 64
    U64,
    /// Use u128 bitmap for ranges ≤ 128
    U128,
    /// Use Vec<u64> bitmap for ranges ≤ 512
    VecU64,
}

impl BitwiseStrategy {
    /// Selects the appropriate strategy based on the ball range.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of ball numbers
    ///
    /// # Returns
    ///
    /// The optimal bitwise strategy or an error if range > 255
    pub fn select(range: &BallRange) -> Result<Self, LottoError> {
        let max_value = range.end().value();
        
        if max_value <= 64 {
            Ok(Self::U64)
        } else if max_value <= 128 {
            Ok(Self::U128)
        } else {
            // For u8, max is 255, so this will handle 129-255
            Ok(Self::VecU64)
        }
    }
}

/// Generates a lottery ticket using u64 bitmap for duplicate checking.
///
/// This strategy works for ranges where max value ≤ 64.
///
/// # Time Complexity
///
/// O(k) where k is the number of picks. Each duplicate check is O(1).
///
/// # Arguments
///
/// * `range` - The range of ball numbers (must have max ≤ 64)
/// * `count` - Number of balls to pick
/// * `rng` - Random number generator
///
/// # Returns
///
/// A vector of unique ball numbers or an error
pub fn generate_ticket_u64_bitmap(
    range: &BallRange,
    count: PickCount,
    rng: &mut dyn RandomNumberGenerator,
) -> Result<Vec<BallNumber>, LottoError> {
    let min = range.start().value();
    let max = range.end().value();
    let picks = count.value();
    
    // Validate range fits in u64
    if max > 64 {
        return Err(LottoError::InvalidRange {
            start: min,
            end: max,
        });
    }
    
    let mut bitmap: u64 = 0;
    let mut ticket = Vec::with_capacity(picks);
    
    while ticket.len() < picks {
        let ball_value = rng.gen_range_u8(min, max);
        let bit_position = ball_value - min; // Normalize to 0-based
        let bit_mask = 1u64 << bit_position;
        
        // Check if ball already picked using bitwise AND
        if bitmap & bit_mask == 0 {
            bitmap |= bit_mask; // Set the bit
            ticket.push(BallNumber::new(ball_value));
        }
    }
    
    Ok(ticket)
}

/// Generates a lottery ticket using u128 bitmap for duplicate checking.
///
/// This strategy works for ranges where max value ≤ 128.
///
/// # Time Complexity
///
/// O(k) where k is the number of picks. Each duplicate check is O(1).
///
/// # Arguments
///
/// * `range` - The range of ball numbers (must have max ≤ 128)
/// * `count` - Number of balls to pick
/// * `rng` - Random number generator
///
/// # Returns
///
/// A vector of unique ball numbers or an error
pub fn generate_ticket_u128_bitmap(
    range: &BallRange,
    count: PickCount,
    rng: &mut dyn RandomNumberGenerator,
) -> Result<Vec<BallNumber>, LottoError> {
    let min = range.start().value();
    let max = range.end().value();
    let picks = count.value();
    
    // Validate range fits in u128
    if max > 128 {
        return Err(LottoError::InvalidRange {
            start: min,
            end: max,
        });
    }
    
    let mut bitmap: u128 = 0;
    let mut ticket = Vec::with_capacity(picks);
    
    while ticket.len() < picks {
        let ball_value = rng.gen_range_u8(min, max);
        let bit_position = ball_value - min; // Normalize to 0-based
        let bit_mask = 1u128 << bit_position;
        
        // Check if ball already picked using bitwise AND
        if bitmap & bit_mask == 0 {
            bitmap |= bit_mask; // Set the bit
            ticket.push(BallNumber::new(ball_value));
        }
    }
    
    Ok(ticket)
}

/// Generates a lottery ticket using Vec<u64> bitmap for duplicate checking.
///
/// This strategy works for ranges where max value ≤ 512 (8 × 64).
///
/// # Time Complexity
///
/// O(k) where k is the number of picks. Each duplicate check is O(1).
///
/// # Arguments
///
/// * `range` - The range of ball numbers (must have max ≤ 512)
/// * `count` - Number of balls to pick
/// * `rng` - Random number generator
///
/// # Returns
///
/// A vector of unique ball numbers or an error
pub fn generate_ticket_vec_bitmap(
    range: &BallRange,
    count: PickCount,
    rng: &mut dyn RandomNumberGenerator,
) -> Result<Vec<BallNumber>, LottoError> {
    let min = range.start().value();
    let max = range.end().value();
    let picks = count.value();
    
    // Validate range fits in Vec<u64> (u8 max is 255, so always fits)
    // Convert to larger types to avoid overflow
    let range_size = (max as usize) - (min as usize) + 1;
    let vec_size = (range_size + 63) / 64; // Ceiling division
    let mut bitmap = vec![0u64; vec_size];
    let mut ticket = Vec::with_capacity(picks);
    
    while ticket.len() < picks {
        let ball_value = rng.gen_range_u8(min, max);
        let bit_position = (ball_value - min) as usize; // Normalize to 0-based
        let vec_index = bit_position / 64;
        let bit_offset = bit_position % 64;
        let bit_mask = 1u64 << bit_offset;
        
        // Check if ball already picked using bitwise AND
        if bitmap[vec_index] & bit_mask == 0 {
            bitmap[vec_index] |= bit_mask; // Set the bit
            ticket.push(BallNumber::new(ball_value));
        }
    }
    
    Ok(ticket)
}

/// Generates a lottery ticket using the optimal bitwise strategy.
///
/// Automatically selects the best bitmap implementation based on the range size:
/// - u64 for ranges ≤ 64
/// - u128 for ranges ≤ 128
/// - Vec<u64> for ranges ≤ 512
///
/// # Arguments
///
/// * `range` - The range of ball numbers
/// * `count` - Number of balls to pick
/// * `rng` - Random number generator
///
/// # Returns
///
/// A vector of unique ball numbers or an error
///
/// # Examples
///
/// ```
/// use lotto_quick_pick::ticket_bitwise::generate_ticket_bitwise;
/// use lotto_quick_pick::newtypes::{BallRange, PickCount};
/// use rand::rng;
///
/// let range = BallRange::mega_sena();
/// let count = PickCount::new(6, &range).unwrap();
/// let mut rng = rand::rng();
///
/// let ticket = generate_ticket_bitwise(&range, count, &mut rng).unwrap();
/// assert_eq!(ticket.len(), 6);
/// ```
pub fn generate_ticket_bitwise(
    range: &BallRange,
    count: PickCount,
    rng: &mut dyn RandomNumberGenerator,
) -> Result<Vec<BallNumber>, LottoError> {
    let strategy = BitwiseStrategy::select(range)?;
    
    match strategy {
        BitwiseStrategy::U64 => generate_ticket_u64_bitmap(range, count, rng),
        BitwiseStrategy::U128 => generate_ticket_u128_bitmap(range, count, rng),
        BitwiseStrategy::VecU64 => generate_ticket_vec_bitmap(range, count, rng),
    }
}

/// Generates a lottery ticket using bitwise operations with generic RNG.
///
/// This is a convenience wrapper that works with any type implementing
/// the `RandomNumberGenerator` trait, making it compatible with the main
/// ticket generation system.
///
/// # Arguments
///
/// * `range` - The range of ball numbers
/// * `count` - Number of balls to pick
/// * `rng` - Any random number generator implementing RandomNumberGenerator
///
/// # Returns
///
/// A vector of unique ball numbers or an error
pub fn generate_ticket_bitwise_generic<R: RandomNumberGenerator>(
    range: &BallRange,
    count: PickCount,
    rng: &mut R,
) -> Result<Vec<BallNumber>, LottoError> {
    generate_ticket_bitwise(range, count, rng)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::newtypes::BallNumber;

    #[test]
    fn test_strategy_selection_u64() {
        let range = BallRange::mega_sena(); // 1-60
        let strategy = BitwiseStrategy::select(&range).unwrap();
        assert_eq!(strategy, BitwiseStrategy::U64);
    }

    #[test]
    fn test_strategy_selection_u128() {
        let range = BallRange::lotomania(); // 0-99
        let strategy = BitwiseStrategy::select(&range).unwrap();
        assert_eq!(strategy, BitwiseStrategy::U128);
    }

    #[test]
    fn test_strategy_selection_vec_u64() {
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(200)).unwrap();
        let strategy = BitwiseStrategy::select(&range).unwrap();
        assert_eq!(strategy, BitwiseStrategy::VecU64);
    }

    #[test]
    fn test_strategy_selection_max_u8() {
        // Test with max u8 value (255)
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(255)).unwrap();
        let strategy = BitwiseStrategy::select(&range);
        assert!(strategy.is_ok());
        assert_eq!(strategy.unwrap(), BitwiseStrategy::VecU64);
    }

    #[test]
    fn test_u64_bitmap_mega_sena() {
        let range = BallRange::mega_sena();
        let count = PickCount::new(6, &range).unwrap();
        let mut rng = rand::rng();

        let ticket = generate_ticket_u64_bitmap(&range, count, &mut rng).unwrap();
        
        assert_eq!(ticket.len(), 6);
        
        // Check all numbers are unique
        for i in 0..ticket.len() {
            for j in (i + 1)..ticket.len() {
                assert_ne!(ticket[i], ticket[j]);
            }
        }
        
        // Check all numbers are in range
        for ball in &ticket {
            assert!(range.contains(*ball));
        }
    }

    #[test]
    fn test_u128_bitmap_lotomania() {
        let range = BallRange::lotomania();
        let count = PickCount::new(50, &range).unwrap();
        let mut rng = rand::rng();

        let ticket = generate_ticket_u128_bitmap(&range, count, &mut rng).unwrap();
        
        assert_eq!(ticket.len(), 50);
        
        // Check all numbers are unique
        for i in 0..ticket.len() {
            for j in (i + 1)..ticket.len() {
                assert_ne!(ticket[i], ticket[j]);
            }
        }
        
        // Check all numbers are in range
        for ball in &ticket {
            assert!(range.contains(*ball));
        }
    }

    #[test]
    fn test_vec_bitmap_large_range() {
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(200)).unwrap();
        let count = PickCount::new(10, &range).unwrap();
        let mut rng = rand::rng();

        let ticket = generate_ticket_vec_bitmap(&range, count, &mut rng).unwrap();
        
        assert_eq!(ticket.len(), 10);
        
        // Check all numbers are unique
        for i in 0..ticket.len() {
            for j in (i + 1)..ticket.len() {
                assert_ne!(ticket[i], ticket[j]);
            }
        }
        
        // Check all numbers are in range
        for ball in &ticket {
            assert!(range.contains(*ball));
        }
    }

    #[test]
    fn test_generate_ticket_bitwise_auto_selection() {
        let mut rng = rand::rng();
        
        // Test u64 strategy
        let range1 = BallRange::mega_sena();
        let ticket1 = generate_ticket_bitwise(&range1, PickCount::new(6, &range1).unwrap(), &mut rng).unwrap();
        assert_eq!(ticket1.len(), 6);
        
        // Test u128 strategy
        let range2 = BallRange::lotomania();
        let ticket2 = generate_ticket_bitwise(&range2, PickCount::new(50, &range2).unwrap(), &mut rng).unwrap();
        assert_eq!(ticket2.len(), 50);
        
        // Test Vec<u64> strategy
        let range3 = BallRange::new(BallNumber::new(1), BallNumber::new(200)).unwrap();
        let ticket3 = generate_ticket_bitwise(&range3, PickCount::new(10, &range3).unwrap(), &mut rng).unwrap();
        assert_eq!(ticket3.len(), 10);
    }

    #[test]
    fn test_u64_bitmap_invalid_range() {
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(100)).unwrap();
        let count = PickCount::new(5, &range).unwrap();
        let mut rng = rand::rng();

        let result = generate_ticket_u64_bitmap(&range, count, &mut rng);
        assert!(result.is_err());
    }

    #[test]
    fn test_u128_bitmap_invalid_range() {
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(200)).unwrap();
        let count = PickCount::new(5, &range).unwrap();
        let mut rng = rand::rng();

        let result = generate_ticket_u128_bitmap(&range, count, &mut rng);
        assert!(result.is_err());
    }
}
