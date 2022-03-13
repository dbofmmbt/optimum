use std::time::Duration;

use optimum::core::{
    neighborhood::{Move, Neighborhood},
    Evaluation, Objective, Problem,
};
use ordered_float::NotNan;
use rand::{thread_rng, Rng};

#[derive(Debug)]
struct Knapsack {
    max_weight: f64,
    available_items: Vec<Item>,
}

#[derive(Debug)]
struct Item {
    value: f64,
    weight: f64,
}

#[derive(Debug)]
struct KnapsackSolution {
    choices: Vec<bool>,
    total_weight: f64,
}

impl Problem for Knapsack {
    /// We want to maximize the value we carry in the knapsack
    const OBJECTIVE: Objective = Objective::Max;
    /// Every position `i` of the [Vec] represents if item `i` was chosen
    type Solution = KnapsackSolution;

    /// We can't use [f64] directly, because it isn't [Ord].
    type Value = NotNan<f64>;

    fn objective_function(&self, solution: Self::Solution) -> Evaluation<Self> {
        let score = self
            .available_items
            .iter()
            .enumerate()
            .filter(|&(i, _)| solution.choices[i])
            .map(|(_, item)| item.value)
            .sum();
        let value = NotNan::new(score).unwrap();
        Evaluation::new(solution, value)
    }
}

struct RandomFlip<R> {
    rng: R,
    problem_size: usize,
}

#[derive(Debug, Clone, Copy)]
struct RandomFlipMove(usize);

impl RandomFlipMove {
    fn is_valid(&self, problem: &Knapsack, solution: &KnapsackSolution) -> bool {
        let chosen = solution.choices[self.0];
        if chosen {
            return true;
        }

        let weight = problem.available_items[self.0].weight;

        solution.total_weight + weight <= problem.max_weight
    }
}

impl Move<Knapsack> for RandomFlipMove {
    fn apply(&self, problem: &Knapsack, evaluation: Evaluation<Knapsack>) -> Evaluation<Knapsack> {
        let mut value = evaluation.value();
        let mut solution = evaluation.into_solution();

        let chosen = &mut solution.choices[self.0];

        *chosen = !*chosen;

        let item = &problem.available_items[self.0];

        if *chosen {
            solution.total_weight += item.weight;
            value += item.value;
        } else {
            solution.total_weight -= item.weight;
            value -= item.value;
        }

        Evaluation::new(solution, value)
    }

    fn value(
        &self,
        problem: &Knapsack,
        evaluation: &Evaluation<Knapsack>,
    ) -> <Knapsack as Problem>::Value {
        let chosen = !evaluation.solution().choices[self.0];

        let item = &problem.available_items[self.0];

        if chosen {
            evaluation.value() + item.value
        } else {
            evaluation.value() - item.value
        }
    }
}

impl<R: Rng> Neighborhood<Knapsack> for RandomFlip<R> {
    type Move = RandomFlipMove;
}

impl<R: Rng> Iterator for RandomFlip<R> {
    type Item = RandomFlipMove;

    fn next(&mut self) -> Option<Self::Item> {
        Some(RandomFlipMove(self.rng.gen_range(0..self.problem_size)))
    }
}

fn main() {
    let problem = Knapsack {
        max_weight: 10.0,
        available_items: vec![
            Item {
                value: 20.0,
                weight: 2.0,
            },
            Item {
                value: 40.0,
                weight: 4.0,
            },
            Item {
                value: 200.0,
                weight: 20.0,
            },
            Item {
                value: 50.0,
                weight: 5.0,
            },
        ],
    };

    let solution = KnapsackSolution {
        choices: vec![false; problem.available_items.len()],
        total_weight: 0.0,
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

    loop {
        let r#move = neighborhood.next().unwrap();

        if !r#move.is_valid(&problem, evaluation.solution()) {
            continue;
        }

        evaluation = r#move.apply(&problem, evaluation);
        std::thread::sleep(Duration::from_secs(2));
        dbg!(&evaluation);
    }
}
