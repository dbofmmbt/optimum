//! See [Solver].

use crate::core::stop_criterion::StopCriterion;

use super::{Comparison, Evaluation, Problem};

/// A solver is a procedure which seeks to find a good solution for a given [Problem].
///
/// There's two kinds of solvers:
/// - Metaheuristics
///     - They are generic over the [Problem] being solved.
/// - Heuristics
///     - They're [Problem]-specific.
pub trait Solver<SC: StopCriterion<Self::P>, LC: LifeCycle<Self::P> = BasicLifeCycle> {
    /// The problem being solved
    type P: Problem;

    /// A iteration consists on a step to generate a candidate solution
    fn iterate(&mut self, stop_criterion: &mut SC) -> Option<Evaluation<Self::P>>;

    /// Execute the whole process defined by the solver to achieve a good solution
    ///
    /// By default, it executes [iterate][Self::iterate] while the stop criterion isn't met and returns
    /// the best solution found among all iterations.
    fn solve(
        &mut self,
        stop_criterion: &mut SC,
        life_cycle: &mut LC,
    ) -> Option<Evaluation<Self::P>> {
        let mut best_evaluation = self.iterate(stop_criterion)?;
        stop_criterion.update(best_evaluation.value());

        while !stop_criterion.should_stop() {
            let candidate = {
                match self.iterate(stop_criterion) {
                    Some(s) => s,
                    None => break,
                }
            };

            stop_criterion.update(candidate.value());
            life_cycle.iterated(&candidate);

            if let Comparison::Better = candidate.compare(&best_evaluation) {
                life_cycle.better_changed(&best_evaluation, &candidate);
                best_evaluation = candidate
            }
        }

        Some(best_evaluation)
    }
}

pub trait LifeCycle<P: Problem> {
    fn iterated(&mut self, new: &Evaluation<P>) {}

    fn better_changed(&mut self, old: &Evaluation<P>, new: &Evaluation<P>) {}
}

pub struct BasicLifeCycle;

impl<P: Problem> LifeCycle<P> for BasicLifeCycle
where
    P::Solution: std::fmt::Debug,
    P::Value: std::fmt::Debug,
{
    fn iterated(&mut self, new: &Evaluation<P>) {
        eprintln!("ITER VALUE {:?}", new.value());
    }
}
