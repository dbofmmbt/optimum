use std::{
    marker::PhantomData,
    time::{Duration, Instant},
};

use crate::core::{Problem, StopCriterion};

/// This criterion is based on a maximum duration.
///
/// The timer starts as soon as the criterion is created (i.e. through [new][TimeCriterion::new])
/// and stops when it exceeds the duration given.
pub struct TimeCriterion<P> {
    current_iter: usize,
    start: Instant,
    elapsed: Duration,
    duration: Duration,
    _p: PhantomData<P>,
}

impl<P> std::fmt::Debug for TimeCriterion<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TimeCriterion")
            .field("current_iter", &self.current_iter)
            .field("start", &self.start)
            .field("elapsed", &self.elapsed)
            .field("duration", &self.duration)
            .field("_p", &self._p)
            .finish()
    }
}

impl<P> Clone for TimeCriterion<P> {
    fn clone(&self) -> Self {
        Self::new(self.duration)
    }
}

impl<P> TimeCriterion<P> {
    #[allow(dead_code)]
    /// Creates a new time criterion.
    pub fn new(duration: Duration) -> Self {
        assert!(!duration.is_zero());

        let start = Instant::now();

        Self {
            current_iter: 0,
            start,
            elapsed: start.elapsed(),
            duration,
            _p: PhantomData,
        }
    }
}

impl<P: Problem> StopCriterion<P> for TimeCriterion<P> {
    fn progress(&self) -> f64 {
        // `duration` is always different from zero, so `ratio` is a finite value.
        self.elapsed.as_secs_f64() / self.duration.as_secs_f64()
    }

    fn update(&mut self, _: <P as Problem>::Value) {
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

    use num_traits::Zero;

    #[test]
    fn it_works() {
        let mut stop = TimeCriterion::<()>::new(Duration::from_secs(10));
        assert_eq!(stop.current_iter(), 0);
        assert!((stop.progress() - f64::zero()).abs() <= 1e-6);

        assert!(!stop.should_stop());

        stop.update(0);

        assert_eq!(stop.current_iter(), 1);

        // Manually force time passing
        stop.elapsed += Duration::from_secs(10);

        assert!(stop.should_stop());
    }

    // TODO add tests for Clone because the semantic is customized
}
