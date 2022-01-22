use std::{
    ops::{Div, Sub},
    time::Duration,
};

use crate::core::{Objective, Problem};

use super::{BatchResult, Execution};

/// Process and collect statistics about a previously executed `Batch`.
pub struct Statistics<'a, P: Problem, H> {
    value_sum: f64,
    time_sum: Duration,
    batch: &'a BatchResult<P, H>,
}

impl<'a, P: Problem, H> Statistics<'a, P, H>
where
    P::Value: Into<f64>,
{
    /// Generate `Statistics` for a given `batch`.
    pub fn new(batch: &'a BatchResult<P, H>) -> Self {
        let score_sum = batch
            .executions()
            .iter()
            .map(|exec| exec.evaluation.value().into())
            .sum();
        let time_sum = batch.executions().iter().map(|exec| exec.duration).sum();

        Self {
            value_sum: score_sum,
            time_sum,
            batch,
        }
    }

    /// The average value of all executions
    pub fn average_value(&self) -> f64 {
        self.value_sum / self.batch.executions.len() as f64
    }

    /// Returns the value's variance of all executions
    pub fn value_variance(&self) -> f64 {
        self.batch
            .executions
            .iter()
            .map(|exec| exec.evaluation.value())
            .map(|value| {
                let diff = self.average_value() - value.into();

                diff * diff
            })
            .sum::<f64>()
            / self.batch.executions.len() as f64
    }

    /// The average time expended on all executions
    pub fn average_time(&self) -> Duration {
        self.time_sum / self.batch.executions.len() as u32
    }

    /// Get a reference to the run's best.
    pub fn best(&self) -> &Execution<P, H> {
        let iter = self.batch.executions().iter();

        match P::OBJECTIVE {
            Objective::Min => iter.min_by_key(|exec| exec.evaluation.value()),
            Objective::Max => iter.max_by_key(|exec| exec.evaluation.value()),
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
