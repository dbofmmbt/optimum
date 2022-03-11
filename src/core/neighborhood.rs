use super::{Comparison, Problem};

pub trait Neighborhood<P: Problem>: Iterator<Item = Self::Move> {
    type Move: Move<P>;
}

// TODO how to evaluate neighbors properly???
pub trait NeighborEvaluator<P: Problem> {
    fn compare(&self, r#move: (), solution: &P::Solution) -> Comparison;

    fn diff(&self, r#move: (), solution: &P::Solution) -> P::Value;
}

pub trait Move<P: Problem> {
    fn apply(&self, solution: P::Solution) -> P::Solution;

    fn reverse(&self) -> Self;
}

pub mod explorers;
