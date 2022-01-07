use crate::components::Percentage;

use super::StopCriterion;
use crate::core::Problem;

use std::marker::PhantomData;

/// Takes two criterions and combines them, finishing as soon as either of them finishes.
///
/// The [progress][StopCriterion::progress] is the highest of the two.
pub struct CriterionCombiner<P, A, B> {
    a: A,
    b: B,
    _p: PhantomData<P>,
}

impl<A, B, P> CriterionCombiner<P, A, B>
where
    A: StopCriterion<P>,
    B: StopCriterion<P>,
    P: Problem,
{
    /// Creates a new criterion combiner based on `a` and `b`.
    pub fn new(a: A, b: B) -> Self {
        Self {
            a,
            b,
            _p: PhantomData,
        }
    }
}

impl<A, B, P> StopCriterion<P> for CriterionCombiner<P, A, B>
where
    A: StopCriterion<P>,
    B: StopCriterion<P>,
    P: Problem,
{
    fn progress(&self) -> Percentage {
        let a = self.a.progress();
        let b = self.b.progress();

        if a > b {
            a
        } else {
            b
        }
    }

    fn update(&mut self, new_value: P::Value) {
        self.a.update(new_value);
        self.b.update(new_value);
    }

    fn current_iter(&self) -> usize {
        // Both should have the same value for it, so I'll just take it from `a`
        self.a.current_iter()
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use crate::core::stop_criterion::*;

    use super::*;

    fn setup<P: Problem>(iter: usize, time: Duration) -> impl StopCriterion<P> {
        let iter = IterCriterion::<P>::new(iter);
        let time = TimeCriterion::<P>::new(time);

        CriterionCombiner::new(iter, time)
    }

    #[test]
    fn iter_works() {
        let max_iters = 5;
        let mut stop_criterion = setup::<()>(max_iters, Duration::MAX);

        (0..max_iters).for_each(|i| {
            assert!(!stop_criterion.should_stop());
            stop_criterion.update(i);
        });

        assert!(stop_criterion.should_stop());
    }

    #[test]
    fn time_works() {
        let stop_criterion = setup::<()>(usize::MAX, Duration::from_nanos(1));

        // Ensure that the required time will pass.
        sleep(Duration::from_millis(1));

        assert!(stop_criterion.should_stop());
    }
}
