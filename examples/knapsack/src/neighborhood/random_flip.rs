use optimum::{
    core::{Evaluation, Problem},
    metaheuristics::neighborhood::Move,
};
use rand::Rng;

use crate::problem::{Knapsack, KnapsackSolution};

pub struct RandomFlip<R> {
    pub rng: R,
    pub problem_size: usize,
}

impl<R: Rng> Iterator for RandomFlip<R> {
    type Item = RandomFlipMove;

    fn next(&mut self) -> Option<Self::Item> {
        Some(RandomFlipMove(self.rng.gen_range(0..self.problem_size)))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RandomFlipMove(usize);

impl RandomFlipMove {
    pub fn is_valid(&self, problem: &Knapsack, solution: &KnapsackSolution) -> bool {
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
