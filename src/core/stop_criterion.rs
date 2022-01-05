use crate::components::Percentage;

mod improvement_criterion;
mod iter_criterion;
mod time_criterion;

#[cfg(test)]
pub(crate) mod test_helpers;

pub use improvement_criterion::ImprovementCriterion;
pub use iter_criterion::IterCriterion;
pub use time_criterion::TimeCriterion;

use super::Problem;

pub trait StopCriterion<P: Problem> {
    fn progress(&self) -> Percentage;

    fn should_stop(&self) -> bool {
        self.progress() >= Percentage::ONE
    }

    fn update(&mut self, new_value: P::Value);

    fn current_iter(&self) -> usize;
}
