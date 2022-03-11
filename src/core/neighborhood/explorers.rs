use std::marker::PhantomData;

use crate::core::{compare_values, neighborhood::Neighborhood, Comparison, Problem};

use super::Move;

pub struct FirstImprovement<'n, 's, P: Problem, N: Neighborhood<P>> {
    neighborhood: &'n mut N,
    solution: &'s P::Solution,
    p: PhantomData<P>,
}

impl<'n, 's, P: Problem, N: Neighborhood<P>> FirstImprovement<'n, 's, P, N> {
    pub fn new(neighborhood: &'n mut N, solution: &'s P::Solution) -> Self {
        Self {
            neighborhood,
            solution,
            p: PhantomData,
        }
    }
}

impl<'n, 's, P, N> Iterator for FirstImprovement<'n, 's, P, N>
where
    P: Problem,
    N: Neighborhood<P>,
{
    type Item = N::Move;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let r#move = self.neighborhood.next()?;

            if r#move.compare(self.solution) == Comparison::Better {
                return Some(r#move);
            }
        }
    }
}

pub struct BestImprovement<'n, 's, P: Problem, N: Neighborhood<P>> {
    neighborhood: &'n mut N,
    solution: &'s P::Solution,
    p: PhantomData<P>,
}

impl<'n, 's, P: Problem, N: Neighborhood<P>> BestImprovement<'n, 's, P, N> {
    pub fn new(neighborhood: &'n mut N, solution: &'s P::Solution) -> Self {
        Self {
            neighborhood,
            solution,
            p: PhantomData,
        }
    }
}

impl<'n, 's, P, N> Iterator for BestImprovement<'n, 's, P, N>
where
    P: Problem,
    N: Neighborhood<P>,
{
    type Item = N::Move;

    fn next(&mut self) -> Option<Self::Item> {
        let mut best = self.neighborhood.next()?;
        let s = self.solution;

        for r#move in self.neighborhood.by_ref() {
            if compare_values::<P>(r#move.diff(s), best.diff(s)) == Comparison::Better {
                best = r#move
            }
        }

        // Check if the best move found enhances the solution.
        // When you reach a local optimum, the best neighbor found
        // isn't better than the current solution.
        if best.compare(s) == Comparison::Better {
            Some(best)
        } else {
            None
        }
    }
}
