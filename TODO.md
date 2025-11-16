# CLI Improvements
- Print friendly error on how to look up the session id value when it is not
  detected or incorrect.
- Print friendly instructions to create a config file or env vars when the session
  id or passphrase isn't set.

# SDK Improvements
- Logging all the things
- Reject submissions for future dates automatically unless forced

# Runner Improvements
- Use solver attribute for registration #[PuzzleSolver(day=,year=,part=)]
- Use example attribute #[Example(part_1="",answer=)]
- Support a "universal" solver fn (eg, both parts in one function)

# Bugs
- No warning for overwriting same day
- Should not fetch input before succesfully running examples
- AOC session should not be required until input or answer submission invoked

# Missing Tests
- Client returns encryption error if cached input is encrypted with different password
- Client returns encryption error if cached input is (un)encrypted and password is (un)set

# Solution Improvements
- Day 10
   - Grid neighbors(pt: Point2) -> Itr<Point2>