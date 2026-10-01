//! Base trait and common math for robot odometry tracking systems.
//!
//! Mirrors `core/subsystems/odometry/odometry_base.h` and `core/subsystems/odometry/odometry_base.cpp`.

use crate::core::geometry::Pose2d;

/// Calculates the smallest signed difference in degrees between two headings.
///
/// Returns a value in $(-180^\circ, +180^\circ]$, where positive indicates a CCW turn
/// and negative indicates a CW turn.
pub fn smallest_angle(start_deg: f64, end_deg: f64) -> f64 {
    let mut diff = (end_deg - start_deg) % 360.0;
    if diff < 0.0 {
        diff += 360.0;
    }
    if diff > 180.0 {
        diff -= 360.0;
    }
    diff
}

/// Common trait implemented by all robot odometry positioning systems.
pub trait Odometry: Send + Sync {
    /// Returns the current estimated robot pose (X, Y, rotation).
    fn get_position(&self) -> Pose2d;

    /// Overrides the current robot pose estimate.
    fn set_position(&mut self, new_pos: Pose2d);

    /// Updates the odometry state from sensor readings and returns the new pose.
    fn update(&mut self) -> Pose2d;

    /// Returns the current linear velocity (inches/second).
    fn get_speed(&self) -> f64 {
        0.0
    }

    /// Returns the current linear acceleration (inches/second$^2$).
    fn get_accel(&self) -> f64 {
        0.0
    }

    /// Returns the current angular velocity (degrees/second).
    fn get_angular_speed_deg(&self) -> f64 {
        0.0
    }

    /// Returns the current angular acceleration (degrees/second$^2$).
    fn get_angular_accel_deg(&self) -> f64 {
        0.0
    }
}

/// Generic base tracking state storage.
#[derive(Debug, Clone, Default)]
pub struct OdometryState {
    /// Current estimated robot pose.
    pub current_pos: Pose2d,
    /// Estimated linear velocity (in/s).
    pub speed: f64,
    /// Estimated linear acceleration (in/s^2).
    pub accel: f64,
    /// Estimated angular velocity (deg/s).
    pub ang_speed_deg: f64,
    /// Estimated angular acceleration (deg/s^2).
    pub ang_accel_deg: f64,
}

impl OdometryState {
    /// Creates a new odometry state with zero initial values.
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_angle() {
        assert!((smallest_angle(10.0, 20.0) - 10.0).abs() < 1e-6);
        assert!((smallest_angle(350.0, 10.0) - 20.0).abs() < 1e-6);
        assert!((smallest_angle(10.0, 350.0) - (-20.0)).abs() < 1e-6);
        assert!((smallest_angle(0.0, 180.0) - 180.0).abs() < 1e-6);
        assert!((smallest_angle(0.0, 270.0) - (-90.0)).abs() < 1e-6);
    }
}
