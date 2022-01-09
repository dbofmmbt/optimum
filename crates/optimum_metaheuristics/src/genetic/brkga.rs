//! # BRKGA
//!
//! This implementation of the Biased Random-Key Genetic Algorithm is a Rust port of the [brkgaAPI](http://mauricio.resende.info/src/brkgaAPI/).
//!
//! The features of _multiple populations_ and _optional parallel decoding_ are not present, but may be added later.
//! The rest should behave the same.
//!
//! The main items here are [Brkga] and its [Params].
//!

use optimum_core::{
    Problem, StopCriterion, {Evaluation, Solver},
};

use super::{
    population::{Member, Population},
    Decoder, RandomKey,
};

use std::{num::NonZeroUsize, usize};

use rand::{prelude::SliceRandom, Rng};

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
    current: BrkgaPopulation<D>,
    next: BrkgaPopulation<D>,
    generations: usize,
    params: Params,
}

/// The population type used by BRKGA
pub type BrkgaPopulation<D> = Population<<D as Decoder>::P, RandomKey>;

/// The member type used by BRKGA
pub type BrkgaMember<D> = Member<RandomKey, <<D as Decoder>::P as Problem>::Value>;

impl<'a, R: Rng, D: Decoder> Brkga<'a, D, R> {
    /// Creates a new BRKGA instance, which solves the [Problem][optimum_core::Problem] defined by the [Decoder].
    pub fn new(decoder: &'a D, mut rng: R, params: Params) -> Self {
        // TODO allow caller to pass a custom_builder
        let random_member_builder = |_| {
            let keys = {
                let mut k = vec![0.0; params.member_size.get()].into_boxed_slice();
                rng.fill(k.as_mut());
                k
            };

            let value = decoder.decode_value(&keys);
            Member { keys, value }
        };

        let current = Population::new(params.population_size.get(), random_member_builder);
        let next = current.clone();

        Self {
            current,
            decoder,
            params,
            next,
            generations: 0,
            rng,
        }
    }

    /// Performs the evolution of the population defined by the algorithm.
    ///
    /// 1. Elites are transferred to the next generation.
    /// 2. Crossover is performed between elites and non elites to generate new members.
    /// 3. The worse members are exchanged by mutants.
    ///
    pub fn evolve(&mut self) {
        self.transfer_elites();
        self.crossover();
        std::mem::swap(&mut self.current, &mut self.next);
        self.mutate_current();
        self.recompute_current();

        self.generations += 1;
    }

    /// Copy the elites from `current` to `next`.
    fn transfer_elites(&mut self) {
        let elites = Self::elites(&self.current, &self.params);

        for (elite, target) in elites.iter().zip(self.next.members.iter_mut()) {
            target.keys.copy_from_slice(&elite.keys);
            target.value = elite.value;
        }
    }

    /// Performs the crossover operation to a new generation.
    ///
    /// Until all necessary chields are created,
    /// it selects an elite parent and a non elite parent and generates a child
    /// by randomly choosing which key comes from which parent
    /// based on the crossover bias parameter.
    fn crossover(&mut self) {
        for member in Self::regulars(&mut self.next, &self.params) {
            let elite_parent = Self::elites(&self.current, &self.params)
                .choose(&mut self.rng)
                .unwrap();
            let non_elite_parent = Self::not_elites(&self.current, &self.params)
                .choose(&mut self.rng)
                .unwrap();

            for gene in 0..self.params.member_size.get() {
                let source_parent = if self.rng.gen::<f64>() < self.params.crossover_bias {
                    elite_parent
                } else {
                    non_elite_parent
                };

                member[gene] = source_parent[gene];
            }
        }
    }

    /// Substitute the worse members for randomly generated mutants.
    fn mutate_current(&mut self) {
        for mutant in Self::mutants(&mut self.current, &self.params) {
            self.rng.fill(mutant.keys.as_mut());
        }
    }

    fn recompute_current(&mut self) {
        for member in self.current.members.iter_mut() {
            member.value = self.decoder.decode_value(&member.keys);
        }

        self.current.sort();
    }

    /// Returns the number of the current generation.
    pub fn current_generation(&self) -> usize {
        self.generations
    }

    /// Returns the current [Population] held by the algorithm.
    pub fn current_population(&self) -> &BrkgaPopulation<D> {
        &self.current
    }

    /// Returns a reference for the best [Member] at this moment.
    pub fn best(&self) -> &BrkgaMember<D> {
        &self.current[0]
    }

    fn regulars<'b>(
        population: &'b mut BrkgaPopulation<D>,
        p: &Params,
    ) -> &'b mut [BrkgaMember<D>] {
        let regulars = { p.elites..(p.population_size.get() - p.mutants) };
        &mut population[regulars]
    }

    fn mutants<'b>(population: &'b mut BrkgaPopulation<D>, p: &Params) -> &'b mut [BrkgaMember<D>] {
        let mutants = (p.population_size.get() - p.mutants)..p.population_size.get();
        &mut population[mutants]
    }

    /// A slice with the best [Member]s.
    fn elites<'b>(population: &'b BrkgaPopulation<D>, p: &Params) -> &'b [BrkgaMember<D>] {
        &population[..p.elites]
    }

    /// A slice with the [Member]s which aren't elites.
    fn not_elites<'b>(population: &'b BrkgaPopulation<D>, p: &Params) -> &'b [BrkgaMember<D>] {
        &population[p.elites..]
    }
}

impl<'a, D, R, SC> Solver<SC> for Brkga<'a, D, R>
where
    D: Decoder,
    R: Rng,
    SC: StopCriterion<D::P>,
{
    type P = D::P;

    fn iterate(&mut self, _: &mut SC) -> Option<Evaluation<Self::P>> {
        self.evolve();

        let solution = self.decoder.decode(&self.best().keys);
        let evaluation = self.decoder.problem().objective_function(solution);

        Some(evaluation)
    }
}

// TODO add constructor to validate the input
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
