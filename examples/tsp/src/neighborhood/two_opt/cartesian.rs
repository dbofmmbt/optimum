use std::ops::Range;

use itertools::{Itertools, Product};
use optimum::{core::Evaluation, metaheuristics::neighborhood::Neighborhood};

use crate::problem::Tsp;

use super::TwoOptMove;

type Iter = Product<Range<usize>, Range<usize>>;

pub struct CartesianTwoOpt {
    iter: Iter,
}

impl CartesianTwoOpt {
    pub fn new(evaluation: &Evaluation<Tsp>) -> Self {
        Self {
            iter: iter(evaluation),
        }
    }
}

fn iter(evaluation: &Evaluation<Tsp>) -> Iter {
    let cities = &evaluation.solution().cities;
    (0..cities.len()).cartesian_product(0..cities.len())
}

impl Neighborhood<Tsp> for CartesianTwoOpt {
    type Move = TwoOptMove;

    fn next_neighbor(&mut self, _: &Tsp, _evaluation: &Evaluation<Tsp>) -> Option<Self::Move> {
        let r#move = TwoOptMove::from(self.iter.next()?);
        Some(r#move)
    }

    fn solution_changed(&mut self, evaluation: &Evaluation<Tsp>) {
        self.iter = iter(evaluation);
    }
}
