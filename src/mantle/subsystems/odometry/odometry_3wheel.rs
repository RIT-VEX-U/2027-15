//! Three-wheel (two parallel + one perpendicular off-axis) tracking wheel odometry.
//!
//! Mirrors `core/subsystems/odometry/odometry_3wheel.h` and `core/subsystems/odometry/odometry_3wheel.cpp`.

use std::f64::consts::PI;
use super::odometry_base::{smallest_angle, Odometry, OdometryState};
use crate::core::geometry::{Pose2d, Rotation2d, Translation2d};
use crate::core::math::math_util::wrap_angle_rad;
use crate::core::time::Timer;

/// Configuration for 3-wheel tracking setup.
#[derive(Debug, Clone, Copy)]
pub struct Odometry3WheelConfig {
    /// Distance between left and right tracking wheels (wheelbase).
    pub wheelbase_dist: f64,
    /// Perpendicular distance from tracking center to the off-axis (back) tracking wheel.
    pub off_axis_center_dist: f64,
    /// Diameter of tracking wheels.
    pub wheel_diam: f64,
}

impl Default for Odometry3WheelConfig {
    fn default() -> Self {
        Self {
            wheelbase_dist: 10.0,
            off_axis_center_dist: 5.0,
            wheel_diam: 2.75,
        }
    }
}

/// Three-wheel unpowered tracking wheel odometry system.
pub struct Odometry3Wheel {
    encoder_readings: Box<dyn Fn() -> (f64, f64, f64) + Send + Sync>,
    cfg: Odometry3WheelConfig,
    state: OdometryState,
    lside_old_deg: f64,
    rside_old_deg: f64,
    offax_old_deg: f64,
    timer: Timer,
    last_pos: Pose2d,
    last_speed: f64,
    last_ang_speed: f64,
}

impl Odometry3Wheel {
    /// Creates a new 3-wheel odometry tracker.
    ///
    /// # Parameters
    /// - `encoder_readings`: Closure returning `(left_deg, right_deg, off_axis_deg)`.
    /// - `cfg`: Mechanical configuration geometry.
    pub fn new(
        encoder_readings: impl Fn() -> (f64, f64, f64) + Send + Sync + 'static,
        cfg: Odometry3WheelConfig,
    ) -> Self {
        Self {
            encoder_readings: Box::new(encoder_readings),
            cfg,
            state: OdometryState::new(),
            lside_old_deg: 0.0,
            rside_old_deg: 0.0,
            offax_old_deg: 0.0,
            timer: Timer::new(),
            last_pos: Pose2d::new(0.0, 0.0, Rotation2d::new(0.0)),
            last_speed: 0.0,
            last_ang_speed: 0.0,
        }
    }

    /// Pure calculation method for advancing robot pose given degree deltas.
    pub fn calculate_new_pos(
        lside_delta_deg: f64,
        rside_delta_deg: f64,
        offax_delta_deg: f64,
        old_pos: &Pose2d,
        cfg: &Odometry3WheelConfig,
    ) -> Pose2d {
        let lside_dist = (cfg.wheel_diam / 2.0) * lside_delta_deg.to_radians();
        let rside_dist = (cfg.wheel_diam / 2.0) * rside_delta_deg.to_radians();
        let offax_dist = (cfg.wheel_diam / 2.0) * offax_delta_deg.to_radians();

        let delta_angle_rad = (rside_dist - lside_dist) / cfg.wheelbase_dist;

        let dist_local_y = (lside_dist + rside_dist) / 2.0;
        let dist_local_x = offax_dist - (delta_angle_rad * cfg.off_axis_center_dist);

        let local_displacement = Translation2d::new(dist_local_x, dist_local_y);

        let dir_delta_from_trans_rad = local_displacement.theta().radians() - (PI / 2.0);
        let global_dir_rad =
            wrap_angle_rad(dir_delta_from_trans_rad + old_pos.rotation().radians());

        let global_displacement =
            Translation2d::from_polar(local_displacement.norm(), Rotation2d::new(global_dir_rad));

        let new_pos_vec = old_pos.translation() + global_displacement;
        let new_rot_rad = wrap_angle_rad(old_pos.rotation().radians() + delta_angle_rad);

        Pose2d::new(new_pos_vec.x(), new_pos_vec.y(), Rotation2d::new(new_rot_rad))
    }
}

impl Odometry for Odometry3Wheel {
    fn get_position(&self) -> Pose2d {
        self.state.current_pos
    }

    fn set_position(&mut self, new_pos: Pose2d) {
        self.state.current_pos = new_pos;
    }

    fn update(&mut self) -> Pose2d {
        let (lside, rside, offax) = (self.encoder_readings)();

        let lside_delta = lside - self.lside_old_deg;
        let rside_delta = rside - self.rside_old_deg;
        let offax_delta = offax - self.offax_old_deg;

        self.lside_old_deg = lside;
        self.rside_old_deg = rside;
        self.offax_old_deg = offax;

        let updated_pos = Self::calculate_new_pos(
            lside_delta,
            rside_delta,
            offax_delta,
            &self.state.current_pos,
            &self.cfg,
        );

        let elapsed = self.timer.time_sec();
        if elapsed > 0.1 {
            let speed = updated_pos
                .translation()
                .distance(self.last_pos.translation())
                / elapsed;
            let accel = (speed - self.last_speed) / elapsed;

            let ang_speed = smallest_angle(
                updated_pos.rotation().degrees(),
                self.last_pos.rotation().degrees(),
            ) / elapsed;
            let ang_accel = (ang_speed - self.last_ang_speed) / elapsed;

            self.timer.reset();
            self.last_pos = updated_pos;
            self.last_speed = speed;
            self.last_ang_speed = ang_speed;

            self.state.speed = speed;
            self.state.accel = accel;
            self.state.ang_speed_deg = ang_speed;
            self.state.ang_accel_deg = ang_accel;
        }

        self.state.current_pos = updated_pos;
        self.state.current_pos
    }

    fn get_speed(&self) -> f64 {
        self.state.speed
    }

    fn get_accel(&self) -> f64 {
        self.state.accel
    }

    fn get_angular_speed_deg(&self) -> f64 {
        self.state.ang_speed_deg
    }

    fn get_angular_accel_deg(&self) -> f64 {
        self.state.ang_accel_deg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odometry_3wheel_translation() {
        let cfg = Odometry3WheelConfig {
            wheelbase_dist: 10.0,
            off_axis_center_dist: 5.0,
            wheel_diam: 2.75,
        };

        let old_pos = Pose2d::new(0.0, 0.0, Rotation2d::from_degrees(90.0));
        // Rotate 360 deg forward for both parallel wheels
        let delta_deg = 360.0;
        let new_pos =
            Odometry3Wheel::calculate_new_pos(delta_deg, delta_deg, 0.0, &old_pos, &cfg);

        let expected_dist = PI * 2.75;
        assert!((new_pos.y() - expected_dist).abs() < 1e-4);
        assert!(new_pos.x().abs() < 1e-4);
    }
}
