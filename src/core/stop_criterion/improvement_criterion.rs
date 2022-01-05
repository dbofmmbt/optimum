use std::time::Duration;

use crate::{
    components::Percentage,
    core::{Objective, Problem},
};

use super::{StopCriterion, TimeCriterion};

pub struct ImprovementCriterion<P: Problem> {
    last_improvement: usize,
    best: P::Value,
    max_without_improvement: usize,
    time_criterion: TimeCriterion<P>,
}

impl<P: Problem> ImprovementCriterion<P> {
    pub fn new(initial_value: P::Value, max_iters: usize, max_time: Duration) -> Self {
        Self {
            best: initial_value,
            last_improvement: 0,
            max_without_improvement: max_iters,
            time_criterion: TimeCriterion::new(max_time),
        }
    }

    pub fn improvement_took_too_long(&self) -> bool {
        let iterations_without_improvement = self.current_iter() - self.last_improvement;

        iterations_without_improvement > self.max_without_improvement
    }

    fn best_solution_improved(&self, other: P::Value) -> bool {
        match P::OBJECTIVE {
            Objective::Min => other < self.best,
            Objective::Max => other > self.best,
        }
    }
}

impl<P: Problem> StopCriterion<P> for ImprovementCriterion<P> {
    fn progress(&self) -> crate::components::Percentage {
        if self.improvement_took_too_long() {
            Percentage::ONE
        } else {
            self.time_criterion.progress()
        }
    }

    fn update(&mut self, new_value: <P as Problem>::Value) {
        self.time_criterion.update(new_value);

        if self.best_solution_improved(new_value) {
            self.best = new_value;
            self.last_improvement = self.current_iter();
        }
    }

    fn current_iter(&self) -> usize {
        self.time_criterion.current_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO add better test cases

    #[test]
    fn it_works() {
        let max = 10;
        let mut criterion = ImprovementCriterion::<()>::new(0, max, Duration::from_secs(3600));

        (0..max).for_each(|_| criterion.update(0));

        assert_ne!(criterion.progress(), Percentage::ONE);

        criterion.update(0);

        assert_eq!(criterion.progress(), Percentage::ONE);
    }
}
