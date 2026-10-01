//! Platform-agnostic monotonic timing, stopwatch, and delay abstractions.

use std::thread;
use std::time::{Duration, Instant};

/// Sleep for a specified number of milliseconds.
pub fn delay_ms(ms: u32) {
    thread::sleep(Duration::from_millis(ms as u64));
}

/// Sleep for a specified number of microseconds.
pub fn delay_us(us: u64) {
    thread::sleep(Duration::from_micros(us));
}

/// Monotonic timer for measuring elapsed time, calculating control intervals,
/// and timing state machine transitions.
#[derive(Debug, Clone)]
pub struct Timer {
    start: Instant,
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Timer {
    /// Constructs and starts a new Timer.
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    /// Returns current system time in milliseconds since UNIX epoch.
    pub fn system_ms() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0)
    }

    /// Alias for system_ms.
    pub fn system() -> u64 {
        Self::system_ms()
    }

    /// Resets the timer to zero.
    pub fn reset(&mut self) {
        self.start = Instant::now();
    }

    /// Returns elapsed time in seconds.
    pub fn value(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }

    /// Returns elapsed time in seconds.
    pub fn time_sec(&self) -> f64 {
        self.value()
    }

    /// Returns elapsed time in milliseconds.
    pub fn time_ms(&self) -> u32 {
        self.start.elapsed().as_millis() as u32
    }

    /// Returns elapsed time in milliseconds as f64 (matching C++ time_msec).
    pub fn time_msec(&self) -> f64 {
        self.start.elapsed().as_secs_f64() * 1000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer() {
        let mut timer = Timer::new();
        assert!(timer.value() >= 0.0);
        timer.reset();
        assert!(timer.value() < 0.5);
    }
}
