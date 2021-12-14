use std::fmt::Debug;

use crate::stop_criterion::StopCriterion;

#[allow(dead_code)]
pub enum Objective {
    Min,
    Max,
}

pub trait Problem {
    const OBJECTIVE: Objective;
    type Solution;
    type Value: Ord + Copy + Debug;

    fn objective_function(&self, solution: &Self::Solution) -> Self::Value;
}

pub trait Constructor<SC: StopCriterion> {
    type P: Problem;

    fn build(&mut self, stop_criterion: &SC) -> <Self::P as Problem>::Solution;
}

pub trait Move {
    type P: Problem;

    fn apply(&mut self, s: <Self::P as Problem>::Solution) -> <Self::P as Problem>::Solution;
}

pub trait ReversibleMove: Move {
    fn reversal(&self) -> Self;
}
