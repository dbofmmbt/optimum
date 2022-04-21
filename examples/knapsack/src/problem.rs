use optimum::core::{Evaluation, Objective, Problem};

#[derive(Debug)]
pub struct Knapsack {
    pub max_weight: usize,
    pub available_items: Vec<Item>,
}

impl Problem for Knapsack {
    /// We want to maximize the value we carry in the knapsack
    const OBJECTIVE: Objective = Objective::Max;
    /// Every position `i` of the [Vec] represents if item `i` was chosen
    type Solution = KnapsackSolution;

    type Value = usize;

    fn objective_function(&self, solution: Self::Solution) -> Evaluation<Self> {
        let score = self
            .available_items
            .iter()
            .enumerate()
            .filter(|&(i, _)| solution.choices[i])
            .map(|(_, item)| item.value)
            .sum();

        Evaluation::new(solution, score)
    }
}

#[derive(Debug)]
pub struct Item {
    pub value: usize,
    pub weight: usize,
}

#[derive(Debug)]
pub struct KnapsackSolution {
    pub choices: Vec<bool>,
    pub total_weight: usize,
}
