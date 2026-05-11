//! Tracks simulation statistics and SLA constraints.

#[derive(Debug, Default)]
pub struct SimulationMetrics {
    pub total_tasks_generated: usize,
    pub successful_tasks: usize,
    pub dropped_tasks: usize,
    pub total_delay_seconds: f64,
}

impl SimulationMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a successfully processed task.
    pub fn record_success(&mut self, delay: f64) {
        self.total_tasks_generated += 1;
        self.successful_tasks += 1;
        self.total_delay_seconds += delay;
    }

    /// Records a task that was dropped due to latency constraints or lack of capacity.
    pub fn record_drop(&mut self) {
        self.total_tasks_generated += 1;
        self.dropped_tasks += 1;
    }

    /// Returns the success rate as a ratio (0.0 to 1.0).
    pub fn success_rate(&self) -> f64 {
        if self.total_tasks_generated == 0 {
            return 1.0;
        }
        self.successful_tasks as f64 / self.total_tasks_generated as f64
    }

    /// Returns the average delay of all *successful* tasks.
    pub fn average_delay(&self) -> f64 {
        if self.successful_tasks == 0 {
            return 0.0;
        }
        self.total_delay_seconds / self.successful_tasks as f64
    }
}
