//! Defines the [StopCriterion] trait and a number of stop criterions which are often useful.
//!
//! - [IterCriterion]: stops based on the maximum number of iterations.
//! - [TimeCriterion]: stops after a given time duration.
//! - [QualityCriterion]: stops after a solution achieves the minimum desired quality.
//! - [ImprovementCriterion]: stops after no more improvements have been performed for a given number of iterations.
//!
//! Additionally, there's the [CriterionCombiner], which allows to combine two [StopCriterion]s
//! into one and stops as soon as either of them stops.

use num_traits::real::Real;

mod improvement_criterion;
mod iter_criterion;
mod quality_criterion;
mod time_criterion;

mod criterion_combiner;

#[cfg(test)]
pub(crate) mod test_helpers;

pub use improvement_criterion::ImprovementCriterion;
pub use iter_criterion::IterCriterion;
pub use quality_criterion::QualityCriterion;
pub use time_criterion::TimeCriterion;

pub use criterion_combiner::CriterionCombiner;

use super::Problem;

/// A stop criterion determines when the [Solver][crate::core::Solver] should stop seeking better solutions
/// and just yield the results. The implementations provided in [core::stop_criterion][crate::core::stop_criterion] should cover
/// most cases of stop criterions, such as [IterCriterion] and [TimeCriterion].
pub trait StopCriterion<P: Problem, R: Real = f64> {
    /// The progress starts at zero and increases during execution.
    fn progress(&self) -> R;

    /// True when [progress][Self::progress] achieves 100%.
    fn should_stop(&self) -> bool {
        self.progress() >= R::one()
    }

    /// Updates the internal state. `new_value` is the value's newly generated solution.
    ///
    /// Must be called at the end of each iteration.
    fn update(&mut self, new_value: P::Value);

    /// This is basically how many times [update][Self::update] was called.
    fn current_iter(&self) -> usize;
}
