//! This module keeps [Population] and [Member], which may be used for genetic algorithms.

mod member;

use std::marker::PhantomData;

use derive_more::{Index, IndexMut};

use crate::core::{Objective, Problem};
pub use member::Member;

/// A list of members ranked by their value.
#[derive(Debug, Index, IndexMut)]
pub struct Population<P: Problem, G> {
    #[index]
    #[index_mut]
    pub(crate) members: Vec<Member<G, P::Value>>,
    _p: PhantomData<P>,
}

impl<P: Problem, G: Clone> Clone for Population<P, G> {
    fn clone(&self) -> Self {
        Self {
            members: self.members.clone(),
            _p: self._p,
        }
    }
}

impl<P: Problem, G> Population<P, G> {
    /// Generates a new [Population] through the given builder
    pub fn new<F>(size: usize, member_builder: F) -> Self
    where
        F: FnMut(usize) -> Member<G, P::Value>,
    {
        let members = (0..size).map(member_builder).collect::<Vec<_>>();

        let mut population = Self {
            members,
            _p: PhantomData,
        };
        population.sort();
        population
    }

    /// Sorts the population, from the best members to the worst.
    pub fn sort(&mut self) {
        match P::OBJECTIVE {
            Objective::Min => self.members.sort_unstable(),
            Objective::Max => self.members.sort_unstable_by(|a, b| b.cmp(a)),
        }
    }

    /// Number of members of `self`
    pub fn size(&self) -> usize {
        self.members.len()
    }

    /// Number of genes in a member of `self`
    pub fn member_size(&self) -> usize {
        self.members[0].len()
    }
}
