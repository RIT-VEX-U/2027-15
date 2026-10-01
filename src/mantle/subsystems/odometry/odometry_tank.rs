//! Differential / tank drivetrain odometry implementation.
//!
//! Mirrors `core/subsystems/odometry/odometry_tank.h` and `core/subsystems/odometry/odometry_tank.cpp`.

use std::f64::consts::PI;
use super::super::tank_drive::RobotSpecs;
use super::odometry_base::{smallest_angle, Odometry, OdometryState};
use crate::core::geometry::{Pose2d, Rotation2d, Translation2d};
use crate::core::filter::{ExponentialMovingAverage, Filter};
use crate::core::time::Timer;

/// Sensor inputs provider for tank odometry.
pub enum TankWheelSensors {
    /// Left and right wheel revolution readers.
    Revolutions(Box<dyn Fn() -> (f64, f64) + Send + Sync>),
}

/// Odometry system for differential / tank drivetrains using wheel revolutions and optional IMU.
pub struct OdometryTank {
    sensors: TankWheelSensors,
    imu_angle_deg: Option<Box<dyn Fn() -> f64 + Send + Sync>>,
    config: RobotSpecs,
    state: OdometryState,
    stored_lside_revs: f64,
    stored_rside_revs: f64,
    rotation_offset: f64,
    ema: ExponentialMovingAverage,
    timer: Timer,
    last_pos: Pose2d,
    last_speed: f64,
    last_ang_speed: f64,
}

impl OdometryTank {
    /// Creates a new tank odometry tracker.
    pub fn new(
        sensors: TankWheelSensors,
        imu_angle_deg: Option<Box<dyn Fn() -> f64 + Send + Sync>>,
        config: RobotSpecs,
    ) -> Self {
        Self {
            sensors,
            imu_angle_deg,
            config,
            state: OdometryState::new(),
            stored_lside_revs: 0.0,
            stored_rside_revs: 0.0,
            rotation_offset: 0.0,
            ema: ExponentialMovingAverage::new(3),
            timer: Timer::new(),
            last_pos: Pose2d::new(0.0, 0.0, Rotation2d::new(0.0)),
            last_speed: 0.0,
            last_ang_speed: 0.0,
        }
    }

    /// Pure calculation method for advancing robot pose given revolution deltas.
    pub fn calculate_new_pos(
        config: &RobotSpecs,
        curr_pos: &Pose2d,
        lside_revs: f64,
        rside_revs: f64,
        stored_lside: f64,
        stored_rside: f64,
        angle_deg: f64,
    ) -> (Pose2d, f64, f64) {
        let lside_diff = (lside_revs - stored_lside) * PI * config.odom_wheel_diam;
        let rside_diff = (rside_revs - stored_rside) * PI * config.odom_wheel_diam;
        let dist_driven = (lside_diff + rside_diff) / 2.0;

        let angle_rad = angle_deg.to_radians();
        let chg_point = Translation2d::from_polar(dist_driven, Rotation2d::new(angle_rad));
        let curr_point = curr_pos.translation();
        let new_point = curr_point + chg_point;

        let new_pos = Pose2d::new(new_point.x(), new_point.y(), Rotation2d::from_degrees(angle_deg));
        (new_pos, lside_revs, rside_revs)
    }
}

impl Odometry for OdometryTank {
    fn get_position(&self) -> Pose2d {
        self.state.current_pos
    }

    fn set_position(&mut self, new_pos: Pose2d) {
        self.rotation_offset = new_pos.rotation().degrees()
            - (self.state.current_pos.rotation().degrees() - self.rotation_offset);
        self.state.current_pos = new_pos;
    }

    fn update(&mut self) -> Pose2d {
        let (raw_lside, raw_rside) = match &self.sensors {
            TankWheelSensors::Revolutions(f) => f(),
        };

        let lside_revs = raw_lside / self.config.odom_gear_ratio;
        let rside_revs = raw_rside / self.config.odom_gear_ratio;

        let angle = match &self.imu_angle_deg {
            None => {
                let distance_diff = (rside_revs - lside_revs) * PI * self.config.odom_wheel_diam;
                (180.0 / PI) * (distance_diff / self.config.dist_between_wheels)
            }
            Some(imu_fn) => -imu_fn(),
        };

        let mut angle = angle + self.rotation_offset;
        angle %= 360.0;
        if angle < 0.0 {
            angle += 360.0;
        }

        let (new_pos, next_stored_l, next_stored_r) = Self::calculate_new_pos(
            &self.config,
            &self.state.current_pos,
            lside_revs,
            rside_revs,
            self.stored_lside_revs,
            self.stored_rside_revs,
            angle,
        );

        self.stored_lside_revs = next_stored_l;
        self.stored_rside_revs = next_stored_r;
        self.state.current_pos = new_pos;

        let elapsed_sec = self.timer.time_sec();
        if elapsed_sec > 0.02 {
            let this_speed = self
                .state
                .current_pos
                .translation()
                .distance(self.last_pos.translation())
                / elapsed_sec;

            self.ema.add_entry(this_speed);
            self.state.speed = self.ema.get_value();
            self.state.accel = (self.state.speed - self.last_speed) / elapsed_sec;

            self.state.ang_speed_deg = smallest_angle(
                self.state.current_pos.rotation().degrees(),
                self.last_pos.rotation().degrees(),
            ) / elapsed_sec;

            self.state.ang_accel_deg =
                (self.state.ang_speed_deg - self.last_ang_speed) / elapsed_sec;

            self.timer.reset();
            self.last_pos = self.state.current_pos;
            self.last_speed = self.state.speed;
            self.last_ang_speed = self.state.ang_speed_deg;
        }

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
    fn test_odometry_tank_straight_drive() {
        let specs = RobotSpecs {
            odom_wheel_diam: 3.25,
            odom_gear_ratio: 1.0,
            dist_between_wheels: 10.0,
            ..Default::default()
        };

        let curr_pos = Pose2d::new(0.0, 0.0, Rotation2d::from_degrees(90.0));
        let (new_pos, _, _) =
            OdometryTank::calculate_new_pos(&specs, &curr_pos, 1.0, 1.0, 0.0, 0.0, 90.0);

        let dist_expected = 1.0 * PI * 3.25;
        assert!((new_pos.y() - dist_expected).abs() < 1e-4);
        assert!(new_pos.x().abs() < 1e-4);
        assert!((new_pos.rotation().degrees() - 90.0).abs() < 1e-4);
    }
}
