use crate::{
    components::Percentage,
    core::{Objective, Problem},
};

use super::StopCriterion;

/// Stops as soon as a target value is reached.
pub struct QualityCriterion<P: Problem> {
    target: P::Value,
    done: bool,
    current_iter: usize,
}

impl<P: Problem> QualityCriterion<P> {
    /// Creates a quality criterion based on `target`.
    pub fn new(target: P::Value) -> Self {
        Self {
            target,
            done: false,
            current_iter: 0,
        }
    }
}

impl<P: Problem> StopCriterion<P> for QualityCriterion<P> {
    fn progress(&self) -> crate::components::Percentage {
        if self.done {
            Percentage::ONE
        } else {
            Percentage::ZERO
        }
    }

    fn update(&mut self, new_value: <P as Problem>::Value) {
        self.done = match P::OBJECTIVE {
            Objective::Min => new_value <= self.target,
            Objective::Max => new_value >= self.target,
        };
        self.current_iter += 1;
    }

    fn current_iter(&self) -> usize {
        self.current_iter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut criterion = QualityCriterion::<()>::new(0);

        assert_ne!(criterion.progress(), Percentage::ONE);

        criterion.update(0);

        assert_eq!(criterion.progress(), Percentage::ONE);
    }
}
