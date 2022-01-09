#![warn(missing_docs)]
//!
//! The basic building blocks for optimization problems.
//!
//! [Problem] and [Solver] are the core traits.
//!
mod problem;
mod solver;
pub mod stop_criterion;

#[doc(inline)]
pub use problem::{Comparison, Evaluation, Objective, Problem};

pub use solver::Solver;
pub use stop_criterion::StopCriterion;
