use rand::Rng;

use crate::problem::TspSolution;

use super::TwoOptMove;

pub struct RandomTwoOpt<'a, R> {
    pub rng: R,
    pub solution: &'a TspSolution,
}

impl<R: Rng> Iterator for RandomTwoOpt<'_, R> {
    type Item = TwoOptMove;

    fn next(&mut self) -> Option<Self::Item> {
        let range = 0..self.solution.cities.len();
        Some(TwoOptMove(
            self.rng.gen_range(range.clone()),
            self.rng.gen_range(range),
        ))
    }
}
