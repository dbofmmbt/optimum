use std::cell::RefCell;

use optimum::{components::coverage::Coverage, metaheuristics::genetic::Decoder};

use crate::{
    problem::{Tsp, TspSolution},
    CITIES,
};

pub struct TspDecoder<'a> {
    problem: &'a Tsp,
    coverage: RefCell<Coverage<usize>>,
}

impl<'a> TspDecoder<'a> {
    pub fn new(problem: &'a Tsp) -> Self {
        Self {
            problem,
            coverage: RefCell::new(Coverage::new(CITIES)),
        }
    }
}

impl Decoder for TspDecoder<'_> {
    type P = Tsp;

    fn decode(
        &self,
        member: &[optimum::metaheuristics::genetic::RandomKey],
    ) -> <Self::P as optimum::core::Problem>::Solution {
        let mut solution = TspSolution {
            cities: vec![0; member.len()],
        };

        let mut coverage = self.coverage.borrow_mut();
        coverage.reset();

        for (idx, key) in member.iter().copied().enumerate() {
            let mut element = (key * member.len() as f64).floor() as usize;

            // Collision treatment
            loop {
                if !coverage.is_covered(element) {
                    coverage.cover(element);
                    break;
                }

                element = (element + 1) % member.len();
            }

            solution.cities[idx] = element;
        }

        #[cfg(debug_assertions)]
        {
            fn check_for_collisions(solution: &TspSolution) {
                for el in solution.cities.iter().copied() {
                    if solution
                        .cities
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
            check_for_collisions(&solution);
        }

        solution
    }

    fn problem(&self) -> &Self::P {
        self.problem
    }
}
