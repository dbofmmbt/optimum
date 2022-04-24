use std::marker::PhantomData;

use crate::core::{compare_values, Comparison, Evaluation, Problem};

use super::{Move, Neighborhood};

pub struct FirstImprovement<P, N> {
    neighborhood: N,
    _p: PhantomData<P>,
}

impl<P: Problem, N: Neighborhood<P>> FirstImprovement<P, N> {
    pub fn new(neighborhood: N) -> Self {
        Self {
            neighborhood,
            _p: PhantomData,
        }
    }
}

impl<P, N> Neighborhood<P> for FirstImprovement<P, N>
where
    P: Problem,
    N: Neighborhood<P>,
{
    type Move = N::Move;

    fn next_neighbor(&mut self, problem: &P, evaluation: &Evaluation<P>) -> Option<Self::Move> {
        loop {
            let r#move = self.neighborhood.next_neighbor(problem, evaluation)?;

            if r#move.compare(problem, evaluation) == Comparison::Better {
                return Some(r#move);
            }
        }
    }

    fn solution_changed(&mut self, evaluation: &Evaluation<P>) {
        self.neighborhood.solution_changed(evaluation)
    }
}

impl<P: Problem, N: Neighborhood<P>> From<N> for FirstImprovement<P, N> {
    fn from(neighborhood: N) -> Self {
        Self::new(neighborhood)
    }
}

pub struct BestImprovement<P, N> {
    neighborhood: N,
    _p: PhantomData<P>,
}

impl<P: Problem, N: Neighborhood<P>> BestImprovement<P, N> {
    pub fn new(neighborhood: N) -> Self {
        Self {
            neighborhood,
            _p: PhantomData,
        }
    }
}

impl<P, N> Neighborhood<P> for BestImprovement<P, N>
where
    P: Problem,
    N: Neighborhood<P>,
{
    type Move = N::Move;

    fn next_neighbor(&mut self, problem: &P, evaluation: &Evaluation<P>) -> Option<Self::Move> {
        let mut best = self.neighborhood.next_neighbor(problem, evaluation)?;
        let e = evaluation;
        let p = problem;

        while let Some(r#move) = self.neighborhood.next_neighbor(problem, evaluation) {
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

    fn solution_changed(&mut self, evaluation: &Evaluation<P>) {
        self.neighborhood.solution_changed(evaluation)
    }
}

impl<P: Problem, N: Neighborhood<P>> From<N> for BestImprovement<P, N> {
    fn from(neighborhood: N) -> Self {
        Self::new(neighborhood)
    }
}

/// Neighborhood adapter to give bounds to an infinite neighborhood. Usually used when you have stochastic neighborhoods i.e. which generates their moves randomly.
pub struct Finite<P, N> {
    _phantom: PhantomData<P>,
    neighborhood: N,
    limit: usize,
    current: usize,
}

impl<P: Problem, N> Finite<P, N> {
    pub fn new(neighborhood: N, limit: usize) -> Self {
        Finite {
            _phantom: PhantomData,
            neighborhood,
            limit,
            current: 0,
        }
    }
}

impl<P, N> Neighborhood<P> for Finite<P, N>
where
    P: Problem,
    N: Neighborhood<P>,
{
    type Move = N::Move;

    fn next_neighbor(&mut self, problem: &P, evaluation: &Evaluation<P>) -> Option<Self::Move> {
        if self.current == self.limit {
            None
        } else {
            self.current += 1;
            self.neighborhood.next_neighbor(problem, evaluation)
        }
    }

    fn solution_changed(&mut self, evaluation: &Evaluation<P>) {
        self.current = 0;
        self.neighborhood.solution_changed(evaluation)
    }
}
