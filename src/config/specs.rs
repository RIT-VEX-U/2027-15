//! Physical and algorithmic robot specification configuration.

pub use crate::mantle::subsystems::tank_drive::RobotSpecs;
use crate::core::controls::pid::PidConfig;

/// Physical robot dimensions and mechanical parameters (inches).
pub const ROBOT_RADIUS_INCHES: f64 = 9.0;
pub const ODOM_WHEEL_DIAMETER_INCHES: f64 = 2.75;
pub const ODOM_GEAR_RATIO: f64 = 1.0;
pub const TRACK_WIDTH_INCHES: f64 = 12.0;
pub const DRIVE_CORRECTION_CUTOFF_INCHES: f64 = 2.0;

/// Default configured robot specifications.
pub fn default_robot_specs() -> RobotSpecs {
    RobotSpecs {
        robot_radius: ROBOT_RADIUS_INCHES,
        odom_wheel_diam: ODOM_WHEEL_DIAMETER_INCHES,
        odom_gear_ratio: ODOM_GEAR_RATIO,
        dist_between_wheels: TRACK_WIDTH_INCHES,
        drive_correction_cutoff: DRIVE_CORRECTION_CUTOFF_INCHES,
        correction_pid: PidConfig {
            p: 0.05,
            i: 0.0,
            d: 0.005,
            ..Default::default()
        },
    }
}
