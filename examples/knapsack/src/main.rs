use std::time::Duration;

use optimum::{core::Problem, metaheuristics::neighborhood::Move};
use rand::thread_rng;

use crate::{
    neighborhood::RandomFlip,
    problem::{Item, Knapsack, KnapsackSolution},
};

mod neighborhood;
mod problem;

fn main() {
    let problem = Knapsack {
        max_weight: 10,
        available_items: vec![
            Item {
                value: 20,
                weight: 2,
            },
            Item {
                value: 40,
                weight: 4,
            },
            Item {
                value: 200,
                weight: 20,
            },
            Item {
                value: 50,
                weight: 5,
            },
        ],
    };

    let solution = KnapsackSolution {
        choices: vec![false; problem.available_items.len()],
        total_weight: 0,
    };

    let mut evaluation = problem.objective_function(solution);

    let rng = thread_rng();
    let mut neighborhood = RandomFlip {
        problem_size: problem.available_items.len(),
        rng,
    };

    let r#move = neighborhood.next().unwrap();

    dbg!(r#move);
    dbg!(&evaluation);

    for r#move in neighborhood {
        if !r#move.is_valid(&problem, evaluation.solution()) {
            continue;
        }

        evaluation = r#move.apply(&problem, evaluation);
        std::thread::sleep(Duration::from_secs(1));
        dbg!(&evaluation);
    }
}
