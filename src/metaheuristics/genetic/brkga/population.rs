mod member;

use std::{
    num::NonZeroUsize,
    ops::{Index, IndexMut, Range},
};

pub use member::{Key, Member};
use rand::{prelude::SliceRandom, Rng};

use crate::core::{Objective, Problem};

use super::{Decoder, Params, Value};

#[derive(Debug)]
pub struct Population<D: Decoder> {
    pub(crate) members: Vec<Member<Value<D>>>,
    params: Params,
}

impl<D: Decoder> Clone for Population<D> {
    fn clone(&self) -> Self {
        Self {
            members: self.members.clone(),
            params: self.params,
        }
    }
}

impl<D: Decoder> Population<D> {
    pub fn new(params: Params, mut rng: impl Rng, decoder: &D) -> Self {
        let members = (0..params.population_size.get())
            .map(|_| {
                let keys = {
                    let mut k = vec![0.0; params.members.get()].into_boxed_slice();
                    rng.fill(k.as_mut());
                    k
                };
                let value = decoder.decode(&keys);
                Member { keys, value }
            })
            .collect::<Vec<_>>();

        let mut population = Self { members, params };
        population.sort();
        population
    }

    fn sort(&mut self) {
        let objective = <D::P as Problem>::OBJECTIVE;

        match objective {
            Objective::Min => self.members.sort_unstable(),
            Objective::Max => self.members.sort_unstable_by(|a, b| b.cmp(a)),
        }
    }

    pub fn size(&self) -> usize {
        self.members.len()
    }

    pub fn member_size(&self) -> NonZeroUsize {
        self.params.members
    }

    fn regular_indices(&self) -> Range<usize> {
        self.params.elites..(self.size() - self.params.mutants)
    }

    fn mutant_indices(&self) -> Range<usize> {
        (self.size() - self.params.mutants)..self.size()
    }

    pub fn elites(&self) -> &[Member<Value<D>>] {
        &self.members[..self.params.elites]
    }

    pub fn not_elites(&self) -> &[Member<Value<D>>] {
        &self.members[self.params.elites..]
    }

    pub fn transfer_elites(&self, next: &mut Self) {
        for (idx, elite) in self.elites().iter().enumerate() {
            let target = &mut next[idx];

            for (idx, gene) in elite.keys.iter().copied().enumerate() {
                target[idx] = gene;
            }
            target.value = elite.value;
        }
    }

    pub fn crossover(&self, next: &mut Self, mut rng: impl Rng) {
        for idx in self.regular_indices() {
            let member = &mut next[idx];

            let elite_parent = self.elites().choose(&mut rng).unwrap();
            let non_elite_parent = self.not_elites().choose(&mut rng).unwrap();

            for gene in 0..self.params.members.get() {
                let source_parent = if rng.gen::<f64>() < self.params.crossover_bias {
                    elite_parent
                } else {
                    non_elite_parent
                };

                member[gene] = source_parent[gene];
            }
        }
    }

    pub fn mutate(&mut self, mut rng: impl Rng) {
        for idx in self.mutant_indices() {
            let mutant = &mut self.members[idx];

            for gene in 0..self.params.members.get() {
                mutant[gene] = rng.gen();
            }
        }
    }

    pub(crate) fn compute_value(&mut self, decoder: &D) {
        for member in self.members.iter_mut() {
            member.value = decoder.decode(&member.keys);
        }

        self.sort();
    }

    pub fn reset(&mut self, mut rng: impl Rng, decoder: &D) {
        for member in self.members.iter_mut() {
            rng.fill(member.keys.as_mut());
        }
        self.compute_value(decoder);
    }
}

impl<D: Decoder> Index<usize> for Population<D> {
    type Output = Member<Value<D>>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.members[index]
    }
}

impl<D: Decoder> IndexMut<usize> for Population<D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.members[index]
    }
}
