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
pub struct Batch<P, B, S, SC, H>
where
    P: Problem,
    SC: StopCriterion<P> + Clone,
    B: FnMut(usize, usize) -> S,
    S: Solver<SC, H, P = P>,
    H: solver::IterHook<P> + Clone,
{
    base_seed: usize,
    executions: usize,
    solver: B,
    stop_criterion: SC,
    hook: H,
}

impl<P, B, S, SC, H> Batch<P, B, S, SC, H>
where
    P: Problem,
    SC: StopCriterion<P> + Clone,
    B: FnMut(usize, usize) -> S,
    S: Solver<SC, H, P = P>,
    H: solver::IterHook<P> + Clone,
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
    pub fn run(mut self) -> Option<BatchResult<P, H>> {
        let executions = (1..=self.executions as usize)
            .flat_map(|exec_number| {
                let start = Instant::now();

                let mut hook = self.hook.clone();
                let evaluation = {
                    let mut solver = (self.solver)(self.base_seed, exec_number);
                    solver.solve(&mut self.stop_criterion.clone(), &mut hook)?
                };

                let duration = start.elapsed();

                Some(Execution {
                    number: exec_number,
                    evaluation,
                    duration,
                    hook,
                })
            })
            .collect::<Vec<_>>();

        if executions.is_empty() {
            None
        } else {
            Some(BatchResult {
                executions,
                base_seed: self.base_seed,
            })
        }
    }
}

/// The results obtained after running a [Batch].
pub struct BatchResult<P: Problem, H> {
    base_seed: usize,
    executions: Vec<Execution<P, H>>,
}

/// an `Execution` contains information about a full execution on a batch,
/// such as best evaluation obtained, its number and duration. It's also possible
/// to obtain e.g. metadata from the [solver::IterHook] used during that execution.
pub struct Execution<P: Problem, H> {
    number: usize,
    evaluation: Evaluation<P>,
    duration: Duration,
    hook: H,
}

impl<P: Problem, H> Execution<P, H> {
    /// Get a reference to the execution's number.
    pub fn number(&self) -> usize {
        self.number
    }

    /// Get a reference to the execution's evaluation.
    pub fn evaluation(&self) -> &Evaluation<P> {
        &self.evaluation
    }

    /// Get a reference to the execution's duration.
    pub fn duration(&self) -> Duration {
        self.duration
    }

    /// Get a reference to the execution's hook.
    pub fn hook(&self) -> &H {
        &self.hook
    }
}

impl<P, H> Debug for Execution<P, H>
where
    P::Value: Debug,
    P: Problem,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Execution")
            .field("number", &self.number)
            .field("solution value", &self.evaluation.value())
            .field("duration", &self.duration)
            .finish()
    }
}

impl<P: Problem, H> BatchResult<P, H> {
    /// Get a reference to the batch's evaluations, which are the best solutions for each execution.
    pub fn executions(&self) -> &[Execution<P, H>] {
        &self.executions
    }

    /// Get the base seed used by the executions.
    pub fn base_seed(&self) -> usize {
        self.base_seed
    }
}

impl<P, H> Debug for BatchResult<P, H>
where
    P: Problem + Debug,
    P::Solution: Debug,
    P::Value: Debug,
    H: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BatchResult")
            .field("base seed", &self.base_seed)
            .field("evaluations", &self.executions)
            .finish()
    }
}
