use std::{
    ops::{Div, Sub},
    time::Duration,
};

use crate::core::{Evaluation, Objective, Problem};

use super::BatchResult;

/// Process and collect statistics about a previously executed `Batch`.
pub struct Statistics<'a, P: Problem> {
    value_sum: f64,
    time_sum: Duration,
    batch: &'a BatchResult<P>,
}

impl<'a, P: Problem> Statistics<'a, P>
where
    P::Value: Into<f64>,
{
    /// Generate `Statistics` for a given `batch`.
    pub fn new(batch: &'a BatchResult<P>) -> Self {
        let score_sum = batch
            .evaluations()
            .iter()
            .map(|(_, e, _)| e.value().into())
            .sum();
        let time_sum = batch.evaluations().iter().map(|(_, _, t)| t).sum();

        Self {
            value_sum: score_sum,
            time_sum,
            batch,
        }
    }

    /// The average value of all executions
    pub fn average_value(&self) -> f64 {
        self.value_sum / self.batch.executions as f64
    }

    /// The average time expended on all executions
    pub fn average_time(&self) -> Duration {
        self.time_sum / self.batch.executions as u32
    }

    /// Get a reference to the run's best.
    pub fn best(&self) -> &(usize, Evaluation<P>, Duration) {
        let iter = self.batch.evaluations().iter();

        match P::OBJECTIVE {
            Objective::Min => iter.min_by_key(|(_, e, _)| e.value()),
            Objective::Max => iter.max_by_key(|(_, e, _)| e.value()),
        }
        .expect("A Batch should always have at least one execution")
    }
}

/// Used to implement the [gap][Gap::gap] comparison function between two values (often from Problem::Value).
pub trait Gap<F: Into<f64>>: Copy + Sub<Output = Self> + Div<Output = F> {
    /// The GAP is the relative difference between `self` and `other`.
    fn gap(self, other: Self) -> f64 {
        ((self - other) / other).into() * 100.0
    }
}

impl<V: Copy + Sub<Output = Self> + Div<Output = f64>> Gap<f64> for V {}
