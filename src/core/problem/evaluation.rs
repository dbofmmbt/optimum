use std::{cmp::Ordering, fmt::Debug};

use super::{Objective, Problem};

/// Represents the association of a [Solution][Problem::Solution] with a [Value][Problem::Value] through the [objective function][Problem::objective_function].
pub struct Evaluation<P: Problem + ?Sized> {
    value: P::Value,
    solution: P::Solution,
}

impl<P: Problem + ?Sized> Debug for Evaluation<P>
where
    P::Solution: Debug,
    P::Value: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Evaluation")
            .field("value", &self.value)
            .field("solution", &self.solution)
            .finish()
    }
}

impl<P: Problem + ?Sized> PartialEq for Evaluation<P>
where
    P::Solution: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.solution == other.solution
    }
}

impl<P: Problem> Evaluation<P> {
    /// Creates a new evaluation, associating a solution with a value for a particular problem.
    pub fn new(solution: P::Solution, value: P::Value) -> Self {
        Self { solution, value }
    }

    /// Get a reference to the evaluation's solution.
    pub fn solution(&self) -> &P::Solution {
        &self.solution
    }

    /// Gives ownership of the solution.
    pub fn into_solution(self) -> P::Solution {
        self.solution
    }

    /// Get a reference to the evaluation's value.
    pub fn value(&self) -> <P as Problem>::Value {
        self.value
    }

    /// Used to define if `self` is better, equal or worse than `other`.
    pub fn compare(&self, other: &Self) -> Comparison {
        self.compare_value(other.value)
    }

    /// Compare `self` with the `other` value directly.
    pub fn compare_value(&self, other: P::Value) -> Comparison {
        compare_values::<P>(self.value, other)
    }
}

/// Compare `a` with `b` value directly.
pub fn compare_values<P: Problem>(a: P::Value, b: P::Value) -> Comparison {
    match P::OBJECTIVE {
        Objective::Min => match a.cmp(&b) {
            Ordering::Less => Comparison::Better,
            Ordering::Equal => Comparison::Equal,
            Ordering::Greater => Comparison::Worse,
        },
        Objective::Max => match a.cmp(&b) {
            Ordering::Less => Comparison::Worse,
            Ordering::Equal => Comparison::Equal,
            Ordering::Greater => Comparison::Better,
        },
    }
}

/// Defines a quality based comparison between two [Evaluation]s.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparison {
    /// The first is better than the second.
    Better,
    /// Both are equally valuable.
    Equal,
    /// The first is worse than the second.
    Worse,
}
