//!
//! The basic building blocks for optimization problems.
//!
//! [Problem] and [Solver] are the core traits.
//!
pub mod neighborhood;
mod problem;
pub mod solver;
pub mod stop_criterion;

#[doc(inline)]
pub use problem::{compare_values, Comparison, Evaluation, Objective, Problem};

pub use solver::Solver;
pub use stop_criterion::StopCriterion;
