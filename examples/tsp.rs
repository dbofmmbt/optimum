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

impl TspSolution {
    fn neighbors(&self, position: usize) -> (usize, usize) {
        let len = self.cities.len();

        let previous = (position - 1 + len) % len;
        let next = (position + 1) % len;

        (previous, next)
    }
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

fn distance_diff(problem: &Tsp, solution: &TspSolution, position: usize, candidate: usize) -> f64 {
    let (previous, next) = solution.neighbors(position);
    distance(problem, previous, candidate, next)
        - distance(problem, previous, solution.cities[position], next)
}

fn distance(problem: &Tsp, previous: usize, current: usize, next: usize) -> f64 {
    let d = &problem.distances;
    d[[previous, current]] + d[[current, next]]
}

fn diff(problem: &Tsp, solution: &TspSolution, first: usize, second: usize) -> NotNan<f64> {
    let first_city = solution.cities[first];
    let second_city = solution.cities[second];

    let first_distance_diff = distance_diff(problem, solution, first, first_city);
    let second_distance_diff = distance_diff(problem, solution, second, second_city);

    NotNan::new(first_distance_diff + second_distance_diff).unwrap()
}

impl Move<Tsp> for TwoOptMove {
    fn value(&self, problem: &Tsp, evaluation: &Evaluation<Tsp>) -> <Tsp as Problem>::Value {
        let solution = evaluation.solution();

        diff(problem, solution, self.0, self.1) + evaluation.value()
    }

    fn apply(&self, problem: &Tsp, evaluation: Evaluation<Tsp>) -> Evaluation<Tsp> {
        let value = self.value(problem, &evaluation);
        let mut solution = evaluation.into_solution();

        solution.cities.swap(self.0, self.1);

        Evaluation::new(solution, value)
    }

    fn reverse(&self) -> Self {
        Self(self.1, self.0)
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
