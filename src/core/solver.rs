use crate::core::stop_criterion::StopCriterion;

use super::Problem;

pub trait Solver<SC: StopCriterion<Self::P>> {
    type P: Problem;

    fn solve(&mut self, stop_criterion: &SC) -> <Self::P as Problem>::Solution;
}
