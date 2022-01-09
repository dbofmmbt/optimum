mod evaluation;

pub use evaluation::{Comparison, Evaluation};

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
/// # use ordered_float::NotNan;
/// # use optimum::core::{Objective, Problem, Evaluation};
///
/// struct Knapsack {
///     max_weight: f64,
///     available_items: Vec<Item>,
/// }
///
/// struct Item {
///     value: f64,
///     weight: f64,
/// }
///
/// impl Problem for Knapsack {
///     /// We want to maximize the value we carry in the knapsack
///     const OBJECTIVE: Objective = Objective::Max;
///     /// Every position `i` of the [Vec] represents if item `i` was chosen
///     type Solution = Vec<bool>;
///     
///     /// We can't use [f64] directly, because it isn't [Ord].
///     type Value = NotNan<f64>;
///     
///     
///     fn objective_function(&self, solution: Self::Solution) -> Evaluation<Self> {
///         let score = self.available_items
///                         .iter()
///                         .enumerate()
///                         .filter(|&(i, _)| solution[i])
///                         .map(|(_, item)| item.value)
///                         .sum();
///         let value = NotNan::new(score).unwrap();
///
///         Evaluation::new(solution, value)
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
