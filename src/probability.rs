//! Probability calculation module.
//!
//! This module provides functions for calculating lottery probabilities
//! using efficient algorithms that avoid factorial calculation.

use crate::error::{LottoError, Result};

/// Calculate the binomial coefficient C(n, k) without using factorial.
///
/// This implementation uses an iterative algorithm that alternates
/// multiplication and division to avoid overflow: C(n,k) = ∏(n-i+1)/i for i=1..k
///
/// Uses u128 to support large lottery configurations while avoiding
/// the need for arbitrary-precision arithmetic (BigInt).
///
/// **Time complexity:** O(k)  
/// **Space complexity:** O(1)
///
/// # Arguments
///
/// * `n` - The total number of items
/// * `k` - The number of items to choose
///
/// # Returns
///
/// The binomial coefficient C(n, k), or an error if overflow occurs.
///
/// # Examples
///
/// ```
/// use lotto_quick_pick::probability::combination;
///
/// // Mega-Sena: choose 6 from 60
/// assert_eq!(combination(60, 6).unwrap(), 50_063_860);
///
/// // Smaller lottery
/// assert_eq!(combination(10, 3).unwrap(), 120);
/// ```
pub fn combination(n: usize, k: usize) -> Result<u128> {
    if k > n {
        return Ok(0);
    }

    if k == 0 || k == n {
        return Ok(1);
    }

    // Use the smaller of k or n-k for efficiency: C(n,k) = C(n, n-k)
    let k = k.min(n - k);

    let mut result: u128 = 1;

    for i in 0..k {
        // Calculate (n - i) / (i + 1) iteratively with overflow checking
        result =
            result
                .checked_mul((n - i) as u128)
                .ok_or_else(|| LottoError::CalculationOverflow {
                    operation: format!(
                        "combination C({},{}) multiplication overflow at iteration {}",
                        n, k, i
                    ),
                })?;

        // Division should never fail, but check for safety
        result =
            result
                .checked_div((i + 1) as u128)
                .ok_or_else(|| LottoError::CalculationOverflow {
                    operation: format!(
                        "combination C({},{}) division by zero at iteration {}",
                        n, k, i
                    ),
                })?;
    }

    Ok(result)
}

/// Calculate the probability of matching exactly `match_count` balls
/// in a lottery game.
///
/// # Arguments
///
/// * `total_balls` - Total number of balls in the lottery
/// * `pick_count` - Number of balls picked per game
/// * `match_count` - Number of balls to match
///
/// # Returns
///
/// A tuple of (favorable_outcomes, total_outcomes) representing the probability.
/// The actual probability is favorable_outcomes / total_outcomes.
///
/// # Examples
///
/// ```
/// use lotto_quick_pick::probability::calculate_probability;
///
/// // Probability of matching 6 out of 6 in Mega-Sena (60 balls, pick 6)
/// let (favorable, total) = calculate_probability(60, 6, 6).unwrap();
/// assert_eq!(favorable, 1);
/// assert_eq!(total, 50_063_860);
/// ```
pub fn calculate_probability(
    total_balls: usize,
    pick_count: usize,
    match_count: usize,
) -> Result<(u128, u128)> {
    if match_count > pick_count {
        return Err(LottoError::InvalidMatchCount {
            match_count,
            pick_count,
        });
    }

    // Total possible outcomes: C(total_balls, pick_count)
    let total_outcomes = combination(total_balls, pick_count)?;

    // Favorable outcomes: C(pick_count, match_count) * C(total_balls - pick_count, pick_count - match_count)
    let ways_to_match = combination(pick_count, match_count)?;
    let ways_to_miss = combination(total_balls - pick_count, pick_count - match_count)?;

    let favorable_outcomes =
        ways_to_match
            .checked_mul(ways_to_miss)
            .ok_or_else(|| LottoError::CalculationOverflow {
                operation: format!("favorable outcomes: {} * {}", ways_to_match, ways_to_miss),
            })?;

    Ok((favorable_outcomes, total_outcomes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_small_values() {
        assert_eq!(combination(5, 0).unwrap(), 1);
        assert_eq!(combination(5, 1).unwrap(), 5);
        assert_eq!(combination(5, 2).unwrap(), 10);
        assert_eq!(combination(5, 3).unwrap(), 10);
        assert_eq!(combination(5, 4).unwrap(), 5);
        assert_eq!(combination(5, 5).unwrap(), 1);
    }

    #[test]
    fn test_combination_mega_sena() {
        // Mega-Sena: C(60, 6) = 50,063,860
        assert_eq!(combination(60, 6).unwrap(), 50_063_860);
    }

    #[test]
    fn test_combination_large_lottery() {
        // Larger lottery: C(100, 10)
        assert_eq!(combination(100, 10).unwrap(), 17_310_309_456_440);
    }

    #[test]
    fn test_combination_symmetry() {
        // C(n, k) = C(n, n-k)
        assert_eq!(combination(20, 5).unwrap(), combination(20, 15).unwrap());
        assert_eq!(combination(30, 10).unwrap(), combination(30, 20).unwrap());
    }

    #[test]
    fn test_combination_edge_cases() {
        assert_eq!(combination(0, 0).unwrap(), 1);
        assert_eq!(combination(10, 0).unwrap(), 1);
        assert_eq!(combination(10, 10).unwrap(), 1);
        assert_eq!(combination(10, 11).unwrap(), 0); // k > n
    }

    #[test]
    fn test_calculate_probability_match_all() {
        // Probability of matching all 6 in Mega-Sena
        let (favorable, total) = calculate_probability(60, 6, 6).unwrap();
        assert_eq!(favorable, 1);
        assert_eq!(total, 50_063_860);
    }

    #[test]
    fn test_calculate_probability_match_five() {
        // Probability of matching exactly 5 in a 60-ball, pick-6 lottery
        let (favorable, total) = calculate_probability(60, 6, 5).unwrap();
        assert_eq!(total, 50_063_860);
        assert!(favorable > 0);
    }

    #[test]
    fn test_calculate_probability_invalid_match_count() {
        // Cannot match more balls than picked
        let result = calculate_probability(60, 6, 7);
        assert!(matches!(result, Err(LottoError::InvalidMatchCount { .. })));
    }

    #[test]
    fn test_calculate_probability_simple_lottery() {
        // Simple lottery: 10 balls, pick 3, match 2
        let (favorable, total) = calculate_probability(10, 3, 2).unwrap();
        assert_eq!(total, 120); // C(10, 3)
        assert_eq!(favorable, 21); // C(3, 2) * C(7, 1) = 3 * 7
    }
}
