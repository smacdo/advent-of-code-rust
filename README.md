# advent-of-code-rust
Advent of Code solutions written in Rust

# Running
define env var AOC_SESSION and AOC_PASSWORD
cargo run

# Project Setup
1. Copy `aoc_settings.example.json` to `aoc_settings.json`
2. Modify the following keys in `aoc_settings.json`:
    - `session_id` should be set to your Advent of Code session id cookie.
    - `encryption_token` is a custom password used to encrypt your input data. Pick any value you want.

## Finding your Advent of Code session cookie
1. Log in to `https://adventofcode.com` in Chrome
2. Open developer tools (F12)
3. Go to "application" -> "Storage" -> "Cookies"
4. Click on the "session" entry in the list of cookies.
5. Copy paste the "Cookie Value" which should be a very long string of numbers and letters.