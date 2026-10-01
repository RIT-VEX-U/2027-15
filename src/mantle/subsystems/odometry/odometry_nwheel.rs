//! Generalized N-pod tracking wheel odometry using SVD pseudoinverse kinematics.
//!
//! Mirrors `core/subsystems/odometry/odometry_nwheel.h`.
//! Integrates wheel displacements via Lie group $SE(2)$ matrix exponential.

use nalgebra::{DMatrix, DVector};
use super::odometry_base::{smallest_angle, Odometry, OdometryState};
use crate::core::geometry::{Pose2d, Rotation2d, Twist2d};
use crate::core::time::Timer;

/// Configuration parameters for an arbitrary unpowered tracking wheel pod.
#[derive(Debug, Clone, Copy)]
pub struct TrackingWheelConfig {
    /// X coordinate of the wheel center in the robot frame (inches).
    pub x: f64,
    /// Y coordinate of the wheel center in the robot frame (inches).
    pub y: f64,
    /// Angle between wheel rolling direction and positive X axis in robot frame (radians).
    pub theta_rad: f64,
    /// Radius of the tracking wheel (inches).
    pub radius: f64,
}

/// N-wheel generalized odometry tracking system.
pub struct OdometryNWheel {
    wheel_configs: Vec<TrackingWheelConfig>,
    encoder_readings: Box<dyn Fn() -> Vec<f64> + Send + Sync>,
    imu_reading_rad: Option<Box<dyn Fn() -> f64 + Send + Sync>>,
    transfer_matrix_pinv: DMatrix<f64>,
    wheel_radii: DVector<f64>,
    old_wheel_angles: DVector<f64>,
    state: OdometryState,
    angle_offset: f64,
    old_imu_angle: f64,
    timer: Timer,
    last_pos: Pose2d,
    last_speed: f64,
    last_ang_speed: f64,
}

impl OdometryNWheel {
    /// Creates a new N-wheel odometry tracker for an arbitrary number of tracking wheels.
    ///
    /// # Parameters
    /// - `wheel_configs`: Physical mounting geometry for each wheel pod.
    /// - `encoder_readings`: Closure returning the current wheel angle in radians for each wheel.
    /// - `imu_reading_rad`: Optional IMU yaw angle reader in radians.
    pub fn new(
        wheel_configs: Vec<TrackingWheelConfig>,
        encoder_readings: impl Fn() -> Vec<f64> + Send + Sync + 'static,
        imu_reading_rad: Option<Box<dyn Fn() -> f64 + Send + Sync>>,
    ) -> Self {
        let n = wheel_configs.len();
        let mut transfer_matrix = DMatrix::<f64>::zeros(n, 3);
        let mut wheel_radii = DVector::<f64>::zeros(n);

        for (i, cfg) in wheel_configs.iter().enumerate() {
            wheel_radii[i] = cfg.radius;

            let mut x_factor = cfg.theta_rad.cos();
            let mut y_factor = -cfg.theta_rad.sin();
            let theta_factor = -(cfg.x * cfg.theta_rad.sin()) - (cfg.y * cfg.theta_rad.cos());

            if y_factor.abs() < 1e-9 {
                y_factor = 0.0;
            }
            if x_factor.abs() < 1e-9 {
                x_factor = 0.0;
            }

            transfer_matrix[(i, 0)] = x_factor;
            transfer_matrix[(i, 1)] = y_factor;
            transfer_matrix[(i, 2)] = theta_factor;
        }

        // Complete orthogonal decomposition / SVD pseudoinverse
        let svd = transfer_matrix.svd(true, true);
        let transfer_matrix_pinv = svd
            .pseudo_inverse(1e-9)
            .unwrap_or_else(|_| DMatrix::<f64>::zeros(3, n));

        Self {
            wheel_configs,
            encoder_readings: Box::new(encoder_readings),
            imu_reading_rad,
            transfer_matrix_pinv,
            wheel_radii,
            old_wheel_angles: DVector::<f64>::zeros(n),
            state: OdometryState::new(),
            angle_offset: 0.0,
            old_imu_angle: 0.0,
            timer: Timer::new(),
            last_pos: Pose2d::new(0.0, 0.0, Rotation2d::new(0.0)),
            last_speed: 0.0,
            last_ang_speed: 0.0,
        }
    }

    /// Advances the robot pose using wheel angle radian deltas and $SE(2)$ exponential mapping.
    pub fn calculate_new_pos(
        &self,
        radian_deltas: &DVector<f64>,
        old_pose: &Pose2d,
        imu_angle: Option<f64>,
    ) -> Pose2d {
        let linear_deltas = radian_deltas.component_mul(&self.wheel_radii);
        let mut pose_delta = &self.transfer_matrix_pinv * linear_deltas;

        if let Some(angle) = imu_angle {
            pose_delta[2] = angle - self.old_imu_angle;
        }

        let twist = Twist2d::new(pose_delta[0], pose_delta[1], pose_delta[2]);
        let mut new_pose = old_pose.exp(&twist);

        if let Some(angle) = imu_angle {
            new_pose.set_rotation(Rotation2d::new(angle));
        }

        new_pose
    }
}

impl Odometry for OdometryNWheel {
    fn get_position(&self) -> Pose2d {
        self.state.current_pos
    }

    fn set_position(&mut self, new_pos: Pose2d) {
        self.angle_offset = new_pos.rotation().degrees()
            - (self.state.current_pos.rotation().degrees() - self.angle_offset);
        self.state.current_pos = new_pos;
    }

    fn update(&mut self) -> Pose2d {
        let current_angles = (self.encoder_readings)();
        let n = self.wheel_configs.len();
        let mut radian_deltas = DVector::<f64>::zeros(n);

        for i in 0..n {
            let angle = if i < current_angles.len() {
                current_angles[i]
            } else {
                0.0
            };
            radian_deltas[i] = angle - self.old_wheel_angles[i];
            self.old_wheel_angles[i] = angle;
        }

        let imu_angle = self.imu_reading_rad.as_ref().map(|f| -f() + self.angle_offset.to_radians());
        let updated_pos = self.calculate_new_pos(&radian_deltas, &self.state.current_pos, imu_angle);

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

        if let Some(angle) = imu_angle {
            self.old_imu_angle = angle;
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
    use std::f64::consts::PI;

    #[test]
    fn test_nwheel_two_parallel_configuration() {
        // Two parallel wheels at y = +5 and y = -5, rolling along x axis (theta = 0)
        let configs = vec![
            TrackingWheelConfig {
                x: 0.0,
                y: 5.0,
                theta_rad: 0.0,
                radius: 1.0,
            },
            TrackingWheelConfig {
                x: 0.0,
                y: -5.0,
                theta_rad: 0.0,
                radius: 1.0,
            },
        ];

        let odom = OdometryNWheel::new(configs, || vec![0.0, 0.0], None);
        let old_pose = Pose2d::new(0.0, 0.0, Rotation2d::new(0.0));

        // Both wheels roll 2*PI radians forward -> 2*PI linear inches
        let deltas = DVector::from_vec(vec![2.0 * PI, 2.0 * PI]);
        let new_pose = odom.calculate_new_pos(&deltas, &old_pose, None);

        assert!((new_pose.x() - 2.0 * PI).abs() < 1e-4);
        assert!(new_pose.y().abs() < 1e-4);
    }
}
