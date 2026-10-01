//! Robot control gains, PID parameters, and motion profile thresholds.

use crate::core::controls::feedforward::FeedForwardConfig;
use crate::core::controls::pid::PidConfig;

/// Default drive straight forward PID gains.
pub const DRIVE_PID_GAINS: PidConfig = PidConfig {
    p: 0.15,
    i: 0.0,
    d: 0.015,
    deadband: 0.5,
    on_target_time: 0.1,
    error_method: crate::core::controls::pid::ErrorType::Linear,
};

/// Default turn to heading PID gains.
pub const TURN_PID_GAINS: PidConfig = PidConfig {
    p: 0.04,
    i: 0.0,
    d: 0.004,
    deadband: 1.0,
    on_target_time: 0.1,
    error_method: crate::core::controls::pid::ErrorType::Angular,
};

/// Lift arm holding PID gains.
pub const LIFT_PID_GAINS: PidConfig = PidConfig {
    p: 0.8,
    i: 0.0,
    d: 0.05,
    deadband: 0.02,
    on_target_time: 0.05,
    error_method: crate::core::controls::pid::ErrorType::Linear,
};

/// Flywheel velocity feedforward gains.
pub const FLYWHEEL_FF_GAINS: FeedForwardConfig = FeedForwardConfig {
    ks: 0.5,
    kv: 0.0035,
    ka: 0.0005,
    kg: 0.0,
};

/// Pure pursuit path following lookahead radius (inches).
pub const PURE_PURSUIT_LOOKAHEAD_RADIUS: f64 = 10.0;
