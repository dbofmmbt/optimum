use crate::percentage::Percentage;

use super::StopCriterion;

pub struct IterCriterion {
    current_iter: usize,
    max_iter: usize,
}

impl IterCriterion {
    #[allow(dead_code)]
    pub fn new(max_iter: usize) -> Self {
        Self {
            max_iter,
            current_iter: 0,
        }
    }
}

impl StopCriterion for IterCriterion {
    fn progress(&self) -> Percentage {
        let ratio = self.current_iter as f64 / self.max_iter as f64;
        unsafe { Percentage::new_unchecked(ratio) }
    }

    fn update(&mut self) {
        self.current_iter += 1;
    }

    fn current_iter(&self) -> usize {
        self.current_iter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut stop = IterCriterion::new(10);
        for _ in 0..10 {
            assert!(!stop.should_stop());
            stop.update();
        }
        assert!(stop.should_stop());
        assert_eq!(stop.current_iter(), 10);
    }
}
