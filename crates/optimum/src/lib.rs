#![warn(missing_docs)]

//! # Optimization Framework
//!
//!
//! - [core]: main elements used by every optimization problem
//! - [metaheuristics]: implementations of consolidated metaheuristics
//! - [components]: Common building blocks to implement the problem-dependent code. Stuff here is very unstable right now.
//!

pub use optimum_core as core;

pub use optimum_components as components;

pub use optimum_metaheuristics as metaheuristics;
