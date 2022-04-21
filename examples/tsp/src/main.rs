//! # Traveling Salesman Problem
//!
//! This example provides a definition of the TSP problem and a simple solver to the problem: apply the two opt move until a stop criterion is met.
//!  
//! Note that this implementation could be more efficient by not using `Clone` on the `TspSolution`.
//! Initially, I implemented the code without it, but the code gets more complex and this is just an example.
//!

use ndarray::Array2;
use neighborhood::two_opt::RandomTwoOpt;
use optimum::core::{
    neighborhood::{
        explorers::{BestImprovement, Finite},
        Move,
    },
    stop_criterion::IterCriterion,
    Problem, StopCriterion,
};
use problem::{Tsp, TspSolution};

use rand::{thread_rng, Rng};

const CITIES: usize = 10;

fn main() {
    let v: Vec<usize> = (0..(CITIES * CITIES))
        .map(|_| thread_rng().gen_range(0..100))
        .collect();
    let distances: Array2<usize> = Array2::from_shape_vec((CITIES, CITIES), v).unwrap();

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
            RandomTwoOpt {
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

pub mod neighborhood;
pub mod problem;
