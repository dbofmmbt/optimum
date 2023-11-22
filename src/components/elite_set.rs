use core::fmt::Debug;
use std::{slice, vec};

use crate::core::{Comparison, Evaluation, Problem};

pub struct EliteSet<P: Problem> {
    elements: Vec<Evaluation<P>>,
    worse_value: P::Value,
}

impl<P: Problem> Debug for EliteSet<P>
where
    P::Solution: Debug,
    P::Value: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EliteSet")
            .field("elements", &self.elements)
            .field("worse_score", &self.worse_value)
            .finish()
    }
}

impl<P: Problem> EliteSet<P>
where
    P::Solution: PartialEq,
{
    pub fn new(size: usize, initial_threshold: P::Value) -> Self {
        assert!(size > 0, "Elite set size must be greater than zero");

        Self {
            elements: Vec::with_capacity(size),
            worse_value: initial_threshold,
        }
    }

    pub fn try_insert(&mut self, candidate: Evaluation<P>) -> Result<(), Evaluation<P>> {
        match candidate.compare_value(self.worse_value) {
            Comparison::Better => (),
            _ => return Err(candidate),
        }

        if self.elements.iter().any(|existing| existing == &candidate) {
            return Err(candidate);
        }

        self.insert(candidate);
        Ok(())
    }

    fn insert(&mut self, new: Evaluation<P>) {
        if self.elements.len() == self.elements.capacity() {
            // Safe to unwrap: `len` can't be zero.
            let min_position = self.worse_position().unwrap();

            self.elements[min_position] = new;
            self.update_worse_value();
        } else {
            self.elements.push(new);

            if self.elements.len() == self.elements.capacity() {
                self.update_worse_value();
            }
        }
    }

    fn worse_position(&self) -> Option<usize> {
        if self.elements.is_empty() {
            return None;
        }

        let mut position = 0;
        for (idx, element) in self.elements.iter().enumerate().skip(1) {
            if let Comparison::Worse = element.compare(&self.elements[position]) {
                position = idx;
            }
        }

        Some(position)
    }

    fn update_worse_value(&mut self) {
        let position = self.worse_position().unwrap();
        let worse_evaluation = &self.elements[position];
        self.worse_value = worse_evaluation.value();
    }

    pub fn best(&self) -> Option<&Evaluation<P>> {
        self.elements.iter().reduce(|a, b| match a.compare(b) {
            Comparison::Better => a,
            _ => b,
        })
    }

    pub fn worse(&self) -> Option<&Evaluation<P>> {
        self.worse_position().map(|pos| &self.elements[pos])
    }

    pub fn iter(&self) -> impl Iterator<Item = &Evaluation<P>> {
        self.into_iter()
    }
}

pub struct EliteIter<'a, P: Problem> {
    elite: &'a EliteSet<P>,
    position: usize,
}

impl<'a, P: Problem> Iterator for EliteIter<'a, P> {
    type Item = &'a Evaluation<P>;

    fn next(&mut self) -> Option<Self::Item> {
        self.position += 1;
        self.elite.elements.get(self.position - 1)
    }
}

impl<'a, P: Problem> IntoIterator for &'a EliteSet<P> {
    type Item = &'a Evaluation<P>;

    type IntoIter = slice::Iter<'a, Evaluation<P>>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

impl<P: Problem> IntoIterator for EliteSet<P> {
    type Item = Evaluation<P>;

    type IntoIter = vec::IntoIter<Evaluation<P>>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Objective;

    use super::*;

    struct Test;

    impl Problem for Test {
        type Solution = usize;

        const OBJECTIVE: Objective = Objective::Max;

        type Value = usize;

        fn objective_function(&self, solution: Self::Solution) -> Evaluation<Self> {
            Evaluation::new(solution, solution)
        }
    }

    fn insert_helper(elite: &mut EliteSet<Test>, number: usize) -> Result<(), usize> {
        elite
            .try_insert(Evaluation::new(number, number))
            .map_err(|e| e.value())
    }

    #[test]
    fn basic_operations_work() {
        let mut elite = EliteSet::<Test>::new(3, usize::MIN);

        let successful_insertions = [5, 8, 7, 6];

        for el in successful_insertions {
            assert_eq!(insert_helper(&mut elite, el), Ok(()));
        }

        dbg!(&elite);

        assert_eq!(insert_helper(&mut elite, 4), Err(4));

        assert_eq!(elite.best().unwrap().value(), 8);
        assert_eq!(elite.worse().unwrap().value(), 6);
    }

    #[test]
    fn iterator_works() {
        let insertions = [1, 2, 3];
        let mut elite = EliteSet::new(3, usize::MIN);

        for el in insertions {
            insert_helper(&mut elite, el).unwrap();
        }

        assert_eq!(
            elite.into_iter().map(|i| i.value()).collect::<Vec<_>>(),
            insertions
        );
    }
}
