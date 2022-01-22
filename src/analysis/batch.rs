//! Run and analyze multiple executions of a solver.

mod statistics;

pub use statistics::{Gap, Statistics};

use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

use typed_builder::TypedBuilder;

use crate::core::{solver, Evaluation, Problem, Solver, StopCriterion};

/// A batch is a sequence of multiple executions of a stochastic solver which is often used to
/// compare the solver's performance across different seed numbers.
#[derive(Debug, TypedBuilder)]
pub struct Batch<'a, P, B, S, SC, H>
where
    P: Problem,
    SC: StopCriterion<P> + Clone,
    B: FnMut(usize, usize) -> S,
    S: Solver<SC, H, P = P>,
    H: solver::IterHook<P>,
{
    base_seed: usize,
    executions: usize,
    solver: B,
    stop_criterion: &'a SC,
    hook: H,
}

impl<'a, P, B, S, SC, H> Batch<'a, P, B, S, SC, H>
where
    P: Problem,
    SC: StopCriterion<P> + Clone,
    B: FnMut(usize, usize) -> S,
    S: Solver<SC, H, P = P>,
    H: solver::IterHook<P>,
{
    /// Runs a new `Batch`.
    ///
    /// ## Example
    ///
    /// ```ignore
    /// let stop_criterion = IterCriterion::new(100);
    /// let batch = Batch::builder()
    ///     .base_seed(1)
    ///     .executions(10)
    ///     .solver(build_solver)
    ///     .stop_criterion(&stop_criterion)
    ///     .hook(hook::Empty)
    ///     .build()
    ///     .run()
    ///     .unwrap();
    /// ```
    pub fn run(mut self) -> Option<BatchResult<P>> {
        let evaluations = (1..=self.executions as usize)
            .flat_map(|exec_number| {
                let start = Instant::now();

                let evaluation = {
                    let mut solver = (self.solver)(self.base_seed, exec_number);
                    solver.solve(&mut self.stop_criterion.clone(), &mut self.hook)?
                };

                let duration = start.elapsed();

                Some((exec_number, evaluation, duration))
            })
            .collect::<Vec<_>>();

        if evaluations.is_empty() {
            None
        } else {
            Some(BatchResult {
                evaluations,
                executions: self.executions,
                base_seed: self.base_seed,
            })
        }
    }
}

/// The results obtained after running a [Batch].
pub struct BatchResult<P: Problem> {
    executions: usize,
    base_seed: usize,
    evaluations: Vec<(usize, Evaluation<P>, Duration)>,
}

impl<P: Problem> BatchResult<P> {
    /// Get a reference to the batch's evaluations, which are the best solutions for each execution.
    pub fn evaluations(&self) -> &[(usize, Evaluation<P>, Duration)] {
        &self.evaluations
    }

    /// Get the number of executions performed.
    pub fn executions(&self) -> usize {
        self.executions
    }

    /// Get the base seed used by the executions.
    pub fn base_seed(&self) -> usize {
        self.base_seed
    }
}

impl<P> Debug for BatchResult<P>
where
    P: Problem + Debug,
    P::Solution: Debug,
    P::Value: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BatchResult")
            .field("base seed", &self.base_seed)
            .field("executions", &self.executions)
            .field("evaluations", &self.evaluations)
            .finish()
    }
}
