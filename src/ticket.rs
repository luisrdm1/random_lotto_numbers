//! Lottery ticket generation module.
//!
//! This module provides functionality for generating unique lottery tickets
//! using a pluggable random number generator.

use crate::newtypes::{BallNumber, BallRange, GameCount, PickCount, Ticket};
use crate::rng::RandomNumberGenerator;
use std::collections::HashSet;

/// Generate a single lottery ticket with unique random ball numbers.
///
/// # Arguments
///
/// * `rng` - Random number generator implementing the RandomNumberGenerator trait
/// * `range` - The range of ball numbers to choose from
/// * `pick` - The number of balls to pick
///
/// # Returns
///
/// A Ticket containing unique, sorted ball numbers.
///
/// # Examples
///
/// ```
/// use rand::rng;
/// use lotto_quick_pick::ticket::generate_ticket;
/// use lotto_quick_pick::newtypes::{BallRange, BallNumber, PickCount};
///
/// let mut rng = rand::rng();
/// let range = BallRange::new(BallNumber::new(1), BallNumber::new(60)).unwrap();
/// let pick = PickCount::new(6, &range).unwrap();
/// let ticket = generate_ticket(&mut rng, &range, &pick);
///
/// assert_eq!(ticket.balls().len(), 6);
/// ```
pub fn generate_ticket<R: RandomNumberGenerator>(
    rng: &mut R,
    range: &BallRange,
    pick: &PickCount,
) -> Ticket {
    // Try bitwise strategy first (fastest, 55-67% performance improvement)
    if let Ok(key) = crate::ticket_bitwise::generate_ticketkey_bitwise(range, *pick, rng) {
        return Ticket::new(key.to_balls(range));
    }

    // Fallback to HashSet strategy for edge cases or errors
    let range_size = range.size();
    let pick_count = pick.value();

    let balls = if should_use_exclusion_strategy(range_size, pick_count) {
        generate_by_exclusion(rng, range, pick_count)
    } else {
        generate_by_insertion(rng, range, pick_count)
    };

    Ticket::new(balls)
}

/// Determine whether to use exclusion or insertion strategy.
///
/// Uses exclusion (generate all then remove) when picking more than half the range.
/// Uses insertion (add until unique) when picking less than half the range.
fn should_use_exclusion_strategy(range_size: usize, pick_count: usize) -> bool {
    pick_count > range_size / 2
}

/// Generate ticket by excluding random balls from the full range.
///
/// More efficient when picking more than half of available numbers.
fn generate_by_exclusion<R: RandomNumberGenerator>(
    rng: &mut R,
    range: &BallRange,
    pick_count: usize,
) -> Vec<BallNumber> {
    let range_size = range.size();
    let exclude_count = range_size - pick_count;

    let mut excluded = HashSet::with_capacity(exclude_count);

    while excluded.len() < exclude_count {
        let value = rng.gen_range_u8(range.start().value(), range.end().value());
        excluded.insert(value);
    }

    collect_non_excluded_balls(range, &excluded)
}

/// Collect all balls not in the excluded set.
fn collect_non_excluded_balls(range: &BallRange, excluded: &HashSet<u8>) -> Vec<BallNumber> {
    (range.start().value()..=range.end().value())
        .filter(|&value| !excluded.contains(&value))
        .map(BallNumber::new)
        .collect()
}

/// Generate ticket by inserting unique random balls.
///
/// More efficient when picking less than half of available numbers.
fn generate_by_insertion<R: RandomNumberGenerator>(
    rng: &mut R,
    range: &BallRange,
    pick_count: usize,
) -> Vec<BallNumber> {
    let mut selected = HashSet::with_capacity(pick_count);

    while selected.len() < pick_count {
        let value = rng.gen_range_u8(range.start().value(), range.end().value());
        selected.insert(value);
    }

    selected.into_iter().map(BallNumber::new).collect()
}

/// Generate multiple unique lottery tickets.
///
/// Ensures that all generated tickets are unique (no duplicate tickets).
/// Returns an error if the requested number of unique tickets exceeds
/// the mathematically possible combinations, or if generation fails
/// after a reasonable number of attempts.
///
/// **Time complexity:** O(n) average case, O(n * max_attempts) worst case  
/// **Space complexity:** O(n) where n is the number of tickets
///
/// # Arguments
///
/// * `rng` - Random number generator implementing the RandomNumberGenerator trait
/// * `range` - The range of ball numbers to choose from
/// * `pick` - The number of balls to pick per ticket
/// * `game_count` - The number of unique tickets to generate
///
/// # Returns
///
/// A Result containing a vector of unique Ticket instances, or an error.
///
/// # Errors
///
/// Returns `LottoError::TooManyUniqueGames` if the requested number exceeds
/// the maximum possible combinations C(n, k).
///
/// Returns `LottoError::UniqueGenerationFailed` if unable to generate
/// the requested number of unique tickets after many attempts (unlikely
/// unless requesting close to the maximum possible).
///
/// # Examples
///
/// ```
/// use rand::rng;
/// use lotto_quick_pick::ticket::generate_unique_tickets;
/// use lotto_quick_pick::newtypes::{BallRange, BallNumber, PickCount, GameCount};
///
/// let mut rng = rand::rng();
/// let range = BallRange::new(BallNumber::new(1), BallNumber::new(60)).unwrap();
/// let pick = PickCount::new(6, &range).unwrap();
/// let count = GameCount::new(10).unwrap();
///
/// let tickets = generate_unique_tickets(&mut rng, &range, &pick, &count).unwrap();
/// assert_eq!(tickets.len(), 10);
/// ```
pub fn generate_unique_tickets<R: RandomNumberGenerator>(
    rng: &mut R,
    range: &BallRange,
    pick: &PickCount,
    game_count: &GameCount,
) -> crate::error::Result<Vec<Ticket>> {
    use crate::probability::combination;

    // Check if the requested number of unique tickets is mathematically possible
    let max_possible = combination(range.size(), pick.value())?;

    if (game_count.value() as u128) > max_possible {
        return Err(crate::error::LottoError::TooManyUniqueGames {
            requested: game_count.value(),
            maximum: max_possible,
        });
    }

    // Use TicketKey for efficient uniqueness checking (smaller, faster hashing)
    let mut ticket_keys = HashSet::with_capacity(game_count.value());

    // Calculate a reasonable maximum number of attempts
    // For small ratios (requested/possible), this is generous
    // For large ratios (approaching maximum), we need many more attempts
    let ratio = (game_count.value() as f64) / (max_possible as f64);
    let max_attempts = if ratio < 0.5 {
        game_count.value() * 100
    } else if ratio < 0.8 {
        game_count.value() * 1000
    } else {
        game_count.value() * 10000
    };

    let mut attempts = 0;

    // Hoist strategy selection outside the loop (doesn't change per iteration)
    use crate::ticket_bitwise::{
        BitwiseStrategy, generate_ticketkey_u64_bitmap, generate_ticketkey_u128_bitmap,
        generate_ticketkey_vec_bitmap,
    };
    let strategy = BitwiseStrategy::select(range)?;

    while ticket_keys.len() < game_count.value() {
        if attempts >= max_attempts {
            return Err(crate::error::LottoError::UniqueGenerationFailed {
                requested: game_count.value(),
                generated: ticket_keys.len(),
            });
        }

        // Generate TicketKey directly without intermediate Vec<BallNumber>
        let key = match strategy {
            BitwiseStrategy::U64 => generate_ticketkey_u64_bitmap(range, *pick, rng)?,
            BitwiseStrategy::U128 => generate_ticketkey_u128_bitmap(range, *pick, rng)?,
            BitwiseStrategy::VecU64 => generate_ticketkey_vec_bitmap(range, *pick, rng)?,
        };
        ticket_keys.insert(key);
        attempts += 1;
    }

    // Convert TicketKey back to Ticket only at the end
    // Use from_sorted since to_balls() returns pre-sorted Vec
    Ok(ticket_keys
        .into_iter()
        .map(|key| Ticket::from_sorted(key.to_balls(range)))
        .collect())
}

/// Generate multiple unique tickets using HashSet<Ticket> directly (old implementation).
///
/// This function is provided for benchmarking comparison purposes only.
/// The main implementation uses TicketKey for better performance.
///
/// # Performance
///
/// This approach is slower because:
/// - Ticket contains Vec<BallNumber> which requires heap allocation
/// - Hashing a Vec is slower than hashing u64/u128
/// - Worse cache locality due to pointer indirection
#[doc(hidden)]
pub fn generate_unique_tickets_with_ticket_hashset<R: RandomNumberGenerator>(
    rng: &mut R,
    range: &BallRange,
    pick: &PickCount,
    game_count: &GameCount,
) -> crate::error::Result<Vec<Ticket>> {
    use crate::probability::combination;

    let max_possible = combination(range.size(), pick.value())?;

    if (game_count.value() as u128) > max_possible {
        return Err(crate::error::LottoError::TooManyUniqueGames {
            requested: game_count.value(),
            maximum: max_possible,
        });
    }

    // Old implementation: Use HashSet<Ticket> directly
    let mut tickets = HashSet::with_capacity(game_count.value());

    let ratio = (game_count.value() as f64) / (max_possible as f64);
    let max_attempts = if ratio < 0.5 {
        game_count.value() * 100
    } else if ratio < 0.8 {
        game_count.value() * 1000
    } else {
        game_count.value() * 10000
    };

    let mut attempts = 0;

    while tickets.len() < game_count.value() {
        if attempts >= max_attempts {
            return Err(crate::error::LottoError::UniqueGenerationFailed {
                requested: game_count.value(),
                generated: tickets.len(),
            });
        }

        let ticket = generate_ticket(rng, range, pick);
        tickets.insert(ticket);
        attempts += 1;
    }

    Ok(tickets.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock RNG for deterministic testing.
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
    fn test_generate_ticket_has_correct_size() {
        let mut rng = rand::rng();
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(60)).unwrap();
        let pick = PickCount::new(6, &range).unwrap();

        let ticket = generate_ticket(&mut rng, &range, &pick);
        assert_eq!(ticket.balls().len(), 6);
    }

    #[test]
    fn test_generate_ticket_has_unique_balls() {
        let mut rng = rand::rng();
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(60)).unwrap();
        let pick = PickCount::new(6, &range).unwrap();

        let ticket = generate_ticket(&mut rng, &range, &pick);
        let unique: HashSet<_> = ticket.balls().iter().collect();
        assert_eq!(unique.len(), 6);
    }

    #[test]
    fn test_generate_ticket_balls_within_range() {
        let mut rng = rand::rng();
        let range = BallRange::new(BallNumber::new(10), BallNumber::new(20)).unwrap();
        let pick = PickCount::new(5, &range).unwrap();

        let ticket = generate_ticket(&mut rng, &range, &pick);
        for ball in ticket.balls() {
            assert!(ball.value() >= 10 && ball.value() <= 20);
        }
    }

    #[test]
    fn test_generate_ticket_with_mock_rng() {
        let mut rng = MockRng::new(vec![5, 10, 15, 20, 25, 30]);
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(60)).unwrap();
        let pick = PickCount::new(6, &range).unwrap();

        let ticket = generate_ticket(&mut rng, &range, &pick);
        assert_eq!(ticket.balls().len(), 6);
    }

    #[test]
    fn test_should_use_exclusion_strategy() {
        assert!(!should_use_exclusion_strategy(60, 6)); // Pick 10%
        assert!(!should_use_exclusion_strategy(60, 30)); // Pick 50%
        assert!(should_use_exclusion_strategy(60, 31)); // Pick >50%
        assert!(should_use_exclusion_strategy(60, 55)); // Pick most
    }

    #[test]
    fn test_generate_by_insertion_produces_correct_count() {
        let mut rng = rand::rng();
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(60)).unwrap();

        let balls = generate_by_insertion(&mut rng, &range, 10);
        assert_eq!(balls.len(), 10);

        // Check uniqueness
        let unique: HashSet<_> = balls.iter().map(|b| b.value()).collect();
        assert_eq!(unique.len(), 10);
    }

    #[test]
    fn test_generate_by_exclusion_produces_correct_count() {
        let mut rng = rand::rng();
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(20)).unwrap();

        let balls = generate_by_exclusion(&mut rng, &range, 15);
        assert_eq!(balls.len(), 15);

        // Check uniqueness
        let unique: HashSet<_> = balls.iter().map(|b| b.value()).collect();
        assert_eq!(unique.len(), 15);
    }

    #[test]
    fn test_generate_unique_tickets_produces_correct_count() {
        let mut rng = rand::rng();
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(60)).unwrap();
        let pick = PickCount::new(6, &range).unwrap();
        let count = GameCount::new(10).unwrap();

        let tickets = generate_unique_tickets(&mut rng, &range, &pick, &count).unwrap();
        assert_eq!(tickets.len(), 10);
    }

    #[test]
    fn test_generate_unique_tickets_all_unique() {
        let mut rng = rand::rng();
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(60)).unwrap();
        let pick = PickCount::new(6, &range).unwrap();
        let count = GameCount::new(20).unwrap();

        let tickets = generate_unique_tickets(&mut rng, &range, &pick, &count).unwrap();
        let unique_tickets: HashSet<_> = tickets.iter().collect();
        assert_eq!(unique_tickets.len(), 20);
    }

    #[test]
    fn test_generate_unique_tickets_small_range() {
        let mut rng = rand::rng();
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(10)).unwrap();
        let pick = PickCount::new(3, &range).unwrap();
        let count = GameCount::new(5).unwrap();

        let result = generate_unique_tickets(&mut rng, &range, &pick, &count);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 5);
    }

    #[test]
    fn test_generate_unique_tickets_returns_result() {
        let mut rng = rand::rng();
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(60)).unwrap();
        let pick = PickCount::new(6, &range).unwrap();
        let count = GameCount::new(10).unwrap();

        let result = generate_unique_tickets(&mut rng, &range, &pick, &count);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 10);
    }

    #[test]
    fn test_generate_unique_tickets_too_many() {
        let mut rng = rand::rng();
        let range = BallRange::new(BallNumber::new(1), BallNumber::new(5)).unwrap();
        let pick = PickCount::new(3, &range).unwrap();
        // C(5,3) = 10, so requesting 11 should fail
        let count = GameCount::new(11).unwrap();

        let result = generate_unique_tickets(&mut rng, &range, &pick, &count);
        assert!(matches!(
            result,
            Err(crate::error::LottoError::TooManyUniqueGames { .. })
        ));
    }
}
