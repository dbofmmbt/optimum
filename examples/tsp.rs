//! # Traveling Salesman Problem
//!
//! This example provides a definition of the TSP problem and a simple solver to the problem: apply the two opt move until a stop criterion is met.
//!  
//! Note that this implementation could be more efficient by not using `Clone` on the `TspSolution`.
//! Initially, I implemented the code without it, but the code gets more complex and this is just an example.
//!

use ndarray::Array2;
use neighborhood::TwoOpt;
use optimum::core::{
    neighborhood::{
        explorers::{BestImprovement, Finite},
        Move,
    },
    stop_criterion::IterCriterion,
    Problem, StopCriterion,
};
use problem_definition::{Tsp, TspSolution};
use rand::{thread_rng, Rng};

const CITIES: usize = 10;

fn main() {
    let v: Vec<f64> = (0..(CITIES * CITIES)).map(|_| thread_rng().gen()).collect();
    let distances: Array2<f64> = Array2::from_shape_vec((CITIES, CITIES), v).unwrap();

    let tsp = Tsp { distances };
    let mut evaluation = tsp.objective_function(TspSolution {
        cities: (0..CITIES).collect(),
    });

    println!("Initial solution: {:?}", evaluation.solution());
    println!("Current value: {}", evaluation.value());

    let mut stop_criterion = IterCriterion::<Tsp>::new(500);

    // TODO substitute this ad-hoc solver for a generic, neighborhood-based one.

    while !stop_criterion.should_stop() {
        let mut neighborhood = Finite::new(
            TwoOpt {
                rng: thread_rng(),
                solution: evaluation.solution(),
            },
            100,
        );
        let mut explorer = BestImprovement::new(&tsp, &mut neighborhood, &evaluation);

        match explorer.next() {
            Some(movement) => evaluation = movement.apply(&tsp, evaluation),
            None => break,
        };
        stop_criterion.update(evaluation.value());
        println!("Current value: {}", evaluation.value());
    }

    println!("Final solution: {:?}", evaluation.solution());
}

pub mod problem_definition {
    use itertools::Itertools;
    use ndarray::Array2;
    use optimum::core::{Evaluation, Objective, Problem};
    use ordered_float::NotNan;

    pub struct Tsp {
        pub distances: Array2<f64>,
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

    #[derive(Debug, Clone)]
    pub struct TspSolution {
        pub cities: Vec<usize>,
    }
}

pub mod neighborhood {
    use optimum::core::{neighborhood::Move, Evaluation, Problem};
    use rand::Rng;

    use crate::problem_definition::{Tsp, TspSolution};

    pub struct TwoOpt<'a, R> {
        pub rng: R,
        pub solution: &'a TspSolution,
    }

    impl<R: Rng> Iterator for TwoOpt<'_, R> {
        type Item = TwoOptMove;

        fn next(&mut self) -> Option<Self::Item> {
            let range = 0..self.solution.cities.len();
            Some(TwoOptMove(
                self.rng.gen_range(range.clone()),
                self.rng.gen_range(range),
            ))
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct TwoOptMove(usize, usize);

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
}
