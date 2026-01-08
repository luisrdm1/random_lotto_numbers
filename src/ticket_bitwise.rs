//! Bitwise/bitmask optimization strategies for lottery ticket generation.
//!
//! This module implements high-performance ticket generation using bitwise operations
//! for duplicate checking. Benchmarks show 55-67% performance improvement over HashSet.
//!
//! # Strategies
//!
//! Selection is based on **range size** (number of possible values), not max value:
//!
//! - **u64 bitmap**: For ranges with ≤ 64 values
//!   - Example: Mega-Sena (1-60) has 60 values → uses u64
//!   - Example: Range 200-255 has 56 values → uses u64
//! - **u128 bitmap**: For ranges with 65-128 values
//!   - Example: Lotomania (0-99) has 100 values → uses u128
//! - **Vec<u64> bitmap**: For ranges with > 128 values
//!   - Example: Range 0-255 has 256 values → uses Vec<u64>
//!
//! # Performance
//!
//! - Zero-cost abstraction: Generic functions enable monomorphization (no vtable)
//! - O(1) duplicate checking with better cache locality than HashSet
//! - No hashing overhead
//! - Measured gains: 55-67% faster than HashSet for typical lottery ranges

use crate::error::LottoError;
use crate::newtypes::{BallNumber, BallRange, PickCount};
use crate::rng::RandomNumberGenerator;
use crate::ticket_key::TicketKey;
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
        let size = range.size();

        if size <= 64 {
            Ok(Self::U64)
        } else if size <= 128 {
            Ok(Self::U128)
        } else {
            // For ranges larger than 128 values
            Ok(Self::VecU64)
        }
    }

    /// Generates a TicketKey using this strategy.
    ///
    /// # Arguments
    ///
    /// * `range` - The range of ball numbers
    /// * `count` - Number of balls to pick
    /// * `rng` - Random number generator
    ///
    /// # Returns
    ///
    /// A TicketKey generated using the selected bitmap strategy
    pub fn generate<R: RandomNumberGenerator>(
        self,
        range: &BallRange,
        count: PickCount,
        rng: &mut R,
    ) -> Result<TicketKey, LottoError> {
        match self {
            Self::U64 => generate_ticketkey_u64_bitmap(range, count, rng),
            Self::U128 => generate_ticketkey_u128_bitmap(range, count, rng),
            Self::VecU64 => generate_ticketkey_vec_bitmap(range, count, rng),
        }
    }
}

/// Generates a lottery ticket using u64 bitmap for duplicate checking.
///
/// **DEPRECATED**: Use [`generate_ticketkey_u64_bitmap`] instead for better performance.
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
#[deprecated(
    since = "1.2.0",
    note = "Use generate_ticketkey_u64_bitmap() instead for better performance"
)]
pub fn generate_ticket_u64_bitmap<R: RandomNumberGenerator>(
    range: &BallRange,
    count: PickCount,
    rng: &mut R,
) -> Result<Vec<BallNumber>, LottoError> {
    let min = range.start().value();
    let max = range.end().value();
    let picks = count.value();

    // Validate range fits in u64 (by size, not by max value)
    if range.size() > 64 {
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
/// **DEPRECATED**: Use [`generate_ticketkey_u128_bitmap`] instead for better performance.
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
#[deprecated(
    since = "1.2.0",
    note = "Use generate_ticketkey_u128_bitmap() instead for better performance"
)]
pub fn generate_ticket_u128_bitmap<R: RandomNumberGenerator>(
    range: &BallRange,
    count: PickCount,
    rng: &mut R,
) -> Result<Vec<BallNumber>, LottoError> {
    let min = range.start().value();
    let max = range.end().value();
    let picks = count.value();

    // Validate range fits in u128 (by size, not by max value)
    if range.size() > 128 {
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
#[deprecated(
    since = "1.2.0",
    note = "Use generate_ticketkey_vec_bitmap() instead for better performance"
)]
pub fn generate_ticket_vec_bitmap<R: RandomNumberGenerator>(
    range: &BallRange,
    count: PickCount,
    rng: &mut R,
) -> Result<Vec<BallNumber>, LottoError> {
    let min = range.start().value();
    let max = range.end().value();
    let picks = count.value();

    // Validate range fits in Vec<u64> (u8 max is 255, so always fits)
    // Convert to larger types to avoid overflow
    let range_size = (max as usize) - (min as usize) + 1;
    let vec_size = range_size.div_ceil(64);
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
#[deprecated(
    since = "1.2.0",
    note = "Use generate_ticketkey_bitwise() instead for better performance"
)]
#[allow(deprecated)]
pub fn generate_ticket_bitwise<R: RandomNumberGenerator>(
    range: &BallRange,
    count: PickCount,
    rng: &mut R,
) -> Result<Vec<BallNumber>, LottoError> {
    let strategy = BitwiseStrategy::select(range)?;

    match strategy {
        BitwiseStrategy::U64 => generate_ticket_u64_bitmap(range, count, rng),
        BitwiseStrategy::U128 => generate_ticket_u128_bitmap(range, count, rng),
        BitwiseStrategy::VecU64 => generate_ticket_vec_bitmap(range, count, rng),
    }
}

/// Generates a lottery ticket using u64 bitmap, returning TicketKey directly.
///
/// This is the optimized version that avoids creating intermediate Vec<BallNumber>.
/// Use this when you need TicketKey for HashSet-based uniqueness checking.
///
/// # Arguments
///
/// * `range` - The range of ball numbers (must have size ≤ 64)
/// * `count` - Number of balls to pick
/// * `rng` - Random number generator
///
/// # Returns
///
/// A TicketKey::U64 containing the bitmap
pub fn generate_ticketkey_u64_bitmap<R: RandomNumberGenerator>(
    range: &BallRange,
    count: PickCount,
    rng: &mut R,
) -> Result<TicketKey, LottoError> {
    let min = range.start().value();
    let max = range.end().value();
    let picks = count.value();

    // Validate range fits in u64 (by size, not by max value)
    if range.size() > 64 {
        return Err(LottoError::InvalidRange {
            start: min,
            end: max,
        });
    }

    let mut bitmap: u64 = 0;
    let mut picked_count = 0;

    while picked_count < picks {
        let ball_value = rng.gen_range_u8(min, max);
        let bit_position = ball_value - min;
        let bit_mask = 1u64 << bit_position;

        if bitmap & bit_mask == 0 {
            bitmap |= bit_mask;
            picked_count += 1;
        }
    }

    // Validate invariants
    let actual_count = bitmap.count_ones() as usize;
    debug_assert_eq!(
        actual_count, picks,
        "bitmap should have exactly {} bits set, got {}",
        picks, actual_count
    );

    // Validate no bits outside range
    let valid_mask = if range.size() == 64 {
        u64::MAX
    } else {
        (1u64 << range.size()) - 1
    };
    debug_assert_eq!(
        bitmap & !valid_mask,
        0,
        "bitmap 0x{:016X} has bits set outside valid range (mask: 0x{:016X})",
        bitmap,
        valid_mask
    );

    Ok(TicketKey::U64(bitmap))
}

/// Generates a lottery ticket using u128 bitmap, returning TicketKey directly.
///
/// This is the optimized version that avoids creating intermediate Vec<BallNumber>.
/// Use this when you need TicketKey for HashSet-based uniqueness checking.
///
/// # Arguments
///
/// * `range` - The range of ball numbers (must have size ≤ 128)
/// * `count` - Number of balls to pick
/// * `rng` - Random number generator
///
/// # Returns
///
/// A TicketKey::U128 containing the bitmap
pub fn generate_ticketkey_u128_bitmap<R: RandomNumberGenerator>(
    range: &BallRange,
    count: PickCount,
    rng: &mut R,
) -> Result<TicketKey, LottoError> {
    let min = range.start().value();
    let max = range.end().value();
    let picks = count.value();

    // Validate range fits in u128
    if range.size() > 128 {
        return Err(LottoError::InvalidRange {
            start: min,
            end: max,
        });
    }

    let mut bitmap: u128 = 0;
    let mut picked_count = 0;

    while picked_count < picks {
        let ball_value = rng.gen_range_u8(min, max);
        let bit_position = ball_value - min;
        let bit_mask = 1u128 << bit_position;

        if bitmap & bit_mask == 0 {
            bitmap |= bit_mask;
            picked_count += 1;
        }
    }

    // Validate invariants
    let actual_count = bitmap.count_ones() as usize;
    debug_assert_eq!(
        actual_count, picks,
        "bitmap should have exactly {} bits set, got {}",
        picks, actual_count
    );

    // Validate no bits outside range
    let valid_mask = if range.size() == 128 {
        u128::MAX
    } else {
        (1u128 << range.size()) - 1
    };
    debug_assert_eq!(
        bitmap & !valid_mask,
        0,
        "bitmap 0x{:032X} has bits set outside valid range (mask: 0x{:032X})",
        bitmap,
        valid_mask
    );

    Ok(TicketKey::U128(bitmap))
}

/// Generates a lottery ticket using Vec<u64> bitmap, returning TicketKey directly.
///
/// This is the optimized version that avoids creating intermediate Vec<BallNumber>.
/// Use this when you need TicketKey for HashSet-based uniqueness checking.
///
/// # Arguments
///
/// * `range` - The range of ball numbers (any size)
/// * `count` - Number of balls to pick
/// * `rng` - Random number generator
///
/// # Returns
///
/// A TicketKey::VecU64 containing the bitmap
pub fn generate_ticketkey_vec_bitmap<R: RandomNumberGenerator>(
    range: &BallRange,
    count: PickCount,
    rng: &mut R,
) -> Result<TicketKey, LottoError> {
    let min = range.start().value();
    let max = range.end().value();
    let picks = count.value();
    let range_size = range.size();

    // Calculate bitmap size
    let words_needed = range_size.div_ceil(64);
    let mut bitmap: Vec<u64> = vec![0; words_needed];
    let mut picked_count = 0;

    while picked_count < picks {
        let ball_value = rng.gen_range_u8(min, max);
        let bit_position = (ball_value - min) as usize;
        let word_index = bit_position / 64;
        let bit_offset = bit_position % 64;
        let bit_mask = 1u64 << bit_offset;

        if bitmap[word_index] & bit_mask == 0 {
            bitmap[word_index] |= bit_mask;
            picked_count += 1;
        }
    }

    // Validate invariants
    let actual_count: usize = bitmap.iter().map(|w| w.count_ones() as usize).sum();
    debug_assert_eq!(
        actual_count, picks,
        "bitmap should have exactly {} bits set, got {}",
        picks, actual_count
    );

    // Validate no bits outside range in last word
    let remaining_bits = range_size % 64;
    if remaining_bits > 0 {
        let last_word = bitmap[words_needed - 1];
        let last_mask = (1u64 << remaining_bits) - 1;
        debug_assert_eq!(
            last_word & !last_mask,
            0,
            "bitmap has bits set outside valid range in last word"
        );
    }

    Ok(TicketKey::VecU64(bitmap))
}

/// Unified wrapper that generates TicketKey using optimal bitwise strategy.
///
/// Automatically selects U64, U128, or VecU64 based on range size.
/// This is the preferred function for generating tickets with TicketKey.
///
/// # Example
///
/// ```
/// use lotto_quick_pick::ticket_bitwise::generate_ticketkey_bitwise;
/// use lotto_quick_pick::newtypes::{BallRange, PickCount};
/// use rand::rng;
///
/// let range = BallRange::mega_sena();
/// let count = PickCount::new(6, &range).unwrap();
/// let mut rng = rand::rng();
///
/// let key = generate_ticketkey_bitwise(&range, count, &mut rng).unwrap();
/// let balls = key.to_balls(&range);
/// assert_eq!(balls.len(), 6);
/// ```
pub fn generate_ticketkey_bitwise<R: RandomNumberGenerator>(
    range: &BallRange,
    count: PickCount,
    rng: &mut R,
) -> Result<TicketKey, LottoError> {
    let strategy = BitwiseStrategy::select(range)?;

    match strategy {
        BitwiseStrategy::U64 => generate_ticketkey_u64_bitmap(range, count, rng),
        BitwiseStrategy::U128 => generate_ticketkey_u128_bitmap(range, count, rng),
        BitwiseStrategy::VecU64 => generate_ticketkey_vec_bitmap(range, count, rng),
    }
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

        let key = generate_ticketkey_u64_bitmap(&range, count, &mut rng).unwrap();
        let ticket = key.to_balls(&range);

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

        let key = generate_ticketkey_u128_bitmap(&range, count, &mut rng).unwrap();
        let ticket = key.to_balls(&range);

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

        let key = generate_ticketkey_vec_bitmap(&range, count, &mut rng).unwrap();
        let ticket = key.to_balls(&range);

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
        let key1 =
            generate_ticketkey_bitwise(&range1, PickCount::new(6, &range1).unwrap(), &mut rng)
                .unwrap();
        assert_eq!(key1.count_balls(), 6);

        // Test u128 strategy
        let range2 = BallRange::lotomania();
        let key2 =
            generate_ticketkey_bitwise(&range2, PickCount::new(50, &range2).unwrap(), &mut rng)
                .unwrap();
        assert_eq!(key2.count_balls(), 50);

        // Test Vec<u64> strategy
        let range3 = BallRange::new(BallNumber::new(1), BallNumber::new(200)).unwrap();
        let key3 =
            generate_ticketkey_bitwise(&range3, PickCount::new(10, &range3).unwrap(), &mut rng)
                .unwrap();
        assert_eq!(key3.count_balls(), 10);
    }

    #[test]
    fn test_u64_bitmap_invalid_range() {
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(100)).unwrap();
        let count = PickCount::new(5, &range).unwrap();
        let mut rng = rand::rng();

        let result = generate_ticketkey_u64_bitmap(&range, count, &mut rng);
        assert!(result.is_err());
    }

    #[test]
    fn test_u128_bitmap_invalid_range() {
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(200)).unwrap();
        let count = PickCount::new(5, &range).unwrap();
        let mut rng = rand::rng();

        let result = generate_ticketkey_u128_bitmap(&range, count, &mut rng);
        assert!(result.is_err());
    }

    // ============================================================================
    // Tests exposing bugs (Commit 1)
    // ============================================================================

    #[test]
    fn test_bug_range_large_value_small_size() {
        // Bug: Range 200..=255 has size 56, but end()=255
        // Expected: BitwiseStrategy::U64 (because size=56 <= 64)
        // Current (buggy): VecU64 (because end=255 > 128)
        let range = BallRange::new(BallNumber::new(200), BallNumber::new(255)).unwrap();
        assert_eq!(range.size(), 56);

        let strategy = BitwiseStrategy::select(&range).unwrap();
        assert_eq!(
            strategy,
            BitwiseStrategy::U64,
            "Range 200-255 (size=56) should use U64, not {:?}",
            strategy
        );
    }

    #[test]
    fn test_bug_range_0_to_64_should_not_be_u64() {
        // Critical bug: Range 0..=64 has size 65
        // Expected: U128 or VecU64 (because size=65 > 64, avoids 1u64 << 64)
        // Current (buggy): U64 (because end=64 <= 64) → causes panic in shift
        let range = BallRange::new(BallNumber::new(0), BallNumber::new(64)).unwrap();
        assert_eq!(range.size(), 65);

        let strategy = BitwiseStrategy::select(&range).unwrap();
        assert_ne!(
            strategy,
            BitwiseStrategy::U64,
            "Range 0-64 (size=65) CANNOT use U64 (would cause shift overflow)"
        );
    }

    #[test]
    fn test_u64_precondition_validation_by_size() {
        // Must reject range with size > 64, even if end <= 64
        let range = BallRange::new(BallNumber::new(0), BallNumber::new(64)).unwrap();
        assert_eq!(range.size(), 65);

        let count = PickCount::new(5, &range).unwrap();
        let mut rng = rand::rng();

        let result = generate_ticketkey_u64_bitmap(&range, count, &mut rng);
        assert!(
            result.is_err(),
            "u64_bitmap must reject range with size=65 (> 64)"
        );
    }

    #[test]
    fn test_u128_precondition_validation_by_size() {
        // Must reject range with size > 128, even if end <= 128
        let range = BallRange::new(BallNumber::new(0), BallNumber::new(128)).unwrap();
        assert_eq!(range.size(), 129);

        let count = PickCount::new(5, &range).unwrap();
        let mut rng = rand::rng();

        let result = generate_ticketkey_u128_bitmap(&range, count, &mut rng);
        assert!(
            result.is_err(),
            "u128_bitmap must reject range with size=129 (> 128)"
        );
    }

    #[test]
    fn test_range_1_to_64_should_use_u64() {
        // Valid case: Range 1..=64 has size 64 (OK for U64)
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(64)).unwrap();
        assert_eq!(range.size(), 64);

        let strategy = BitwiseStrategy::select(&range).unwrap();
        assert_eq!(
            strategy,
            BitwiseStrategy::U64,
            "Range 1-64 (size=64) should use U64"
        );
    }

    #[test]
    fn test_range_1_to_65_should_not_use_u64() {
        // Range 1..=65 has size 65 (too large for U64)
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(65)).unwrap();
        assert_eq!(range.size(), 65);

        let strategy = BitwiseStrategy::select(&range).unwrap();
        assert_ne!(
            strategy,
            BitwiseStrategy::U64,
            "Range 1-65 (size=65) CANNOT use U64"
        );
    }

    // Tests for TicketKey generation functions

    #[test]
    fn test_ticketkey_u64_validates_bit_count() {
        let range = BallRange::mega_sena();
        let pick = PickCount::new(6, &range).unwrap();
        let mut rng = rand::rng();

        let key = generate_ticketkey_u64_bitmap(&range, pick, &mut rng).unwrap();
        assert_eq!(key.count_balls(), 6);
    }

    #[test]
    fn test_ticketkey_u64_rejects_size_65() {
        let range = BallRange::new(BallNumber::new(0), BallNumber::new(64)).unwrap();
        assert_eq!(range.size(), 65);

        let count = PickCount::new(5, &range).unwrap();
        let mut rng = rand::rng();

        let result = generate_ticketkey_u64_bitmap(&range, count, &mut rng);
        assert!(result.is_err(), "Should reject range size > 64");
    }

    #[test]
    fn test_ticketkey_u64_max_range_boundary() {
        let range = BallRange::new(BallNumber::new(0), BallNumber::new(63)).unwrap();
        assert_eq!(range.size(), 64);

        let count = PickCount::new(10, &range).unwrap();
        let mut rng = rand::rng();

        let key = generate_ticketkey_u64_bitmap(&range, count, &mut rng).unwrap();
        assert_eq!(key.count_balls(), 10);
    }

    #[test]
    fn test_ticketkey_u128_validates_bit_count() {
        let range = BallRange::lotomania();
        let pick = PickCount::new(50, &range).unwrap();
        let mut rng = rand::rng();

        let key = generate_ticketkey_u128_bitmap(&range, pick, &mut rng).unwrap();
        assert_eq!(key.count_balls(), 50);
    }

    #[test]
    fn test_ticketkey_u128_max_range_boundary() {
        let range = BallRange::new(BallNumber::new(0), BallNumber::new(127)).unwrap();
        assert_eq!(range.size(), 128);

        let count = PickCount::new(20, &range).unwrap();
        let mut rng = rand::rng();

        let key = generate_ticketkey_u128_bitmap(&range, count, &mut rng).unwrap();
        assert_eq!(key.count_balls(), 20);
    }

    #[test]
    fn test_ticketkey_vec_validates_bit_count() {
        let range = BallRange::new(BallNumber::new(0), BallNumber::new(200)).unwrap();
        let pick = PickCount::new(15, &range).unwrap();
        let mut rng = rand::rng();

        let key = generate_ticketkey_vec_bitmap(&range, pick, &mut rng).unwrap();
        assert_eq!(key.count_balls(), 15);
    }

    #[test]
    fn test_ticketkey_roundtrip_preserves_all_balls() {
        let range = BallRange::mega_sena();
        let pick = PickCount::new(6, &range).unwrap();
        let mut rng = rand::rng();

        let key = generate_ticketkey_bitwise(&range, pick, &mut rng).unwrap();
        let balls = key.to_balls(&range);

        assert_eq!(balls.len(), pick.value());
        for &ball in &balls {
            assert!(range.contains(ball), "Ball {:?} is outside range", ball);
        }
    }

    #[test]
    fn test_ticketkey_no_duplicates() {
        let range = BallRange::mega_sena();
        let pick = PickCount::new(6, &range).unwrap();
        let mut rng = rand::rng();

        let key = generate_ticketkey_bitwise(&range, pick, &mut rng).unwrap();
        let balls = key.to_balls(&range);

        let mut seen = std::collections::HashSet::new();
        for ball in balls {
            assert!(seen.insert(ball), "Duplicate ball: {:?}", ball);
        }
    }

    #[test]
    fn test_ticketkey_single_value_range() {
        let range = BallRange::new(BallNumber::new(42), BallNumber::new(43)).unwrap();
        assert_eq!(range.size(), 2);

        let count = PickCount::new(1, &range).unwrap();
        let mut rng = rand::rng();

        let key = generate_ticketkey_bitwise(&range, count, &mut rng).unwrap();
        let balls = key.to_balls(&range);

        assert_eq!(balls.len(), 1);
        assert!(balls[0].value() == 42 || balls[0].value() == 43);
    }

    #[test]
    fn test_ticketkey_full_range_selection() {
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(10)).unwrap();
        let count = PickCount::new(10, &range).unwrap();
        let mut rng = rand::rng();

        let key = generate_ticketkey_bitwise(&range, count, &mut rng).unwrap();
        let balls = key.to_balls(&range);

        assert_eq!(balls.len(), 10);
        let mut sorted = balls.clone();
        sorted.sort_by_key(|b| b.value());
        for (i, &ball) in sorted.iter().enumerate() {
            assert_eq!(ball.value(), (i + 1) as u8);
        }
    }
}
