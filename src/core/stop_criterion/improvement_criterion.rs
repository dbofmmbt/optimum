use crate::{
    components::Percentage,
    core::{Objective, Problem},
};

use super::StopCriterion;

/// The stop criterion is met when the solver executes N iterations in sequence without improving the best solution.
pub struct ImprovementCriterion<P: Problem> {
    last_improvement: usize,
    best: P::Value,
    max_without_improvement: usize,
    current_iter: usize,
}

impl<P: Problem> ImprovementCriterion<P> {
    /// Creates a new improvement criterion.
    ///
    /// - `initial` is the initial best value (a very high value for minimization or very low value for maximization).
    /// - `max_iters` number of iterations in sequence without improvement to stop.
    pub fn new(initial: P::Value, max_iters: usize) -> Self {
        Self {
            best: initial,
            last_improvement: 0,
            current_iter: 0,
            max_without_improvement: max_iters,
        }
    }

    fn improvement_took_too_long(&self) -> bool {
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
            Percentage::ZERO
        }
    }

    fn update(&mut self, new_value: <P as Problem>::Value) {
        self.current_iter += 1;

        if self.best_solution_improved(new_value) {
            self.best = new_value;
            self.last_improvement = self.current_iter();
        }
    }

    fn current_iter(&self) -> usize {
        self.current_iter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO add better test cases

    #[test]
    fn it_works() {
        let max = 10;
        let mut criterion = ImprovementCriterion::<()>::new(0, max);

        (0..max).for_each(|_| criterion.update(0));

        assert_ne!(criterion.progress(), Percentage::ONE);

        criterion.update(0);

        assert_eq!(criterion.progress(), Percentage::ONE);
    }
}
