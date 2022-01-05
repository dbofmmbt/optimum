use crate::core::{Objective, Problem};

impl Problem for () {
    const OBJECTIVE: Objective = Objective::Min;

    type Solution = ();

    type Value = usize;

    fn objective_function(&self, _: &Self::Solution) -> Self::Value {
        0
    }
}
