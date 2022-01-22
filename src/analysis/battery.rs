//! Run and analyze multiple executions of a solver.

mod statistics;

pub use statistics::{Gap, Statistics};

use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

use crate::core::{solver, Evaluation, Problem, Solver, StopCriterion};

/// A battery is a sequence of multiple executions of a stochastic solver which is often used to
/// compare the solver's performance across different seed numbers.
pub struct Battery<P: Problem> {
    executions: usize,
    base_seed: usize,
    evaluations: Vec<(usize, Evaluation<P>, Duration)>,
}

impl<P> Debug for Battery<P>
where
    P: Problem + Debug,
    P::Solution: Debug,
    P::Value: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Battery")
            .field("base seed", &self.base_seed)
            .field("executions", &self.executions)
            .field("evaluations", &self.evaluations)
            .finish()
    }
}

impl<P: Problem> Battery<P> {
    /// Runs a new `Battery`.
    ///
    /// ## Params
    ///
    /// - `base_seed`: the seed value passed to the `solver` parameter.
    ///
    /// - `executions`: The number of executions
    ///
    /// - `solver`: this is a way to let you configure/build your solver based on the `base_seed` and the number of the current execution.
    ///     It receives the `base_seed` and the current execution number, which goes from `1..=executions`.  
    ///
    /// - `stop_criterion`: used to control how long the solver will run on each execution.
    ///
    /// ## Example
    ///
    /// ```ignore
    /// let stop_criterion = IterCriterion::new(100);
    /// let battery = Battery::new(
    ///     1,
    ///     10,
    ///     |seed, exec_number| {
    ///         /// ...
    ///         let solver = build_solver(seed, exec_number);
    ///         /// ...
    ///         solver
    ///     }
    ///     &stop_criterion,
    /// );
    /// ```
    pub fn new<B, S, SC, LC>(
        base_seed: usize,
        executions: usize,
        mut solver: B,
        stop_criterion: &SC,
        mut life_cycle: LC,
    ) -> Option<Self>
    where
        SC: StopCriterion<P> + Clone,
        B: FnMut(usize, usize) -> S,
        S: Solver<SC, LC, P = P>,
        LC: solver::LifeCycle<P>,
    {
        let evaluations = (1..=executions as usize)
            .flat_map(|exec_number| {
                let start = Instant::now();

                let evaluation = {
                    let mut solver = solver(base_seed, exec_number);
                    solver.solve(&mut stop_criterion.clone(), &mut life_cycle)?
                };

                let duration = start.elapsed();

                Some((exec_number, evaluation, duration))
            })
            .collect::<Vec<_>>();

        if evaluations.is_empty() {
            None
        } else {
            Some(Self {
                evaluations,
                executions,
                base_seed,
            })
        }
    }

    /// Get a reference to the battery's evaluations.
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
