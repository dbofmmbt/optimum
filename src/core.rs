//! TODO document this module properly.
//!
//! Take a look at [Problem].
//!

mod problem;
mod solver;
pub mod stop_criterion;

pub use problem::{Objective, Problem};

pub use stop_criterion::StopCriterion;

pub trait Move {
    type P: Problem;

    fn apply(&mut self, s: <Self::P as Problem>::Solution) -> <Self::P as Problem>::Solution;
}

pub trait ReversibleMove: Move {
    fn reversal(&self) -> Self;
}
