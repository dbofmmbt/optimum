use ndarray::Array2;
use optimum::core::{Evaluation, Objective, Problem};
use ordered_float::NotNan;

pub mod decoders;

pub struct MaximumDiversity {
    pub matrix: Array2<f64>,
    pub solution_size: usize,
    pub input_size: usize,
}

#[derive(Debug)]
pub struct MdpSolution {
    elements: Vec<usize>,
}

impl Problem for MaximumDiversity {
    const OBJECTIVE: Objective = Objective::Max;

    type Solution = MdpSolution;

    type Value = NotNan<f64>;

    fn objective_function(&self, solution: Self::Solution) -> Evaluation<Self> {
        let mut value = 0.0;
        for (i, a) in solution.elements.iter().copied().enumerate() {
            for b in solution.elements[(i + 1)..].iter().copied() {
                value += self.matrix[[a, b]];
            }
        }
        let value = NotNan::new(value).expect("objective function failed");
        Evaluation::new(solution, value)
    }
}
