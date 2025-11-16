# advent-of-code-rust

Advent of Code solutions written in Rust.

## Quick Start

Run the solutions:

```shell
cargo run
```

## Configuration

To fetch puzzle inputs, you need to provide your AoC session cookie. Choose one:

**Option 1: Environment variables (fastest)**
```shell
export AOC_SESSION=your_session_cookie_here
export AOC_PASSPHRASE=your_encryption_passphrase  # Required if using custom puzzle_dir
cargo run
```

**Option 2: Configuration file**
1. Create `.aoc_settings.toml` in the project root
2. Add your session and passphrase:
```toml
[client]
session_id = "your_session_cookie_here"
passphrase = "your_encryption_passphrase"
```

See the [advent-of-code-data](./crates/advent-of-code-data) crate documentation for more configuration options.

## Crates

This project contains multiple crates:

- **[advent-of-code-data](./crates/advent-of-code-data)**: Library for fetching puzzle inputs and submitting answers to AoC
- **[noclip](./crates/noclip)**: Data structures, algorithms, and utilities
- **[yuletide](./crates/yuletide)**: Framework for writing and managing AoC solutions 
