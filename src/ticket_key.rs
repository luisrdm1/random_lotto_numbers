//! Bitmap-based ticket representation for efficient uniqueness checking.
//!
//! This module provides a compact representation of lottery tickets using bitmaps,
//! which is more efficient than using full `Vec<BallNumber>` in `HashSet` for
//! checking uniqueness when generating multiple tickets.
//!
//! # Performance
//!
//! - **Smaller memory footprint**: u64/u128 vs Vec allocations
//! - **Faster hashing**: Direct integer hash vs hashing Vec contents
//! - **Better cache locality**: Contiguous bits vs scattered heap allocations

use crate::newtypes::{BallNumber, BallRange};
use std::hash::{Hash, Hasher};

/// Compact bitmap representation of a lottery ticket.
///
/// Uses the most efficient storage based on range size:
/// - `U64`: For ranges with ≤ 64 values
/// - `U128`: For ranges with 65-128 values  
/// - `VecU64`: For ranges with > 128 values
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TicketKey {
    /// Bitmap using single u64 (up to 64 values)
    U64(u64),
    /// Bitmap using single u128 (up to 128 values)
    U128(u128),
    /// Bitmap using Vec<u64> for larger ranges
    VecU64(Vec<u64>),
}

impl TicketKey {
    /// Create a TicketKey from ball numbers and range.
    ///
    /// # Arguments
    ///
    /// * `balls` - Sorted slice of ball numbers
    /// * `range` - The valid range for balls
    ///
    /// # Panics
    ///
    /// Panics if any ball is outside the range (should be validated beforehand)
    pub fn from_balls(balls: &[BallNumber], range: &BallRange) -> Self {
        let size = range.size();
        let min = range.start().value();

        if size <= 64 {
            let mut bitmap = 0u64;
            for &ball in balls {
                let offset = ball.value() - min;
                bitmap |= 1u64 << offset;
            }
            TicketKey::U64(bitmap)
        } else if size <= 128 {
            let mut bitmap = 0u128;
            for &ball in balls {
                let offset = ball.value() - min;
                bitmap |= 1u128 << offset;
            }
            TicketKey::U128(bitmap)
        } else {
            let vec_size = size.div_ceil(64);
            let mut bitmap = vec![0u64; vec_size];
            for &ball in balls {
                let offset = (ball.value() - min) as usize;
                let idx = offset / 64;
                let bit = offset % 64;
                bitmap[idx] |= 1u64 << bit;
            }
            TicketKey::VecU64(bitmap)
        }
    }

    /// Convert TicketKey back to Vec<BallNumber>.
    ///
    /// # Arguments
    ///
    /// * `range` - The valid range (needed to calculate absolute ball values)
    pub fn to_balls(&self, range: &BallRange) -> Vec<BallNumber> {
        let min = range.start().value();
        let mut balls = Vec::new();

        match self {
            TicketKey::U64(bitmap) => {
                for i in 0..64 {
                    if bitmap & (1u64 << i) != 0 {
                        balls.push(BallNumber::new(min + i as u8));
                    }
                }
            }
            TicketKey::U128(bitmap) => {
                for i in 0..128 {
                    if bitmap & (1u128 << i) != 0 {
                        balls.push(BallNumber::new(min + i as u8));
                    }
                }
            }
            TicketKey::VecU64(bitmap) => {
                for (idx, &word) in bitmap.iter().enumerate() {
                    for bit in 0..64 {
                        if word & (1u64 << bit) != 0 {
                            let offset = idx * 64 + bit;
                            balls.push(BallNumber::new(min + offset as u8));
                        }
                    }
                }
            }
        }

        balls
    }

    /// Count the number of set bits (balls in the ticket).
    pub fn count_balls(&self) -> usize {
        match self {
            TicketKey::U64(bitmap) => bitmap.count_ones() as usize,
            TicketKey::U128(bitmap) => bitmap.count_ones() as usize,
            TicketKey::VecU64(bitmap) => bitmap.iter().map(|w| w.count_ones() as usize).sum(),
        }
    }
}

impl Hash for TicketKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            TicketKey::U64(bitmap) => {
                0u8.hash(state); // Discriminant
                bitmap.hash(state);
            }
            TicketKey::U128(bitmap) => {
                1u8.hash(state); // Discriminant
                bitmap.hash(state);
            }
            TicketKey::VecU64(bitmap) => {
                2u8.hash(state); // Discriminant
                bitmap.hash(state);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::newtypes::BallRange;

    #[test]
    fn test_ticket_key_u64_round_trip() {
        let range = BallRange::mega_sena(); // 1-60
        let balls = vec![
            BallNumber::new(5),
            BallNumber::new(10),
            BallNumber::new(15),
            BallNumber::new(20),
            BallNumber::new(25),
            BallNumber::new(30),
        ];

        let key = TicketKey::from_balls(&balls, &range);
        assert!(matches!(key, TicketKey::U64(_)));

        let recovered = key.to_balls(&range);
        assert_eq!(recovered, balls);
    }

    #[test]
    fn test_ticket_key_u128_round_trip() {
        let range = BallRange::lotomania(); // 0-99
        let balls = vec![
            BallNumber::new(0),
            BallNumber::new(10),
            BallNumber::new(20),
            BallNumber::new(30),
            BallNumber::new(40),
            BallNumber::new(50),
        ];

        let key = TicketKey::from_balls(&balls, &range);
        assert!(matches!(key, TicketKey::U128(_)));

        let recovered = key.to_balls(&range);
        assert_eq!(recovered, balls);
    }

    #[test]
    fn test_ticket_key_vec_u64_round_trip() {
        let range = BallRange::new(BallNumber::new(0), BallNumber::new(200)).unwrap();
        let balls = vec![
            BallNumber::new(0),
            BallNumber::new(50),
            BallNumber::new(100),
            BallNumber::new(150),
            BallNumber::new(200),
        ];

        let key = TicketKey::from_balls(&balls, &range);
        assert!(matches!(key, TicketKey::VecU64(_)));

        let recovered = key.to_balls(&range);
        assert_eq!(recovered, balls);
    }

    #[test]
    fn test_ticket_key_count_balls() {
        let range = BallRange::mega_sena();
        let balls = vec![
            BallNumber::new(5),
            BallNumber::new(10),
            BallNumber::new(15),
            BallNumber::new(20),
            BallNumber::new(25),
            BallNumber::new(30),
        ];

        let key = TicketKey::from_balls(&balls, &range);
        assert_eq!(key.count_balls(), 6);
    }

    #[test]
    fn test_ticket_key_equality() {
        let range = BallRange::mega_sena();
        let balls1 = vec![BallNumber::new(5), BallNumber::new(10), BallNumber::new(15)];
        let balls2 = vec![BallNumber::new(5), BallNumber::new(10), BallNumber::new(15)];
        let balls3 = vec![BallNumber::new(5), BallNumber::new(10), BallNumber::new(20)];

        let key1 = TicketKey::from_balls(&balls1, &range);
        let key2 = TicketKey::from_balls(&balls2, &range);
        let key3 = TicketKey::from_balls(&balls3, &range);

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_ticket_key_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let range = BallRange::mega_sena();
        let balls = vec![BallNumber::new(5), BallNumber::new(10), BallNumber::new(15)];

        let key1 = TicketKey::from_balls(&balls, &range);
        let key2 = TicketKey::from_balls(&balls, &range);

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        key1.hash(&mut hasher1);
        key2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_ticket_key_preserves_all_bits() {
        let range = BallRange::mega_sena();
        // Test edge cases: first, last, and middle values
        let balls = vec![
            BallNumber::new(1),  // First
            BallNumber::new(30), // Middle
            BallNumber::new(60), // Last
        ];

        let key = TicketKey::from_balls(&balls, &range);
        let recovered = key.to_balls(&range);

        assert_eq!(recovered.len(), 3);
        assert_eq!(recovered[0].value(), 1);
        assert_eq!(recovered[1].value(), 30);
        assert_eq!(recovered[2].value(), 60);
    }
}
