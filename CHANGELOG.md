# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.3.0] - 2026-01-07

### Performance

- **Hoisted strategy selection**: `BitwiseStrategy::select()` moved outside hot loop in `generate_unique_tickets()`
  - Eliminates redundant strategy selection on every ticket generation
  - Strategy determined once per batch instead of per ticket

- **Optimized `to_balls()` with `trailing_zeros()`**: Changed from O(range_size) to O(pick_count)
  - Uses bit manipulation (`trailing_zeros()` + `bits &= bits - 1`) to iterate only set bits
  - For Mega-Sena (6 picks from 60 range): iterates 6 times instead of 60
  - Mega-Sena single ticket: **52.6ns** (2.2x faster than HashSet)
  - Mega-Sena 3 picks: **14.4ns** (2.9x faster than HashSet)

- **Pre-allocated vectors**: Added `Vec::with_capacity(count_balls())` in `to_balls()`
  - Eliminates incremental reallocations during ball collection
  - Single allocation at exact required size

- **Eliminated unnecessary sorting**: New `Ticket::from_sorted()` method
  - `to_balls()` returns pre-sorted Vec (bits iterated in ascending order)
  - Skips redundant `sort_unstable()` call in final conversion
  - Includes `debug_assert!` to validate sorted invariant

### Changed

- **Updated tests and benchmarks**: All deprecated function calls replaced with `_ticketkey_` variants
  - 8 tests in `ticket_bitwise.rs` migrated
  - 6 benchmark functions in `bitwise_comparison.rs` migrated
  - Zero clippy warnings

### Performance Benchmarks

Comparison of HashSet vs Bitwise (with all optimizations):

| Scenario | HashSet | Bitwise | Speedup | Improvement vs v1.2.0 |
|----------|---------|---------|---------|----------------------|
| Mega-Sena 6 picks | 52.6 ns | 23.6 ns | **2.23x** | HashSet: -61.7%, Bitwise: -48.4% |
| Mega-Sena 3 picks | 41.2 ns | 14.4 ns | **2.86x** | HashSet: -52.1%, Bitwise: -55.2% |
| Lotomania 50 picks | 447.7 ns | 374.3 ns | **1.20x** | HashSet: -58.9%, Bitwise: -23.4% |
| Powerball 5 picks | 52.9 ns | 23.4 ns | **2.26x** | HashSet: -55.0%, Bitwise: -46.5% |
| Range 200, 10 picks | 111.0 ns | 67.6 ns | **1.64x** | HashSet: -44.7%, Bitwise: -19.0% |

**Key insight**: Optimizations benefit both implementations, with bitwise maintaining 1.2x-2.9x advantage.

## [1.2.0] - 2026-01-07

### Added

- **New optimized API**: `generate_ticketkey_bitwise()` and variants (`generate_ticketkey_u64_bitmap()`, `generate_ticketkey_u128_bitmap()`, `generate_ticketkey_vec_bitmap()`)
  - Generate `TicketKey` directly from bitmaps without intermediate `Vec<BallNumber>` allocation
  - 30% performance improvement vs v1.1.0 through elimination of double conversion
  
- **Validation with `assert!`**: All bitmap generation functions now validate invariants:
  - Bit count matches expected pick count
  - No bits set outside valid range (using bitmask validation)
  - Always-on validation (not just debug builds) for correctness guarantees

- **11 new validation tests**:
  - Boundary tests (max range u64/u128)
  - Bit count validation tests
  - Round-trip preservation tests
  - Edge case tests (single value range, full range selection)
  - Duplicate detection tests

- **Comparison benchmark**: `ticket_key_comparison.rs` now compares `HashSet<TicketKey>` vs `HashSet<Ticket>` implementations

### Changed

- **BREAKING**: `generate_unique_tickets()` now uses `generate_ticketkey_bitwise()` internally
  - Eliminates `Vec<BallNumber> → TicketKey` conversion overhead
  - **Performance impact**: Mega-Sena 1000 games: 167µs → 117µs (-30%)

- **Fixed**: `TicketKey::to_balls()` now correctly iterates only up to `range.size()`
  - Previously iterated full 64/128 bits, could convert invalid bits
  - Now respects actual range boundaries preventing out-of-range ball generation

- **Improved**: `generate_ticketkey_vec_bitmap()` uses `div_ceil()` instead of manual calculation

### Deprecated

- `generate_ticket_bitwise()` - Use `generate_ticketkey_bitwise()` instead
- `generate_ticket_u64_bitmap()` - Use `generate_ticketkey_u64_bitmap()` instead
- `generate_ticket_u128_bitmap()` - Use `generate_ticketkey_u128_bitmap()` instead
- `generate_ticket_vec_bitmap()` - Use `generate_ticketkey_vec_bitmap()` instead

**Migration guide**: Replace function calls with `_ticketkey_` variants and use `key.to_balls(&range)` if you need `Vec<BallNumber>`.

### Performance

Benchmark results on typical lottery scenarios:

| Operation | v1.1.0 | v1.2.0 | Improvement |
|-----------|--------|--------|-------------|
| Mega-Sena 1000 games | 167.74 µs | 117.16 µs | **-30.2%** |
| Mega-Sena 100 games | 16.68 µs | 11.78 µs | **-29.3%** |
| Mega-Sena 10 games | 1.71 µs | 1.25 µs | **-26.9%** |
| Lotomania 100 games | 115.01 µs | 88.89 µs | **-22.7%** |

New implementation is now competitive with or faster than `HashSet<Ticket>` direct approach while maintaining the benefits of compact bitmap representation.

### Technical Details

**Root cause of v1.1 performance issue**: Double conversion
```rust
// v1.1 (slow)
bitmap → Vec<BallNumber> → TicketKey → insert into HashSet
         ^^^^^^^^^^^^^^^ heap allocation + iteration
                          ^^^^^^^^^ rebuild bitmap from Vec

// v1.2 (fast)
bitmap → TicketKey → insert into HashSet → Vec<BallNumber> (only at end)
         ^^^^^^^^^ direct, no intermediate allocation
```

**Memory efficiency**: For Mega-Sena (6 numbers), `TicketKey::U64` uses 8 bytes vs `Ticket` with `Vec<BallNumber>` using ~32+ bytes (heap allocation overhead).

**Correctness improvements**: Validation ensures all generated bitmaps are well-formed:
- No shift overflows (bit position always < bitmap size)
- Exact bit count matches requested picks
- No bits set outside valid range

---

## [1.1.0] - 2025-12-XX

### Added

- `TicketKey` bitmap representation for efficient uniqueness checking
- Three bitmap strategies: U64, U128, VecU64 based on range size
- Comprehensive test suite (75 unit tests + 23 doctests)

### Changed

- Upgraded to Rust Edition 2024
- Upgraded to rand 0.9.2 (breaking change: `thread_rng()` → `rng()`)

### Performance

- 55-67% faster than HashSet for single ticket generation
- Bitwise duplicate checking: O(1) vs O(log n) HashSet lookup

---

## [1.0.0] - 2025-XX-XX

### Added

- Initial release with core lottery ticket generation
- CLI interface with clap
- Probability calculations without overflow (u128)
- Multiple generation strategies (insertion, exclusion)
- Comprehensive error handling
- Colored terminal output

[1.2.0]: https://github.com/yourusername/lotto-quick-pick/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/yourusername/lotto-quick-pick/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/yourusername/lotto-quick-pick/releases/tag/v1.0.0
