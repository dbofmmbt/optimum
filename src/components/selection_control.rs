use std::ops::Index;

use rand::Rng;

/// `SelectionControl` is an abstraction over random selection of elements in an array.
#[derive(Debug)]
pub struct SelectionControl {
    already_chosen: Vec<bool>,
    total_selected: usize,
}

impl SelectionControl {
    /// Initializes `SelectionControl` with the given quantity as the number of elements.
    pub fn new(quantity: usize) -> Self {
        let already_chosen = vec![false; quantity as usize];

        Self {
            already_chosen,
            total_selected: 0,
        }
    }

    pub fn next(&mut self, rng: &mut impl Rng) -> Option<usize> {
        let len = self.already_chosen.len();

        if self.total_selected == len {
            return None;
        }

        let chosen = loop {
            let value = rng.gen_range(0..len);
            if !self.already_chosen[value] {
                break value;
            }
        };

        self.mark_as_selected(chosen);

        Some(chosen)
    }

    fn mark_as_selected(&mut self, element: usize) {
        self.already_chosen[element] = true;
        self.total_selected += 1;
    }

    // It may be useful in the future
    #[allow(dead_code)]
    fn mark_as_not_selected(&mut self, element: usize) {
        self.already_chosen[element] = false;
        self.total_selected -= 1;
    }
}

impl Index<usize> for SelectionControl {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.already_chosen[index]
    }
}
