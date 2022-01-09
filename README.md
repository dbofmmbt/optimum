# Optimum: a Rust-based Optimization Framework

[![crate](https://img.shields.io/crates/v/optimum.svg)](https://crates.io/crates/optimum)
[![documentation](https://docs.rs/optimum/badge.svg)](https://docs.rs/optimum)
[![build status](https://github.com/dbofmmbt/optimum/workflows/rust/badge.svg)](https://github.com/dbofmmbt/optimum/actions)

**Current state:** Design, Research and Experimentation. Be aware that I **will** change a lot of things until the project reaches an initial version.

Based on my experience working with Metaheuristics, I plan to build a framework capable of accelerating the development of solutions efficiently by leveraging Rust features that ease the creation of reliable and efficient software.

## Goals

Optimum will be:

- Flexible
  - It must be reasonably easy to design new metaheuristics and solve many kinds of problems using Optimum. Therefore it needs to be extensible and configurable.
- Fast
  - Based of zero-cost abstractions, using the framework shouldn't imply in a significant computational overhead.
- Complete
  - Coding the heuristic is not the only task we have when solving a problem. Parameter tuning, validation of generated solutions, instances, result analysis... It should be easier to get all this done in a convenient, standardized way.

Optimum will have:

- Increasing support for many kinds of Metaheuristics
- Common building blocks for the development of problem-specific solutions
- Evaluation tools to speed up the analysis of the heuristics built
- And whatever makes sense for us as developers and researchers!

## Inspirations

To learn from others is key to design a solution which pushes things forward.

- [OptFrame](https://github.com/optframe/optframe), a C++ based framework
