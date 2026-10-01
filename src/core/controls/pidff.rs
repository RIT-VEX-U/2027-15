//! Combined PID + Feedforward (PIDFF) controller.

use super::feedback_base::Feedback;
use super::feedforward::{FeedForward, FeedForwardConfig};
use super::pid::{PIDConfig, PID};
use crate::core::math::math_util::{clamp, sign};

/// Integrated PID and Feedforward controller.
#[derive(Debug, Clone)]
pub struct PIDFF {
    pub pid: PID,
    pub ff_cfg: FeedForwardConfig,
    pub ff: FeedForward,
    out: f64,
    lower_lim: f64,
    upper_lim: f64,
}

impl PIDFF {
    /// Creates a new PIDFF controller.
    pub fn new(pid_cfg: PIDConfig, ff_cfg: FeedForwardConfig) -> Self {
        Self {
            pid: PID::new(pid_cfg),
            ff_cfg,
            ff: FeedForward::new(ff_cfg),
            out: 0.0,
            lower_lim: 0.0,
            upper_lim: 0.0,
        }
    }

    /// Sets the target setpoint.
    pub fn set_target(&mut self, set_pt: f64) {
        self.pid.set_target(set_pt);
    }

    /// Returns the target setpoint.
    pub fn get_target(&self) -> f64 {
        self.pid.get_target()
    }

    /// Returns the last measured sensor value.
    pub fn get_sensor_val(&self) -> f64 {
        self.pid.get_sensor_val()
    }

    /// Iterates the feedback loop with velocity and acceleration feedforward setpoints.
    pub fn update_with_ff(&mut self, val: f64, vel_setpt: f64, a_setpt: f64) -> f64 {
        let pid_out = self.pid.update(val);
        let ff_out = self.ff.calculate(vel_setpt, a_setpt, 0.0);
        self.out = pid_out + ff_out;

        if self.lower_lim != self.upper_lim {
            self.out = clamp(self.out, self.lower_lim, self.upper_lim);
        }

        self.out
    }

    /// Resets the internal PID state.
    pub fn reset(&mut self) {
        self.pid.reset();
    }
}

impl Feedback for PIDFF {
    fn init(&mut self, start_pt: f64, set_pt: f64) {
        self.pid.init(start_pt, set_pt);
    }

    fn update(&mut self, val: f64) -> f64 {
        let pid_out = self.pid.update(val);
        let ff_out = self.ff_cfg.kg + (self.ff_cfg.ks * sign(pid_out));
        self.out = pid_out + ff_out;

        if self.lower_lim != self.upper_lim {
            self.out = clamp(self.out, self.lower_lim, self.upper_lim);
        }

        self.out
    }

    fn get(&self) -> f64 {
        self.out
    }

    fn set_limits(&mut self, lower: f64, upper: f64) {
        self.lower_lim = lower;
        self.upper_lim = upper;
        self.pid.set_limits(lower, upper);
    }

    fn is_on_target(&self) -> bool {
        self.pid.is_on_target()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pidff() {
        let mut pidff = PIDFF::new(
            PIDConfig {
                p: 1.0,
                ..Default::default()
            },
            FeedForwardConfig {
                ks: 0.5,
                kv: 1.0,
                ..Default::default()
            },
        );

        pidff.init(0.0, 10.0);
        let out = pidff.update_with_ff(0.0, 2.0, 0.0);
        // PID(10.0) + FF(kv=1.0*2.0 + ks=0.5*1.0) = 12.5
        assert_eq!(out, 12.5);
    }
}
