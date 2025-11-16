# advent-of-code-rust
Advent of Code solutions written in Rust

# Running
cargo run

# Project Setup
1. Copy `aoc_settings.example.json` to `aoc_settings.json`
2. Modify the following keys in `aoc_settings.json`:
    - `session_id` should be set to your Advent of Code session cookie.
    - `passphrase` is a custom password to encrypt your cached input data.

## Alternative project set up
1. Set env var `AOC_SESSION` to your Advent of Code session cookie.
2. Set env var `AOC_PASSPHRASE` to encrypt your cached input data.

# Configuration
This library supports configuration with TOML config files. Setting a configuration file via the
`AOC_CONFIG_FILE` environment variable will prevent loading of any other configuration file.
Otherwise, configuration is loaded by searching the following paths in order:

    1. Standard user configuration directory:
        - Linux: `$XDG_CONFIG_HOME/advent_of_code_data/aoc_settings.toml`
        - Mac: `$HOME/Library/Application Support/com.smacdo.advent_of_code_data/aoc_settings.toml`
        - Windows: `%APPDATA%/com.smacdo.advent_of_code_data/config/aoc_settings.toml`
    2. Home dir (only if no file was found in the previous step!)
        - Linux: `$HOME/.advent_of_code_data.toml`
        - Mac: `$HOME/.advent_of_code_data.toml`
        - Windows: `%USERPROFILE%/.advent_of_code_data.toml`
    3. The current directory is searched last.
       -. `.aoc_settings.toml`

Configuration files are loaded in the order above, and values from later loads will overwrite the
earlier values. This lets you store global settings in your user configuration directory, and then
override specific values with a project config file.

Once the above configuration files have been searched and loaded, the Advent of Code client will
source additional configuration values from the following environment variables:

    - `AOC_SESSION`: The Advent of Code session cookie.
    - `AOC_PASSPHRASE`: An encryption password for locally stored puzzle inputs.
    - `AOC_CONFIG_FILE`: Loads config settings from this file. Other default locations are not used.
    - `AOC_PUZZLE_DIR`: A directory to store cached puzzle inputs and answers.
    - `AOC_SESSIONS_DIR`: A directory to store AOC session information.

Finally any configuration values specified via the command line will take affect after reading the
environment variables.

## Example Configuration
Here is an example configuration file showing the various keys you can specify. Any key that is not
set has a sensible default value except for the encryption passphrase. That must be set in a config
file, an environment variable or via the command line.

```
[client]
passphrase = ""   # Use to encrypt/decrypt the puzzle cache.
session_id = ""   # See "Finding your Advent of Code session cookie" in the README for help.
puzzle_dir = ""   # Directory where cached puzzle inputs and answers are stored.
sessions_dir = "" # Directory where user session data is cached.
```

# Troubleshooting
## Finding your Advent of Code session cookie
1. Log in to `https://adventofcode.com` in Chrome
2. Open developer tools (F12)
3. Go to "application" -> "Storage" -> "Cookies"
4. Click on the "session" entry in the list of cookies.
5. Copy paste the "Cookie Value" which should be a very long string of numbers and letters.
