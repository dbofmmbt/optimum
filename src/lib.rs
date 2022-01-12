#![warn(missing_docs)]
//! # Optimization Framework
//!
//! - [core]: main elements used by every optimization problem
//! - [metaheuristics]: implementations of consolidated metaheuristics
//! - [analysis]: Tools to gather and visualize metrics to improve understanding of a solver's behavior.
//! - [components]: Common building blocks to implement the problem-dependent code. Stuff here is very unstable right now.
//!

#[allow(missing_docs)]
pub mod components;

pub mod analysis;
pub mod core;
pub mod metaheuristics;
