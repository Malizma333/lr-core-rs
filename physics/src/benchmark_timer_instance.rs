use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::benchmark_timer::BenchmarkTimer;

pub(crate) struct BenchmarkTimerInstance {
    times: HashMap<&'static str, Duration>,
    instant: Instant,
}

impl BenchmarkTimer for BenchmarkTimerInstance {
    fn reset_timer(&mut self) {
        self.instant = Instant::now();
    }

    fn mark_timer(&mut self, label: &'static str) {
        let duration = self.instant.elapsed();
        self.times
            .entry(label)
            .and_modify(|d| *d += duration)
            .or_insert(duration);
    }

    fn print_times(&self) {
        if self.times.is_empty() {
            println!("(no timings recorded)");
            return;
        }

        let total: Duration = self.times.values().copied().sum();

        let total_secs = total.as_secs_f64();

        println!("{:<20} {:>12} {:>8}", "Label", "Time", "Percent");
        println!("{}", "-".repeat(44));

        let mut entries: Vec<_> = self.times.iter().collect();
        // sort descending
        entries.sort_by(|a, b| b.1.cmp(a.1));

        for (label, duration) in entries {
            let secs = duration.as_secs_f64();
            let percent = if total_secs > 0.0 {
                (secs / total_secs) * 100.0
            } else {
                0.0
            };

            println!("{:<20} {:>12.3}s {:>7.2}%", label, secs, percent);
        }

        println!("{}", "-".repeat(44));
        println!("{:<20} {:>12.3}s {:>7}", "TOTAL", total_secs, "100%");
    }
}

impl BenchmarkTimerInstance {
    pub(crate) fn new() -> Self {
        Self {
            times: HashMap::new(),
            instant: Instant::now(),
        }
    }
}
