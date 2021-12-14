mod decoder;
mod population;

pub use decoder::Decoder;
use population::Member;
pub use population::{Key, Population};

use std::{num::NonZeroUsize, usize};

use rand::Rng;

use crate::core::Problem;

type Value<D> = <<D as Decoder>::P as Problem>::Value;

pub struct Brkga<'a, D: Decoder, R: Rng> {
    decoder: &'a D,
    rng: R,
    current: Population<D>,
    previous: Population<D>,
    generations: usize,
}

impl<'a, R: Rng, D: Decoder> Brkga<'a, D, R> {
    pub fn new(decoder: &'a D, mut rng: R, params: Params) -> Self {
        let current = Population::new(params, &mut rng, decoder);
        let previous = current.clone();

        Self {
            decoder,
            rng,
            current,
            previous,
            generations: 0,
        }
    }

    pub fn evolve(&mut self) {
        let next = &mut self.previous;

        self.current.transfer_elites(next);
        self.current.crossover(next, &mut self.rng);
        next.mutate(&mut self.rng);

        next.compute_value(self.decoder);

        std::mem::swap(&mut self.current, next);
        self.generations += 1;
    }

    pub fn reset(&mut self) {
        self.generations = 1;
    }

    pub fn current_generation(&self) -> usize {
        self.generations
    }

    pub fn current_population(&self) -> &Population<D> {
        &self.current
    }

    pub fn best(&self) -> &Member<Value<D>> {
        &self.current[0]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Params {
    population_size: NonZeroUsize,
    members: NonZeroUsize,
    elites: usize,
    mutants: usize,
    crossover_bias: f64,
}
