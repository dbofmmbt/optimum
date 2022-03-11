use itertools::Itertools;
use ndarray::Array2;
use optimum::core::{
    neighborhood::{Move, Neighborhood},
    Evaluation, Objective, Problem,
};
use ordered_float::NotNan;
use rand::Rng;

fn main() {}

struct Tsp {
    distances: Array2<f64>,
}

struct TspSolution {
    cities: Vec<usize>,
}

impl Problem for Tsp {
    const OBJECTIVE: Objective = Objective::Min;

    type Solution = TspSolution;

    type Value = NotNan<f64>;

    fn objective_function(&self, solution: Self::Solution) -> optimum::core::Evaluation<Self> {
        let total_distance: f64 = solution
            .cities
            .iter()
            .copied()
            .tuple_windows::<(_, _)>()
            .map(|(a, b)| self.distances[[a, b]])
            .sum();

        Evaluation::new(solution, NotNan::new(total_distance).unwrap())
    }
}

struct TwoOpt<'a, R> {
    rng: R,
    solution: &'a TspSolution,
}

#[derive(Debug, Clone, Copy)]
struct TwoOptMove(usize, usize);

impl Move<Tsp> for TwoOptMove {
    fn compare(&self, problem: &Tsp, solution: &<Tsp as Problem>::Solution) -> optimum::core::Comparison {
        self.apply(solution);
    }

    fn diff(&self, problem: &Tsp, solution: &<Tsp as Problem>::Solution) -> <Tsp as Problem>::Value {
        todo!()
    }

    fn apply(self, mut solution: <Tsp as Problem>::Solution) -> <Tsp as Problem>::Solution {
        solution.cities.swap(self.0, self.1);
        solution
    }
}

impl<R: Rng> Neighborhood<Tsp> for TwoOpt<'_, R> {
    type Move = TwoOptMove;
}

impl<R: Rng> Iterator for TwoOpt<'_, R> {
    type Item = <Self as Neighborhood<Tsp>>::Move;

    fn next(&mut self) -> Option<Self::Item> {
        let range = 0..self.solution.cities.len();
        Some(TwoOptMove(
            self.rng.gen_range(range.clone()),
            self.rng.gen_range(range),
        ))
    }
}
