# CLI Improvements
- Print friendly error on how to look up the session id value when it is not
  detected or incorrect.
- Print friendly instructions to create a config file or env vars when the session
  id or encryption token isn't set.

# SDK Improvements
- Logging all the things
- Reject submissions for future dates automatically unless forced

# Runner Improvements
- Move registry.rs, runner.rs -> rudolph package
- Use per-function registration and de-dup in registry.
- #[PuzzleSolver(day=,year=,part=)]
- #[Example(answer=,input=)]

# Bugs
- No warning or error when running a day that doesn't exist
- No warning for overwriting same day
