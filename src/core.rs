#![warn(missing_docs)]
//!
//! The basic building blocks for optimization problems.
//!
//! [Problem] and [Solver] are the core traits.
//!
mod problem;
mod solver;
pub mod stop_criterion;

pub use problem::{Objective, Problem};

pub use solver::{Iteration, Solver};
pub use stop_criterion::StopCriterion;
