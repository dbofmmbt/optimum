use crate::core::stop_criterion::StopCriterion;

use super::{Objective, Problem};

pub type Iteration<P> = (<P as Problem>::Solution, <P as Problem>::Value);

pub trait Solver<SC: StopCriterion<Self::P>> {
    type P: Problem;

    fn iterate(&mut self, stop_criterion: &mut SC) -> Option<Iteration<Self::P>>;

    fn solve(&mut self, stop_criterion: &mut SC) -> Option<<Self::P as Problem>::Solution> {
        let (mut best_solution, mut best_value) = self.iterate(stop_criterion)?;
        stop_criterion.update(best_value);

        while !stop_criterion.should_stop() {
            let (candidate_solution, candidate_value) = {
                match self.iterate(stop_criterion) {
                    Some(s) => s,
                    None => break,
                }
            };

            if candidate_is_better::<Self::P, _>(&best_value, &candidate_value) {
                best_solution = candidate_solution;
                best_value = candidate_value;
            }

            stop_criterion.update(candidate_value);
        }

        Some(best_solution)
    }
}

fn candidate_is_better<P: Problem, T: Ord>(current: &T, candidate: &T) -> bool {
    match P::OBJECTIVE {
        Objective::Min => candidate < current,
        Objective::Max => candidate > current,
    }
}
