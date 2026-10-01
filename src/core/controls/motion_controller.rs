//! High-level motion controller combining TrapezoidProfile, PID, and FeedForward.

use super::feedback_base::Feedback;
use super::feedforward::{FeedForward, FeedForwardConfig};
use super::pid::{PIDConfig, PID};
use super::trapezoid_profile::{MotionState, TrapezoidProfile};
use crate::core::math::math_util::clamp;
use crate::core::time::Timer;

/// Configuration for a motion profile controller.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MotionProfileConfig {
    pub max_v: f64,
    pub accel: f64,
    pub pid_cfg: PIDConfig,
    pub ff_cfg: FeedForwardConfig,
}

/// Integrated motion controller that tracks position and velocity targets along a motion profile.
#[derive(Debug, Clone)]
pub struct MotionController {
    pub config: MotionProfileConfig,
    pub pid: PID,
    pub ff: FeedForward,
    profile: TrapezoidProfile,
    cur_motion: MotionState,
    tmr: Timer,
    out: f64,
    lower_limit: f64,
    upper_limit: f64,
}

impl MotionController {
    /// Constructs a new MotionController.
    pub fn new(config: MotionProfileConfig) -> Self {
        Self {
            pid: PID::new(config.pid_cfg),
            ff: FeedForward::new(config.ff_cfg),
            profile: TrapezoidProfile::new(0.0, 0.0, config.max_v, config.accel, config.accel),
            cur_motion: MotionState::default(),
            tmr: Timer::new(),
            out: 0.0,
            lower_limit: 0.0,
            upper_limit: 0.0,
            config,
        }
    }

    /// Returns the current motion profile target state.
    pub fn get_motion(&self) -> MotionState {
        self.cur_motion
    }
}

impl Feedback for MotionController {
    fn init(&mut self, start_pt: f64, end_pt: f64) {
        self.profile = TrapezoidProfile::new(
            start_pt,
            end_pt,
            self.config.max_v,
            self.config.accel,
            self.config.accel,
        );
        self.pid.reset();
        self.tmr.reset();
    }

    fn update(&mut self, sensor_val: f64) -> f64 {
        self.cur_motion = self.profile.calculate(self.tmr.time_sec());
        self.pid.set_target(self.cur_motion.pos);
        self.pid
            .update_with_v_setpt(sensor_val, self.cur_motion.vel);

        self.out = self.pid.get()
            + self
                .ff
                .calculate(self.cur_motion.vel, self.cur_motion.acc, self.pid.get());

        if self.lower_limit != self.upper_limit {
            self.out = clamp(self.out, self.lower_limit, self.upper_limit);
        }

        self.out
    }

    fn get(&self) -> f64 {
        self.out
    }

    fn set_limits(&mut self, lower: f64, upper: f64) {
        self.lower_limit = lower;
        self.upper_limit = upper;
        self.pid.set_limits(lower, upper);
    }

    fn is_on_target(&self) -> bool {
        self.tmr.time_sec() >= self.profile.total_time() && self.pid.is_on_target()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motion_controller() {
        let mut mc = MotionController::new(MotionProfileConfig {
            max_v: 5.0,
            accel: 2.0,
            pid_cfg: PIDConfig {
                p: 1.0,
                deadband: 0.1,
                on_target_time: 0.01,
                ..Default::default()
            },
            ff_cfg: FeedForwardConfig {
                kv: 1.0,
                ..Default::default()
            },
        });

        mc.init(0.0, 10.0);
        let out = mc.update(0.0);
        assert!(out >= 0.0);
    }
}
