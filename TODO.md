# Improvements
## CLI
- Print friendly error on how to look up the session id value when it is not
  detected or incorrect.
- Print friendly instructions to create a config file or env vars when the session
  id or passphrase isn't set.
- CLI command that prints out the configuration path

## Advent of Code SDK
- Logging all the things
- Reject submissions for future dates automatically unless forced

## Runner
- Use solver attribute for registration #[PuzzleSolver(day=,year=,part=)]
- Use example attribute #[Example(part_1="",answer=)]
- Support a "universal" solver fn (eg, both parts in one function)

## Ube
### Grid
- Add cols() and col(c) like the rows() and row(r) methods.

### Point
- Add tests for EAST/NORTH/WEST/SOUTH constants.
- Convert zero(), one(), unit_x(), and unit_y() methods to be constants.
- Complete implementation for Point3.

### Graph
- Move to standalone module.
- Support non-string keys.
- Support storing additional data in nodes.
- Add tests, documentation.
- Disallow $\d+ prefixed node names as they are reserved by the system.
- Optimize the builder by reducing hte number of string copies that are happening.

### Misc
- Finish counter module. 
  - Documentation.
  - Tests
- Finish union find module.
  - Documentation.
  - Tests
- Replace `unwrap()` and `expect(...)` with `Option<T>` and `Result<T,E>` for union_find.rs.
- Improve counter and union find implementations with fast hashing.

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