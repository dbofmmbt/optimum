//! Defines a `Hook` trait for [Solver][super::Solver] and basic `Hook` structs.
//!
//!  It's highly recommended that you create `Hook` traits for your metaheuristics to allow callers to add custom
//! behavior into your metaheuristic.

use super::super::{Evaluation, Problem};

/// This trait allows callers to hook into special moments in the execution of the `Solver` to do things such as logging.
pub trait IterHook<P: Problem> {
    /// Called right after the iteration is performed. `new` is the newly generated evaluation yield by [iterate][super::Solver::iterate].
    fn iterated(&mut self, _new: &Evaluation<P>) {}

    /// Called when the "global" best is being replaced by a new evaluation.
    fn better_changed(&mut self, _old: &Evaluation<P>, _new: &Evaluation<P>) {}
}

/// It does nothing.
pub struct Empty;

impl<P: Problem> IterHook<P> for Empty {}

/// This hook just prints the iteration's values on stderr.
pub struct Print(usize);

impl Print {
    /// Creates a new `Print` hook.
    pub fn new() -> Self {
        Self(0)
    }
}

impl Default for Print {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: Problem> IterHook<P> for Print
where
    P::Value: std::fmt::Display,
{
    fn iterated(&mut self, new: &Evaluation<P>) {
        self.0 += 1;
        eprintln!("ITER {} VALUE {}", self.0, new.value());
    }
}
