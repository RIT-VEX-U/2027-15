//! Delay command pausing autonomous execution for a specified duration.

use super::auto_command::AutoCommand;
use crate::core::time::Timer;

/// Autonomous command that pauses progression for a specified number of milliseconds.
pub struct DelayCommand {
    ms: u64,
    timer: Timer,
    started: bool,
}

impl DelayCommand {
    /// Constructs a DelayCommand.
    pub fn new(ms: u64) -> Self {
        Self {
            ms,
            timer: Timer::new(),
            started: false,
        }
    }
}

impl AutoCommand for DelayCommand {
    fn run(&mut self) -> bool {
        if !self.started {
            self.timer.reset();
            self.started = true;
        }

        self.timer.time_msec() >= (self.ms as f64)
    }

    fn timeout_seconds(&self) -> f64 {
        (self.ms as f64) / 1000.0 + 1.0
    }

    fn to_string_desc(&self) -> String {
        format!("DelayCommand({} ms)", self.ms)
    }
}
