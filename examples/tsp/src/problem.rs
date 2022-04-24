use std::collections::HashMap;

use itertools::Itertools;
use ndarray::Array2;
use optimum::core::{Evaluation, Objective, Problem};

use crate::CITIES;

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

impl TspSolution {
    pub fn is_valid(&self, _problem: &Tsp) -> bool {
        let mut map = HashMap::with_capacity(self.cities.len());

        for city in self.cities.iter().copied() {
            *map.entry(city).or_insert(0) += 1;
        }

        for city in 0..CITIES {
            if map[&city] != 1 {
                return false;
            }
        }
        true
    }
}
