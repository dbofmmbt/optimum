//! # BRKGA
//!
//! This implementation of the Biased Random-Key Genetic Algorithm is a Rust port of the [brkgaAPI](http://mauricio.resende.info/src/brkgaAPI/).
//!
//! The features of _multiple populations_ and _optional parallel decoding_ are not present, but may be added later.
//! The rest should behave the same.
//!
//! The main items here are [Brkga] and its [Params].
//!

pub mod population;

use super::{decoder::Value, Decoder};
use population::{Member, Population};

use std::{num::NonZeroUsize, usize};

use rand::Rng;

/// The interface to execute the BRKGA algorithm.
///

// TODO use a real doctest

/// ```ignore
/// let brkga = Brkga::new(decoder, rng, params);
///
/// println!("Initial value: {}", brkga.best().value);
///
/// // Apply evolutions
/// for _ in 0..100 {
///     brkga.evolve();
/// }
///
/// println!("Final value: {}", brkga.best().value);
/// ```
///
pub struct Brkga<'a, D: Decoder, R: Rng> {
    decoder: &'a D,
    rng: R,
    current: Population<D>,
    previous: Population<D>,
    generations: usize,
}

impl<'a, R: Rng, D: Decoder> Brkga<'a, D, R> {
    /// Creates a new BRKGA instance, which solves the [Problem][crate::core::Problem] defined by the [Decoder].
    pub fn new(decoder: &'a D, mut rng: R, params: Params) -> Self {
        // TODO decouple initial population generation from the method itself to ease reuse
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

    /// Performs the evolution of the population defined by the algorithm.
    ///
    /// 1. Elites are transferred to the next generation.
    /// 2. Crossover is performed between elites and non elites to generate new members.
    /// 3. The worse members are exchanged by mutants.
    ///
    pub fn evolve(&mut self) {
        let next = &mut self.previous;

        self.current.transfer_elites(next);
        self.current.crossover(next, &mut self.rng);
        next.mutate(&mut self.rng);

        next.compute_value(self.decoder);

        std::mem::swap(&mut self.current, next);
        self.generations += 1;
    }

    /// Resets the algorithm.
    pub fn reset(&mut self) {
        // TODO confirm if this function matches the reset on brkgaAPI.
        self.generations = 1;
    }

    /// Returns the number of the current generation.
    pub fn current_generation(&self) -> usize {
        self.generations
    }

    /// Returns the current [Population] held by the algorithm.
    pub fn current_population(&self) -> &Population<D> {
        &self.current
    }

    /// Returns a reference for the best [Member] at this moment.
    pub fn best(&self) -> &Member<Value<D>> {
        &self.current[0]
    }
}

/// The parameters needed to run the BRKGA algorithm
#[derive(Debug, Clone, Copy)]
pub struct Params {
    /// Size of population, which must be greater than 0.
    pub population_size: NonZeroUsize,
    /// Size of a member in a population. The [RandomKey][super::RandomKey] slice will have this size.
    pub member_size: NonZeroUsize,
    /// Number of elites (best solutions) in a population.
    pub elites: usize,
    /// Number of mutants that will be generated from a generation to another.
    pub mutants: usize,
    /// It defines how probable is to choose a gene from the elite parent.
    ///
    /// Should be a value in \[0.5, 1.0\]
    pub crossover_bias: f64,
}
