use itertools::Itertools;
use ndarray::Array2;
use optimum::core::{Evaluation, Objective, Problem};

pub struct Tsp {
    pub distances: Array2<usize>,
}

impl Problem for Tsp {
    const OBJECTIVE: Objective = Objective::Min;

    type Solution = TspSolution;

    type Value = usize;

    fn objective_function(&self, solution: Self::Solution) -> optimum::core::Evaluation<Self> {
        let total_distance = solution
            .cities
            .iter()
            .copied()
            .tuple_windows::<(_, _)>()
            .map(|(a, b)| self.distances[[a, b]])
            .sum();

        Evaluation::new(solution, total_distance)
    }
}

#[derive(Debug, Clone)]
pub struct TspSolution {
    pub cities: Vec<usize>,
}
