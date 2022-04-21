use optimum::core::{neighborhood::Move, Evaluation, Problem};

use crate::problem::Tsp;

mod random_two_opt;
pub use random_two_opt::RandomTwoOpt;

pub trait TwoOpt: Iterator<Item = TwoOptMove> {}

impl<T> TwoOpt for T where T: Iterator<Item = TwoOptMove> {}

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