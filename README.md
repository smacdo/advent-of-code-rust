# advent-of-code-rust
Advent of Code solutions written in Rust

# Running
cargo run

# Project Setup
1. Copy `aoc_settings.example.json` to `aoc_settings.json`
2. Modify the following keys in `aoc_settings.json`:
    - `session_id` should be set to your Advent of Code session cookie.
    - `encryption_token` is a custom password to encrypt your cached input data.

## Alternative project set up
1. Set env var `AOC_SESSION` to your Advent of Code session cookie.
2. Set env var `AOC_PASSWORD` to encrypt your cached input data.

# Configuration
This library supports custom configuration via environment variables and config
files. Configuration files are loaded first, and any missing files are silently
skipped. Files are loaded in the following order:

    1. App config
        - Linux: `$XDG_CONFIG_HOME/advent_of_code_data/config.toml`
        - Mac: `$HOME/Library/Application Support/com.smacdo.advent_of_code_data/config.toml`
        - Windows: `%APPDATA%/com.smacdo.advent_of_code_data/config/config/config.toml`
    2. Home dir
        - Linux: `$HOME/.advent_of_code_data.toml`
        - Mac: `$HOME/.advent_of_code_data.toml`
        - Windows: `%USERPROFILE%/.advent_of_code_data.toml`
    3. Custom path from environment variable `AOC_CONFIG_PATH`, if set.
    4. Local directory
       -. `.advent_of_code_data.toml`

The following environment variables are read, after loading configuration files:

    - `AOC_SESSION`: The Advent of Code session cookie.
    - `AOC_PASSWORD`: An encryption password for locally stored puzzle inputs.
    - `AOC_CONFIG_PATH`: Sets a custom configuration file path.
    - `PUZZLE_DIR`: Directory path where puzzle inputs and answers are stored.

# Troubleshooting
## Finding your Advent of Code session cookie
1. Log in to `https://adventofcode.com` in Chrome
2. Open developer tools (F12)
3. Go to "application" -> "Storage" -> "Cookies"
4. Click on the "session" entry in the list of cookies.
5. Copy paste the "Cookie Value" which should be a very long string of numbers and letters.


