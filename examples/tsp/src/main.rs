//! # Traveling Salesman Problem
//!
//! This example provides a definition of the TSP problem and a simple solver to the problem: apply the two opt move until a stop criterion is met.
//!  
//! Note that this implementation could be more efficient by not using `Clone` on the `TspSolution`.
//! Initially, I implemented the code without it, but the code gets more complex and this is just an example.
//!

use std::{clone::Clone, time::Duration};

use ndarray::Array2;
use neighborhood::two_opt::RandomTwoOpt;
use optimum::{
    core::{stop_criterion::TimeCriterion, Problem, Solver, StopCriterion},
    metaheuristics::{
        genetic::{
            brkga::{Brkga, EmptyHook, Params, RandomMemberBuilder},
            Decoder,
        },
        neighborhood::{
            explorers::Finite,
            local_search::{LocalSearch, SteepestAscent},
        },
    },
};
use problem::{Tsp, TspSolution};

use rand::{thread_rng, Rng};

use crate::genetic_decoder::TspDecoder;

const CITIES: usize = 100;

pub mod genetic_decoder;
pub mod neighborhood;
pub mod problem;

#[allow(clippy::redundant_clone)]
fn main() {
    let tsp = problem_instance();

    let stop_criterion = TimeCriterion::<Tsp>::new(Duration::from_secs(10));

    println!("Neighborhood-based solver.");
    if !neighborhood(&tsp, stop_criterion.clone()).is_valid(&tsp) {
        panic!();
    }

    println!("Genetic Solver");
    if !genetic(&tsp, stop_criterion.clone()).is_valid(&tsp) {
        panic!();
    }
}

fn problem_instance() -> Tsp {
    let v: Vec<usize> = (0..(CITIES * CITIES))
        .map(|_| thread_rng().gen_range(0..100))
        .collect();
    let distances: Array2<usize> = Array2::from_shape_vec((CITIES, CITIES), v).unwrap();
    Tsp { distances }
}

fn neighborhood(tsp: &Tsp, mut stop_criterion: impl StopCriterion<Tsp>) -> TspSolution {
    let mut evaluation = tsp.objective_function(TspSolution {
        cities: (0..CITIES).collect(),
    });
    println!(
        "Initial solution: {:?}, value: {}",
        evaluation.solution(),
        evaluation.value()
    );
    let neighborhood = Finite::new(RandomTwoOpt { rng: thread_rng() }, 100);
    let mut local_search = SteepestAscent::new(neighborhood);

    evaluation = local_search.reach_local_optima(tsp, evaluation, &mut stop_criterion);
    println!(
        "Final solution: {:?}, value: {}",
        evaluation.solution(),
        evaluation.value()
    );

    evaluation.into_solution()
}

#[allow(clippy::redundant_clone)]
fn genetic(tsp: &Tsp, mut stop_criterion: impl StopCriterion<Tsp>) -> TspSolution {
    let tsp_decoder = TspDecoder::new(tsp);
    let mut brkga = Brkga::new(
        &tsp_decoder,
        thread_rng(),
        Params {
            population_size: 100.try_into().unwrap(),
            member_size: CITIES.try_into().unwrap(),
            elites: 20,
            mutants: 30,
            crossover_bias: 0.7,
        },
        RandomMemberBuilder,
    );
    println!("Initial solution: {:?}", brkga.best().value);
    brkga.solve(&mut stop_criterion, &mut EmptyHook);
    println!("Final solution: {:?}", brkga.best().value);

    tsp_decoder.decode(&brkga.best().keys)
}
