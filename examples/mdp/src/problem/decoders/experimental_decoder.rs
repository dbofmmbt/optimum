use std::cell::RefCell;

use optimum::{
    components::coverage::Coverage,
    core::Problem,
    metaheuristics::genetic::{Decoder, RandomKey},
};

use crate::MaximumDiversity;

use crate::MdpSolution;

pub struct ExperimentalDecoder<'a> {
    pub problem: &'a MaximumDiversity,
    coverage: RefCell<Coverage<usize>>,
}

impl<'a> ExperimentalDecoder<'a> {
    pub fn new(problem: &'a MaximumDiversity) -> Self {
        Self {
            problem,
            coverage: RefCell::new(Coverage::new(problem.input_size)),
        }
    }
}

impl Decoder for ExperimentalDecoder<'_> {
    type P = MaximumDiversity;

    fn decode(&self, member: &[RandomKey]) -> <Self::P as Problem>::Solution {
        let mut solution = MdpSolution {
            elements: vec![0; self.problem.solution_size],
        };

        let mut coverage = self.coverage.borrow_mut();
        coverage.reset();

        for (idx, key) in member.iter().copied().enumerate() {
            let mut element = (key * self.problem.input_size as f64).floor() as usize;

            // Collision treatment
            loop {
                if !coverage.is_covered(element) {
                    coverage.cover(element);
                    break;
                }

                element = (element + 1) % self.problem.input_size;
            }

            solution.elements[idx] = element;
        }

        #[cfg(debug_assertions)]
        check_for_collisions(&solution);

        solution
    }

    fn problem(&self) -> &Self::P {
        self.problem
    }
}

#[cfg(debug_assertions)]
fn check_for_collisions(solution: &MdpSolution) {
    for el in solution.elements.iter().copied() {
        if solution
            .elements
            .iter()
            .copied()
            .filter(|&it| it == el)
            .count()
            != 1
        {
            println!("Collision! {}", el);
        }
    }
}
