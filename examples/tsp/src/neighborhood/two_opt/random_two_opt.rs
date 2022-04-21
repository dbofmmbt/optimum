use optimum::{core::Evaluation, metaheuristics::neighborhood::Neighborhood};
use rand::Rng;

use crate::problem::Tsp;

use super::TwoOptMove;

pub struct RandomTwoOpt<R> {
    pub rng: R,
}

impl<R: Rng> Neighborhood<Tsp> for RandomTwoOpt<R> {
    type Move = TwoOptMove;

    fn next_neighbor(
        &mut self,
        _problem: &Tsp,
        evaluation: &Evaluation<Tsp>,
    ) -> Option<Self::Move> {
        let range = 0..evaluation.solution().cities.len();
        Some(TwoOptMove(
            self.rng.gen_range(range.clone()),
            self.rng.gen_range(range),
        ))
    }
}
