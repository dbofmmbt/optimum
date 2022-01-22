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
pub trait Solver<SC: StopCriterion<Self::P>, H: IterHook<Self::P>> {
    /// The problem being solved
    type P: Problem;

    /// A iteration consists on a step to generate a candidate solution
    fn iterate(&mut self, stop_criterion: &mut SC, hook: &mut H) -> Option<Evaluation<Self::P>>;

    /// Execute the whole process defined by the solver to achieve a good solution
    ///
    /// By default, it executes [iterate][Self::iterate] while the stop criterion isn't met and returns
    /// the best solution found among all iterations.
    fn solve(&mut self, stop_criterion: &mut SC, hook: &mut H) -> Option<Evaluation<Self::P>> {
        let mut best_evaluation = self.iterate(stop_criterion, hook)?;
        hook.iterated(&best_evaluation);
        stop_criterion.update(best_evaluation.value());

        while !stop_criterion.should_stop() {
            let candidate = {
                match self.iterate(stop_criterion, hook) {
                    Some(s) => s,
                    None => break,
                }
            };

            stop_criterion.update(candidate.value());
            hook.iterated(&candidate);

            if let Comparison::Better = candidate.compare(&best_evaluation) {
                hook.better_changed(&best_evaluation, &candidate);
                best_evaluation = candidate
            }
        }

        Some(best_evaluation)
    }
}

pub mod hook;

pub use hook::IterHook;
