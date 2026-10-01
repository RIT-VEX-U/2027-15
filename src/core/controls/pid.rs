//! Proportional-Integral-Derivative (PID) feedback controller with integral anti-windup clamping.

use super::feedback_base::Feedback;
use crate::core::math::math_util::clamp;
use crate::core::time::Timer;

/// Determines whether PID error is computed linearly or wrapped angularly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ErrorType {
    #[default]
    Linear,
    Angular, // Assumes degrees
}

/// Configuration parameters for a PID controller.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PIDConfig {
    pub p: f64,
    pub i: f64,
    pub d: f64,
    pub deadband: f64,
    pub on_target_time: f64,
    pub error_method: ErrorType,
}

/// Type alias for PIDConfig.
pub type PidConfig = PIDConfig;

/// Standard PID feedback controller.
#[derive(Debug, Clone)]
pub struct PID {
    pub config: PIDConfig,

    last_error: f64,
    accum_error: f64,
    last_time: f64,
    on_target_last_time: f64,

    lower_limit: f64,
    upper_limit: f64,

    target: f64,
    target_vel: f64,
    sensor_val: f64,
    out: f64,

    is_checking_on_target: bool,
    pid_timer: Timer,
}

/// Type alias for PID controller.
pub type Pid = PID;

impl PID {
    /// Creates a new PID controller from configuration.
    pub fn new(config: PIDConfig) -> Self {
        Self {
            config,
            last_error: 0.0,
            accum_error: 0.0,
            last_time: 0.0,
            on_target_last_time: 0.0,
            lower_limit: 0.0,
            upper_limit: 0.0,
            target: 0.0,
            target_vel: 0.0,
            sensor_val: 0.0,
            out: 0.0,
            is_checking_on_target: false,
            pid_timer: Timer::new(),
        }
    }

    /// Resets the internal timer, accumulated integral error, and target states.
    pub fn reset(&mut self) {
        self.pid_timer.reset();
        self.last_error = 0.0;
        self.last_time = 0.0;
        self.accum_error = 0.0;
        self.is_checking_on_target = false;
        self.on_target_last_time = 0.0;
    }

    /// Returns the last measured sensor value.
    pub fn get_sensor_val(&self) -> f64 {
        self.sensor_val
    }

    /// Returns the target setpoint.
    pub fn get_target(&self) -> f64 {
        self.target
    }

    /// Sets the target setpoint.
    pub fn set_target(&mut self, target: f64) {
        self.target = target;
    }

    /// Returns the most recent controller output.
    pub fn get(&self) -> f64 {
        self.out
    }

    /// Updates the PID controller with the current sensor value.
    pub fn update(&mut self, sensor_val: f64) -> f64 {
        self.update_with_v_setpt(sensor_val, 0.0)
    }

    /// Updates the PID controller with an explicit timestep dt (seconds).
    pub fn update_dt(&mut self, sensor_val: f64, dt: f64) -> f64 {
        self.sensor_val = sensor_val;

        let d_term = if dt > 1e-7 {
            self.config.d * ((self.get_error() - self.last_error) / dt)
        } else {
            0.0
        };

        self.out = (self.config.p * self.get_error()) + d_term;

        let limits_exist = self.lower_limit != 0.0 || self.upper_limit != 0.0;
        if !limits_exist || (self.out < self.upper_limit && self.out > self.lower_limit) {
            self.accum_error += dt * self.get_error();
        }

        self.out += self.config.i * self.accum_error;
        self.last_error = self.get_error();

        if limits_exist {
            self.out = clamp(self.out, self.lower_limit, self.upper_limit);
        }

        self.out
    }

    /// Computes the error between setpoint and sensor value.
    pub fn get_error(&self) -> f64 {
        match self.config.error_method {
            ErrorType::Angular => {
                let mut diff = (self.target - self.sensor_val) % 360.0;
                if diff < -180.0 {
                    diff += 360.0;
                } else if diff > 180.0 {
                    diff -= 360.0;
                }
                diff
            }
            ErrorType::Linear => self.target - self.sensor_val,
        }
    }

    /// Updates the PID loop with a new sensor reading and velocity setpoint subtraction.
    pub fn update_with_v_setpt(&mut self, sensor_val: f64, v_setpt: f64) -> f64 {
        self.sensor_val = sensor_val;

        let current_time = self.pid_timer.value();
        let time_delta = current_time - self.last_time;

        let d_term = if time_delta > 1e-7 {
            self.config.d * (((self.get_error() - self.last_error) / time_delta) - v_setpt)
        } else {
            0.0
        };

        self.out = (self.config.p * self.get_error()) + d_term;

        let limits_exist = self.lower_limit != 0.0 || self.upper_limit != 0.0;
        if !limits_exist || (self.out < self.upper_limit && self.out > self.lower_limit) {
            self.accum_error += time_delta * self.get_error();
        }

        self.out += self.config.i * self.accum_error;

        self.last_time = current_time;
        self.last_error = self.get_error();

        if limits_exist {
            self.out = clamp(self.out, self.lower_limit, self.upper_limit);
        }

        self.out
    }
}

impl Feedback for PID {
    fn init(&mut self, start_pt: f64, set_pt: f64) {
        self.set_target(set_pt);
        self.target_vel = 0.0;
        self.sensor_val = start_pt;
        self.reset();
    }

    fn update(&mut self, sensor_val: f64) -> f64 {
        self.update_with_v_setpt(sensor_val, 0.0)
    }

    fn get(&self) -> f64 {
        self.out
    }

    fn set_limits(&mut self, lower: f64, upper: f64) {
        self.lower_limit = lower;
        self.upper_limit = upper;
    }

    fn is_on_target(&self) -> bool {
        if self.get_error().abs() < self.config.deadband {
            if self.target_vel != 0.0 {
                return true;
            }
            if !self.is_checking_on_target {
                return false;
            }
            if self.pid_timer.value() - self.on_target_last_time > self.config.on_target_time {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pid_linear() {
        let mut pid = PID::new(PIDConfig {
            p: 1.0,
            i: 0.0,
            d: 0.0,
            deadband: 0.1,
            on_target_time: 0.05,
            error_method: ErrorType::Linear,
        });

        pid.init(0.0, 10.0);
        let out = pid.update(0.0);
        assert_eq!(out, 10.0);

        let out2 = pid.update(10.0);
        assert_eq!(out2, 0.0);
    }
}
