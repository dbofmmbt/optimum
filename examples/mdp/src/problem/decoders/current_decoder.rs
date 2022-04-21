use std::cell::RefCell;

use optimum::{
    core::Problem,
    metaheuristics::genetic::{Decoder, RandomKey},
};
use ordered_float::NotNan;

use crate::{MaximumDiversity, MdpSolution};

pub struct CurrentDecoder<'a> {
    auxiliary: RefCell<Vec<(NotNan<RandomKey>, usize)>>,
    problem: &'a MaximumDiversity,
}

impl<'a> CurrentDecoder<'a> {
    pub fn new(problem: &'a MaximumDiversity) -> Self {
        Self {
            auxiliary: RefCell::new(vec![(NotNan::new(0.0).unwrap(), 0); problem.input_size]),
            problem,
        }
    }
}

impl Decoder for CurrentDecoder<'_> {
    type P = MaximumDiversity;

    fn decode(&self, member: &[RandomKey]) -> <Self::P as Problem>::Solution {
        let mut aux = self.auxiliary.borrow_mut();

        aux.iter_mut().enumerate().for_each(|(i, el)| {
            *el = (unsafe { NotNan::new_unchecked(member[i]) }, i);
        });

        aux.sort_unstable();

        MdpSolution {
            elements: aux[0..self.problem.solution_size]
                .iter()
                .map(|(_, position)| *position)
                .collect(),
        }
    }

    fn problem(&self) -> &Self::P {
        self.problem
    }
}
