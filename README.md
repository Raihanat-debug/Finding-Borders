# finding_borders

A Rust implementation of the CSES problem "Finding Borders".

This project provides two standalone solutions for finding all border lengths of a string:

- KMP prefix-function approach
- Polynomial rolling-hash approach

Both programs read a string from standard input and print all border lengths in increasing order.

## Features

- Deterministic linear-time solution using KMP
- Hash-based alternative using polynomial rolling hash
- Sample tests for both implementations
- Benchmark notes in [benchmark.md](benchmark.md)

## Project structure

- [src/bin/kmp.rs](src/bin/kmp.rs) — KMP-based solution
- [src/bin/rolling_hash.rs](src/bin/rolling_hash.rs) — rolling-hash solution
- [tests/kmp_tests.rs](tests/kmp_tests.rs) — integration tests for KMP
- [tests/rolling_hash_tests.rs](tests/rolling_hash_tests.rs) — integration tests for rolling hash
- [benchmark.md](benchmark.md) — comparison of both approaches

## Build and run

From the project root, build the crate with:

```bash
cargo build
```

Run the KMP version:

```bash
cargo run --bin kmp
```

Run the rolling-hash version:

```bash
cargo run --bin rolling_hash
```

## Testing

Run the test suite:

```bash
cargo test
```

## Notes

The KMP solution is the preferred approach when exactness is required, while the rolling-hash variant demonstrates a common hashing technique used in string algorithms.
