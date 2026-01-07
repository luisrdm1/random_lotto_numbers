//! Error types for the lotto quick pick library.
//!
//! This module defines all error variants that can occur during
//! lottery ticket generation and probability calculations.

use thiserror::Error;

/// Represents all possible errors in the lotto quick pick library.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum LottoError {
    /// The starting number is greater than or equal to the ending number.
    #[error("Start value ({start}) must be less than end value ({end})")]
    InvalidRange { start: u8, end: u8 },

    /// The number of balls to pick exceeds the available range.
    #[error("Cannot pick {pick} balls from a range of {available} values")]
    PickExceedsRange { pick: usize, available: usize },

    /// The number of games requested is zero.
    #[error("Number of games must be at least 1")]
    ZeroGames,

    /// An arithmetic overflow occurred during probability calculation.
    #[error("Arithmetic overflow during calculation: {operation}")]
    CalculationOverflow { operation: String },

    /// The number of balls to match exceeds the pick size.
    #[error("Cannot match {match_count} balls when only picking {pick_count}")]
    InvalidMatchCount {
        match_count: usize,
        pick_count: usize,
    },

    /// Requested more unique tickets than mathematically possible.
    #[error("Cannot generate {requested} unique tickets (maximum possible: {maximum})")]
    TooManyUniqueGames { requested: usize, maximum: u128 },

    /// Failed to generate requested number of unique tickets after many attempts.
    #[error("Failed to generate {requested} unique tickets (only generated {generated} after maximum attempts)")]
    UniqueGenerationFailed {
        requested: usize,
        generated: usize,
    },

    /// Input/output error during user interaction.
    #[error("I/O error: {0}")]
    IoError(String),

    /// Failed to parse user input.
    #[error("Failed to parse input: {0}")]
    ParseError(String),
}

/// Type alias for Results using LottoError.
pub type Result<T> = std::result::Result<T, LottoError>;
