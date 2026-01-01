use std::time::Duration;

use bevy::prelude::*;

#[derive(Resource)]
pub struct CollisionDiagnostic {
    pub times: Vec<Duration>,
    pub max_samples: usize,
}

impl CollisionDiagnostic {
    pub fn new(max_samples: usize) -> Self {
        Self {
            times: Vec::with_capacity(max_samples),
            max_samples,
        }
    }

    pub fn print_result(&self) {
        if self.times.is_empty() {
            return;
        }

        let total: Duration = self.times.iter().sum();
        let avg = total.as_micros() / self.times.len() as u128;
        let min = self.times.iter().min().unwrap().as_micros();
        let max = self.times.iter().max().unwrap().as_micros();

        println!("Times to run a complete call of solve_collisions fn");
        println!("Total diag duration {} seconds", total.as_secs_f64());
        println!("Average {} µs", avg);
        println!("Minimum {} µs", min);
        println!("Maximum {} µs", max);
    }
}
