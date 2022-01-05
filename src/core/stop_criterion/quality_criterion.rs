use std::time::Duration;

use crate::{
    components::Percentage,
    core::{Objective, Problem},
};

use super::{StopCriterion, TimeCriterion};

pub struct QualityCriterion<P: Problem> {
    target: P::Value,
    done: bool,
    time_criterion: TimeCriterion<P>,
}

impl<P: Problem> QualityCriterion<P> {
    pub fn new(target: P::Value, max_time: Duration) -> Self {
        Self {
            target,
            done: false,
            time_criterion: TimeCriterion::new(max_time),
        }
    }
}

impl<P: Problem> StopCriterion<P> for QualityCriterion<P> {
    fn progress(&self) -> crate::components::Percentage {
        if self.done {
            Percentage::ONE
        } else {
            self.time_criterion.progress()
        }
    }

    fn update(&mut self, new_value: <P as Problem>::Value) {
        self.time_criterion.update(new_value);

        self.done = match P::OBJECTIVE {
            Objective::Min => new_value <= self.target,
            Objective::Max => new_value >= self.target,
        };
    }

    fn current_iter(&self) -> usize {
        self.time_criterion.current_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut criterion = QualityCriterion::<()>::new(0, Duration::from_secs(3600));

        assert_ne!(criterion.progress(), Percentage::ONE);

        criterion.update(0);

        assert_eq!(criterion.progress(), Percentage::ONE);
    }
}
