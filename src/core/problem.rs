mod evaluation;

pub use evaluation::{compare_values, Comparison, Evaluation};

/// Determines if you want to minimize or maximize the [objective_function][Problem::objective_function]
pub enum Objective {
    /// Minimization
    Min,
    /// Maximization
    Max,
}

/// This trait is a core definition for the library, as every optimization task is related to a [Problem].
///
/// ```
/// # use optimum::core::{Objective, Problem, Evaluation};
///
/// struct Knapsack {
///     max_weight: usize,
///     available_items: Vec<Item>,
/// }
///
/// struct Item {
///     value: usize,
///     weight: usize,
/// }
///
/// impl Problem for Knapsack {
///     /// We want to maximize the value we carry in the knapsack
///     const OBJECTIVE: Objective = Objective::Max;
///     /// Every position `i` of the [Vec] represents if item `i` was chosen
///     type Solution = Vec<bool>;
///     
///     type Value = usize;
///     
///     fn objective_function(&self, solution: Self::Solution) -> Evaluation<Self> {
///         let score = self.available_items
///                         .iter()
///                         .enumerate()
///                         .filter(|&(i, _)| solution[i])
///                         .map(|(_, item)| item.value)
///                         .sum();
///
///         Evaluation::new(solution, score)
///     }
/// }
///
/// ```
///
pub trait Problem {
    /// Defines if the problem seeks Minimization or Maximization of the [objective_function][Self::objective_function].
    const OBJECTIVE: Objective;
    /// Determines the structure of the solution to the [Problem].
    type Solution;
    /// This is how you score the [Solution][Self::Solution]. Usually an integer or real number.
    type Value: Ord + Copy;

    /// Associates a [Value][Self::Value] to each [Solution][Self::Solution] to the [Problem] through an [Evaluation].
    fn objective_function(&self, solution: Self::Solution) -> Evaluation<Self>;
}
