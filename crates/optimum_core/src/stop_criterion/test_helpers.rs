use crate::{Evaluation, Objective, Problem};

impl Problem for () {
    const OBJECTIVE: Objective = Objective::Min;

    type Solution = ();

    type Value = usize;

    fn objective_function(&self, _: Self::Solution) -> Evaluation<Self> {
        Evaluation::new((), 0)
    }
}
