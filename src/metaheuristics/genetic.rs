//! # Genetic Algorithms
//!
//! This module contains genetic-based metaheuristics.
//!
//! Currently there's an implementation of [BRKGA][brkga], but more may be added in the future.
//!

pub mod brkga;

pub mod population;

mod decoder;
pub use decoder::{Decoder, RandomKey};
