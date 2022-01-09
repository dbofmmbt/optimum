use std::marker::PhantomData;

/// Abstraction over the coverage of elements by their position
#[derive(Debug, Clone)]
pub struct Coverage<T: ?Sized> {
    state: Vec<u8>,
    phantom: PhantomData<T>,
}

impl<T: Position> Coverage<T> {
    /// Initializes the coverage with the given number of elements
    pub fn new(quantity: usize) -> Self {
        Self {
            state: vec![0; quantity],
            phantom: PhantomData,
        }
    }

    /// Cover the given element.
    ///
    /// # Panics
    ///
    /// It will panic if `element` exceeds the coverage range.
    pub fn cover(&mut self, element: T) {
        self.state[element.position()] += 1;
    }

    /// Uncovers the given element.
    ///
    /// # Panics
    ///
    /// It will panic if `element` exceeds the coverage range or makes the coverage be negative.
    pub fn uncover(&mut self, element: T) {
        self.state[element.position()] -= 1;
    }

    /// Returns how many times the element is covered.
    ///
    /// # Panics
    ///
    /// It will panic if `element` exceeds the coverage range.
    pub fn count(&self, element: T) -> usize {
        self.state[element.position()] as usize
    }

    /// `true` if it was covered at least once.
    pub fn is_covered(&self, element: T) -> bool {
        self.state[element.position()] > 0
    }

    /// Incorporates `other` into `self`
    ///
    /// # Panics
    ///
    /// I'm assuming that `other` has the same length as `self` because
    /// it would probably be a programming error otherwise. if the length
    /// of `self` is bigger than `other`'s, it will panic.
    pub fn merge(&mut self, other: &Self) {
        self.state
            .iter_mut()
            .enumerate()
            .for_each(|(position, cover)| *cover += other.state[position] as u8);
    }

    /// Marks all elements as uncovered. Useful to allow reuse of the coverage.
    pub fn reset(&mut self) {
        self.state.fill(0);
    }
}

pub trait Position {
    fn position(&self) -> usize;
}

macro_rules! impl_position_for_numbers {
    ($($number:ident),+) => ($(
        impl Position for $number {
            fn position(&self) -> usize {
                *self as usize
            }
        }
    )*)
}

impl_position_for_numbers!(i32, isize, u32, usize, f64);

#[cfg(test)]
mod tests {
    use super::Coverage;

    #[test]
    fn test_coverage() {
        let quantity = 10;
        let mut coverage = Coverage::new(quantity);

        coverage.cover(5);

        assert_eq!(coverage.count(5), 1);
        assert_eq!(coverage.count(2), 0);

        coverage.cover(5);

        assert_eq!(coverage.count(5), 2);
    }
    #[test]
    #[should_panic]
    fn test_invalid_position() {
        let mut coverage = Coverage::new(5);
        coverage.cover(20);
        coverage.count(50);
    }
}
