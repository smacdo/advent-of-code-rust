mod registry;
pub mod runner;

pub use registry::*;

/// A collection of parameters provided to the puzzle solver at runtime.
pub struct SolverArgs<'a> {
    /// The puzzle input provided to the solver.
    pub input: &'a str,
}
