use std::time::{Duration, Instant};

use crate::components::Percentage;

use super::StopCriterion;

pub struct TimeCriterion {
    current_iter: usize,
    start: Instant,
    elapsed: Duration,
    duration: Duration,
}

impl TimeCriterion {
    #[allow(dead_code)]
    pub fn new(duration: Duration) -> Self {
        assert!(!duration.is_zero());

        let start = Instant::now();

        Self {
            current_iter: 0,
            start,
            elapsed: start.elapsed(),
            duration,
        }
    }
}

impl StopCriterion for TimeCriterion {
    fn progress(&self) -> Percentage {
        let ratio = self.elapsed.as_secs_f64() / self.duration.as_secs_f64();

        // SAFETY: `duration` is always different from zero, so `ratio` is a finite value.
        unsafe { Percentage::new_unchecked(ratio) }
    }

    fn update(&mut self) {
        self.elapsed = self.start.elapsed();
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
        let mut stop = TimeCriterion::new(Duration::from_secs(10));
        assert_eq!(stop.current_iter(), 0);
        assert!((stop.progress().value() - Percentage::ZERO.value()).abs() <= 1e-6);

        assert!(!stop.should_stop());

        stop.update();

        assert_eq!(stop.current_iter(), 1);

        // Manually force time passing
        stop.elapsed += Duration::from_secs(10);

        assert!(stop.should_stop());
    }
}
