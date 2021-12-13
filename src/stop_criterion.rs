use crate::percentage::Percentage;

mod iter_criterion;
mod time_criterion;

pub use iter_criterion::IterCriterion;
pub use time_criterion::TimeCriterion;

pub trait StopCriterion {
    fn progress(&self) -> Percentage;

    fn should_stop(&self) -> bool {
        self.progress() >= Percentage::ONE
    }

    fn update(&mut self);

    fn current_iter(&self) -> usize;
}
