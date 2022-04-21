use std::marker::PhantomData;

use crate::core::{Evaluation, Problem, StopCriterion};

use super::{
    explorers::{BestImprovement, FirstImprovement},
    Move, Neighborhood,
};

/// Explores the surroundings of the current evaluation in order to reach better solutions and hopefully find the global best one.
pub trait LocalSearch<P: Problem> {
    fn reach_local_optima(
        &mut self,
        problem: &P,
        evaluation: Evaluation<P>,
        stop_criterion: &mut impl StopCriterion<P>,
    ) -> Evaluation<P>;
}

/// A simple local search strategy which just applies the first movement yielded by the given [Neighborhood].
pub struct HillWalking<P, N> {
    neighborhood: N,
    _p: PhantomData<P>,
}

impl<P: Problem, N: Neighborhood<P>> HillWalking<P, N> {
    pub fn new(neighborhood: N) -> Self {
        Self {
            neighborhood,
            _p: PhantomData,
        }
    }
}

impl<P: Problem, N: Neighborhood<P>> LocalSearch<P> for HillWalking<P, N> {
    fn reach_local_optima(
        &mut self,
        problem: &P,
        evaluation: Evaluation<P>,
        stop_criterion: &mut impl StopCriterion<P>,
    ) -> Evaluation<P> {
        go_to_local_optima(problem, evaluation, stop_criterion, &mut self.neighborhood)
    }
}

/// Reaches the local optima by applying first-improving movements on the current solution.
pub struct HillClimbing<P, N> {
    _p: PhantomData<P>,
    neighborhood: FirstImprovement<P, N>,
}

impl<P: Problem, N: Neighborhood<P>> HillClimbing<P, N> {
    pub fn new(neighborhood: N) -> Self {
        Self {
            neighborhood: FirstImprovement::new(neighborhood),
            _p: PhantomData,
        }
    }
}

impl<P, N> LocalSearch<P> for HillClimbing<P, N>
where
    P: Problem,
    N: Neighborhood<P>,
{
    fn reach_local_optima(
        &mut self,
        problem: &P,
        evaluation: Evaluation<P>,
        stop_criterion: &mut impl StopCriterion<P>,
    ) -> Evaluation<P> {
        go_to_local_optima(problem, evaluation, stop_criterion, &mut self.neighborhood)
    }
}

fn go_to_local_optima<P, N>(
    problem: &P,
    mut evaluation: Evaluation<P>,
    stop_criterion: &mut impl StopCriterion<P>,
    neighborhood: &mut N,
) -> Evaluation<P>
where
    P: Problem,
    N: Neighborhood<P>,
{
    while !stop_criterion.should_stop() {
        if let Some(r#move) = neighborhood.next_neighbor(problem, &evaluation) {
            evaluation = r#move.apply(problem, evaluation);
        }
        stop_criterion.update(evaluation.value());
    }
    evaluation
}

/// A [LocalSearch] which only takes the best improvements.
///
/// Despite the name, it can be used for minimization problems too.
pub struct SteepestAscent<P, N> {
    neighborhood: BestImprovement<P, N>,
}

impl<P: Problem, N: Neighborhood<P>> SteepestAscent<P, N> {
    pub fn new(neighborhood: N) -> Self {
        Self {
            neighborhood: BestImprovement::new(neighborhood),
        }
    }
}

impl<P: Problem, N: Neighborhood<P>> LocalSearch<P> for SteepestAscent<P, N> {
    fn reach_local_optima(
        &mut self,
        problem: &P,
        evaluation: Evaluation<P>,
        stop_criterion: &mut impl StopCriterion<P>,
    ) -> Evaluation<P> {
        go_to_local_optima(problem, evaluation, stop_criterion, &mut self.neighborhood)
    }
}
