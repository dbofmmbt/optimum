use core::fmt::Debug;
use std::{slice, vec};

use itertools::Itertools;

pub trait Score {
    type Value: Ord;

    fn score(&self) -> Self::Value;
}

#[derive(Debug)]
pub struct EliteSet<T: Score> {
    elements: Vec<T>,
    worse_score: T::Value,
}

impl<T: Score + PartialEq> EliteSet<T> {
    pub fn new(size: usize, initial_threshold: T::Value) -> Self {
        assert!(size > 0, "Elite set size must be greater than zero");

        Self {
            elements: Vec::with_capacity(size),
            worse_score: initial_threshold,
        }
    }

    pub fn try_insert(&mut self, candidate: T) -> Result<(), T> {
        if candidate.score() < self.worse_score {
            return Err(candidate);
        }

        if self.elements.iter().any(|existing| existing == &candidate) {
            return Err(candidate);
        }

        self.insert(candidate);
        Ok(())
    }

    fn insert(&mut self, new: T) {
        if self.elements.len() == self.elements.capacity() {
            // Safe to unwrap: `len` can't be zero.
            let min_position = self.min_position().unwrap();

            self.elements[min_position] = new;
            self.update_worse_value();
        } else {
            self.elements.push(new);

            if self.elements.len() == self.elements.capacity() {
                self.update_worse_value();
            }
        }
    }

    fn min_position(&self) -> Option<usize> {
        self.elements.iter().position_min_by_key(|it| it.score())
    }

    fn update_worse_value(&mut self) {
        let position = self.min_position().unwrap();
        let worse_solution = &self.elements[position];
        self.worse_score = worse_solution.score();
    }

    pub fn best(&self) -> Option<&T> {
        self.elements.iter().max_by_key(|it| it.score())
    }

    pub fn worse(&self) -> Option<&T> {
        self.min_position().map(|pos| &self.elements[pos])
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.into_iter()
    }
}

pub struct EliteIter<'a, T: Score> {
    elite: &'a EliteSet<T>,
    position: usize,
}

impl<'a, T: Score> Iterator for EliteIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.position += 1;
        self.elite.elements.get(self.position - 1)
    }
}

impl<'a, T: Score> IntoIterator for &'a EliteSet<T> {
    type Item = &'a T;

    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

impl<T: Score> IntoIterator for EliteSet<T> {
    type Item = T;

    type IntoIter = vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Score for isize {
        type Value = Self;

        fn score(&self) -> Self::Value {
            *self
        }
    }

    #[test]
    fn basic_operations_work() {
        let mut elite = EliteSet::<isize>::new(3, isize::MIN);

        let successful_insertions = [5, 8, 7, 6];

        for el in successful_insertions {
            assert_eq!(elite.try_insert(el), Ok(()));
        }

        assert_eq!(elite.try_insert(4), Err(4));

        assert_eq!(*elite.best().unwrap(), 8);
        assert_eq!(*elite.worse().unwrap(), 6);
    }

    #[test]
    fn iterator_works() {
        let insertions = [1, 2, 3];
        let mut elite = EliteSet::<isize>::new(3, isize::MIN);

        for el in insertions {
            elite.try_insert(el).unwrap();
        }

        assert_eq!(elite.into_iter().collect::<Vec<_>>(), insertions);
    }
}
