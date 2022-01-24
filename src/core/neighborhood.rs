use std::marker::PhantomData;

use super::{compare_values, Comparison, Problem};

pub trait Neighborhood<P: Problem>: Iterator<Item = Self::Move> {
    type Move: Move<P>;

    fn new(solution: &P::Solution) -> Self;
}

pub trait Move<P: Problem> {
    fn compare(&self) -> Comparison;

    fn diff(&self) -> P::Value;

    fn apply(self, solution: P::Solution) -> P::Solution;
}

pub struct FirstImprovement<'a, P: Problem, N: Neighborhood<P>> {
    neighborhood: &'a mut N,
    p: PhantomData<P>,
}

impl<'a, P: Problem, N: Neighborhood<P>> FirstImprovement<'a, P, N> {
    pub fn new(neighborhood: &'a mut N) -> Self {
        Self {
            neighborhood,
            p: PhantomData,
        }
    }
}

impl<'a, P, N> Iterator for FirstImprovement<'a, P, N>
where
    P: Problem,
    N: Neighborhood<P>,
{
    type Item = N::Move;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let r#move = self.neighborhood.next()?;

            if r#move.compare() == Comparison::Better {
                return Some(r#move);
            }
        }
    }
}

pub struct BestImprovement<'a, P: Problem, N: Neighborhood<P>> {
    neighborhood: &'a mut N,
    p: PhantomData<P>,
}

impl<'a, P: Problem, N: Neighborhood<P>> BestImprovement<'a, P, N> {
    pub fn new(neighborhood: &'a mut N) -> Self {
        Self {
            neighborhood,
            p: PhantomData,
        }
    }
}

impl<'a, P, N> Iterator for BestImprovement<'a, P, N>
where
    P: Problem,
    N: Neighborhood<P>,
{
    type Item = N::Move;

    fn next(&mut self) -> Option<Self::Item> {
        let mut best = self.neighborhood.next()?;

        for r#move in self.neighborhood.by_ref() {
            if compare_values::<P>(r#move.diff(), best.diff()) == Comparison::Better {
                best = r#move
            }
        }

        // Check if the best move found enhances the solution.
        // When you reach a local optimum, the best neighbor found
        // isn't better than the current solution.
        if best.compare() == Comparison::Better {
            Some(best)
        } else {
            None
        }
    }
}
