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
- **[ube](./crates/ube)**: Data structures, algorithms, and utilities
- **[yuletide](./crates/yuletide)**: Framework for writing and managing AoC solutions 

## AI Disclaimer
I do not use any AI to solve or write code for any Advent of Code puzzles in any way. Figuring those puzzles out and coding them up is a joy. I wouldn't want to rob myself of the fun! The only time I use AI with the puzzle solvers is to perform refactorings, eg if I change how the `yuletide` registration API works.

I use Claude and other AI tools to help with the following non-puzzle bits:

  - Writing/maintaining/modifying CLI scripts in `scripts/`.
  - Writing _some_ of the unit tests and documentation for functions in `crates/`.
  - Improving my writing prose after writing the technical details myself.
  - Using Claude as a code reviewer to catch bugs or other inconsistencies.
  
In general, I use AI to provide the extra polish for my crates that I otherwise would not have time to do myself. The vast majority of the code I write is my own creation. I do not vibe code anything outside of simple CLI scripts.