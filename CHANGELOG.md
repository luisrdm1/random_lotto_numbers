# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
