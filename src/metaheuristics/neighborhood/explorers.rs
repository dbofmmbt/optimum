use std::marker::PhantomData;

use crate::core::{compare_values, Comparison, Evaluation, Problem};

use super::Move;

pub struct FirstImprovement<'n, 'p, P: Problem, N> {
    neighborhood: &'n mut N,
    problem: &'p P,
    evaluation: &'p Evaluation<P>,
}

impl<'n, 'p, P: Problem, N> FirstImprovement<'n, 'p, P, N> {
    pub fn new(problem: &'p P, neighborhood: &'n mut N, evaluation: &'p Evaluation<P>) -> Self {
        Self {
            problem,
            neighborhood,
            evaluation,
        }
    }
}

impl<'n, 'p, P, N> Iterator for FirstImprovement<'n, 'p, P, N>
where
    P: Problem,
    N: Iterator,
    N::Item: Move<P>,
{
    type Item = N::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let r#move = self.neighborhood.next()?;

            if r#move.compare(self.problem, self.evaluation) == Comparison::Better {
                return Some(r#move);
            }
        }
    }
}

pub struct BestImprovement<'n, 'p, P: Problem, N> {
    neighborhood: &'n mut N,
    problem: &'p P,
    evaluation: &'p Evaluation<P>,
}

impl<'n, 'p, P: Problem, N> BestImprovement<'n, 'p, P, N> {
    pub fn new(problem: &'p P, neighborhood: &'n mut N, evaluation: &'p Evaluation<P>) -> Self {
        Self {
            problem,
            neighborhood,
            evaluation,
        }
    }
}

impl<'n, 'p, P, N> Iterator for BestImprovement<'n, 'p, P, N>
where
    P: Problem,
    N: Iterator,
    N::Item: Move<P>,
{
    type Item = N::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut best = self.neighborhood.next()?;
        let e = self.evaluation;
        let p = self.problem;

        for r#move in self.neighborhood.by_ref() {
            if compare_values::<P>(r#move.value(p, e), best.value(p, e)) == Comparison::Better {
                best = r#move
            }
        }

        // Check if the best move found enhances the solution.
        // When you reach a local optimum, the best neighbor found
        // isn't better than the current solution.
        if best.compare(p, e) == Comparison::Better {
            Some(best)
        } else {
            None
        }
    }
}

pub struct Finite<P, N> {
    _phantom: PhantomData<P>,
    inner: N,
    limit: usize,
    current: usize,
}

impl<P: Problem, N> Finite<P, N> {
    pub fn new(neighborhood: N, limit: usize) -> Self {
        Finite {
            _phantom: PhantomData,
            inner: neighborhood,
            limit,
            current: 0,
        }
    }
}

impl<P, N> Iterator for Finite<P, N>
where
    P: Problem,
    N: Iterator,
    N::Item: Move<P>,
{
    type Item = <N as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.limit {
            None
        } else {
            self.current += 1;
            self.inner.next()
        }
    }
}
