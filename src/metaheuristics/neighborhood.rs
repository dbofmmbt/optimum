#![allow(missing_docs)]

use crate::core::{compare_values, Comparison, Evaluation, Problem};

/// Represents the surround of the current solution. Acts like an iterator of [Move]s to neighbors inside it.
pub trait Neighborhood<P: Problem> {
    type Move: Move<P>;

    fn next_neighbor(&mut self, problem: &P, evaluation: &Evaluation<P>) -> Option<Self::Move>;
}

pub trait Move<P: Problem> {
    fn apply(&self, problem: &P, evaluation: Evaluation<P>) -> Evaluation<P>;

    fn value(&self, problem: &P, evaluation: &Evaluation<P>) -> P::Value;

    fn compare(&self, problem: &P, evaluation: &Evaluation<P>) -> Comparison {
        let neighbor_value = self.value(problem, evaluation);
        compare_values::<P>(neighbor_value, evaluation.value())
    }
}

pub mod explorers;
pub mod local_search;
