#![allow(missing_docs)]

use super::{compare_values, Comparison, Evaluation, Problem};

pub trait Move<P: Problem> {
    fn apply(&self, problem: &P, evaluation: Evaluation<P>) -> Evaluation<P>;

    fn value(&self, problem: &P, evaluation: &Evaluation<P>) -> P::Value;

    fn compare(&self, problem: &P, evaluation: &Evaluation<P>) -> Comparison {
        let neighbor_value = self.value(problem, evaluation);
        compare_values::<P>(neighbor_value, evaluation.value())
    }
}

pub mod explorers;
