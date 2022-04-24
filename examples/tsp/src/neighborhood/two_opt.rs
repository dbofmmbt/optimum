use optimum::{
    core::{Evaluation, Problem},
    metaheuristics::neighborhood::{Move, Neighborhood},
};

use crate::problem::Tsp;

mod random_two_opt;
pub use random_two_opt::RandomTwoOpt;

pub mod cartesian;

pub trait TwoOpt<P: Problem>: Neighborhood<P, Move = TwoOptMove> {}

impl<T, P: Problem> TwoOpt<P> for T where T: Neighborhood<P, Move = TwoOptMove> {}

#[derive(Debug, Clone, Copy)]
pub struct TwoOptMove(pub usize, pub usize);

impl Move<Tsp> for TwoOptMove {
    fn value(&self, problem: &Tsp, evaluation: &Evaluation<Tsp>) -> <Tsp as Problem>::Value {
        let mut solution = evaluation.solution().clone();
        solution.cities.swap(self.0, self.1);
        problem.objective_function(solution).value()
    }

    fn apply(&self, problem: &Tsp, evaluation: Evaluation<Tsp>) -> Evaluation<Tsp> {
        let value = self.value(problem, &evaluation);
        let mut solution = evaluation.into_solution();

        solution.cities.swap(self.0, self.1);

        Evaluation::new(solution, value)
    }
}

impl From<(usize, usize)> for TwoOptMove {
    fn from(t: (usize, usize)) -> Self {
        TwoOptMove(t.0, t.1)
    }
}
