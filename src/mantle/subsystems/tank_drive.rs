//! Tank / differential drivetrain subsystem.
//!
//! Mirrors `core/subsystems/tank_drive.h` and `core/subsystems/tank_drive.cpp`.

use super::odometry::{smallest_angle, Odometry};
use crate::core::controls::feedback_base::Feedback;
use crate::core::controls::pid::{Pid, PidConfig};
use crate::core::geometry::{Pose2d, Rotation2d, Translation2d};
use crate::core::math::math_util::{clamp, estimate_path_length, sign};
use crate::core::pathing::pure_pursuit::{estimate_remaining_dist, get_lookahead, Path};
use crate::mantle::motor::{DirectionType, MotorGroup, VoltageUnits};
use std::f64::consts::PI;
use std::sync::{Arc, Mutex};

/// Physical and algorithmic configuration specification for differential / tank drive.
#[derive(Debug, Clone)]
pub struct RobotSpecs {
    /// Radius of the bounding circle containing the robot (inches).
    pub robot_radius: f64,
    /// Diameter of tracking / odometry wheels (inches).
    pub odom_wheel_diam: f64,
    /// Gear ratio from tracking wheel to encoder (e.g., 1.0).
    pub odom_gear_ratio: f64,
    /// Track width: distance between wheel centers of the central drive wheels (inches).
    pub dist_between_wheels: f64,
    /// Threshold distance within which heading correction towards a target point is suppressed (inches).
    pub drive_correction_cutoff: f64,
    /// Configuration for the heading correction PID loop while driving straight.
    pub correction_pid: PidConfig,
}

impl Default for RobotSpecs {
    fn default() -> Self {
        Self {
            robot_radius: 9.0,
            odom_wheel_diam: 2.75,
            odom_gear_ratio: 1.0,
            dist_between_wheels: 12.0,
            drive_correction_cutoff: 2.0,
            correction_pid: PidConfig {
                p: 0.05,
                i: 0.0,
                d: 0.005,
                ..Default::default()
            },
        }
    }
}

/// Drivetrain braking modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrakeType {
    /// Send 0 volts to the motors (coasting or motor brake mode).
    None,
    /// Actively decelerate to bring robot velocity to zero without holding position.
    ZeroVelocity,
    /// Bring robot to rest and actively hold the position using closed-loop control.
    Smart,
    /// Turn only brake mode.
    TurnOnly,
}

/// Differential / tank drive subsystem with autonomous trajectory following,
/// arcade/tank teleop, and odometry integration.
pub struct TankDrive {
    left_motors: MotorGroup,
    right_motors: MotorGroup,
    pub odometry: Option<Arc<Mutex<dyn Odometry>>>,
    config: RobotSpecs,
    correction_pid: Pid,
    drive_default_feedback: Option<Box<dyn Feedback>>,
    turn_default_feedback: Option<Box<dyn Feedback>>,
    arcade_throttle: f64,
    func_initialized: bool,
    is_pure_pursuit: bool,
    captured_position: bool,
    was_braking: bool,
    zero_vel_pid: Pid,
    target_pose: Pose2d,
    drive_forward_pt: Pose2d,
    target_heading: f64,
}

impl TankDrive {
    /// Creates a new TankDrive subsystem instance.
    pub fn new(
        left_motors: MotorGroup,
        right_motors: MotorGroup,
        config: RobotSpecs,
        odometry: Option<Arc<Mutex<dyn Odometry>>>,
    ) -> Self {
        let correction_pid = Pid::new(config.correction_pid);
        let zero_vel_pid = Pid::new(crate::core::controls::pid::PidConfig {
            p: 0.005,
            d: 0.0005,
            ..Default::default()
        });

        Self {
            left_motors,
            right_motors,
            odometry,
            config,
            correction_pid,
            drive_default_feedback: None,
            turn_default_feedback: None,
            arcade_throttle: 0.0,
            func_initialized: false,
            is_pure_pursuit: false,
            captured_position: false,
            was_braking: false,
            zero_vel_pid,
            target_pose: Pose2d::new(0.0, 0.0, Rotation2d::new(0.0)),
            drive_forward_pt: Pose2d::new(0.0, 0.0, Rotation2d::new(0.0)),
            target_heading: 0.0,
        }
    }

    /// Sets default feedback controllers for driving and turning.
    pub fn set_default_feedbacks(
        &mut self,
        drive_fb: Box<dyn Feedback>,
        turn_fb: Box<dyn Feedback>,
    ) {
        self.drive_default_feedback = Some(drive_fb);
        self.turn_default_feedback = Some(turn_fb);
    }

    /// Stops both motor groups and resets arcade throttle.
    pub fn stop(&mut self) {
        self.arcade_throttle = 0.0;
        self.left_motors.stop();
        self.right_motors.stop();
    }

    /// Resets autonomous function initialization flags.
    pub fn reset_auto(&mut self) {
        self.func_initialized = false;
        self.is_pure_pursuit = false;
    }

    /// Returns the current robot position from odometry (or (0,0,0) if no odometry is attached).
    pub fn get_position(&self) -> Pose2d {
        if let Some(ref odom) = self.odometry {
            odom.lock().unwrap().get_position()
        } else {
            Pose2d::new(0.0, 0.0, Rotation2d::new(0.0))
        }
    }

    /// Drives motors directly using normalized voltage percentages in $[-1.0, 1.0]$.
    pub fn drive_tank_raw(&mut self, left_norm: f64, right_norm: f64) {
        self.left_motors
            .spin(DirectionType::Forward, left_norm * 12.0, VoltageUnits::Volt);
        self.right_motors.spin(
            DirectionType::Forward,
            right_norm * 12.0,
            VoltageUnits::Volt,
        );
    }

    /// Modifies user inputs with exponential power curve while preserving sign.
    pub fn modify_inputs(input: f64, power: i32) -> f64 {
        sign(input) * input.abs().powi(power)
    }

    /// Tank drive control with optional power curves and active braking.
    pub fn drive_tank(&mut self, mut left: f64, mut right: f64, power: i32, bt: BrakeType) {
        left = Self::modify_inputs(left, power);
        right = Self::modify_inputs(right, power);
        let brake_threshold = 0.05;
        let should_brake =
            bt != BrakeType::None && left.abs() < brake_threshold && right.abs() < brake_threshold;

        if !should_brake {
            self.drive_tank_raw(left, right);
            self.was_braking = false;
            return;
        }

        if should_brake && !self.was_braking {
            self.captured_position = false;
        }

        match bt {
            BrakeType::ZeroVelocity => {
                self.zero_vel_pid.set_target(0.0);
                let vel = self.left_motors.velocity() + self.right_motors.velocity();
                let outp = self.zero_vel_pid.update(vel);
                self.left_motors
                    .spin(DirectionType::Forward, outp, VoltageUnits::Volt);
                self.right_motors
                    .spin(DirectionType::Forward, outp, VoltageUnits::Volt);
            }
            BrakeType::Smart => {
                if let Some(ref odom) = self.odometry {
                    let (speed, current_pos) = {
                        let o = odom.lock().unwrap();
                        (o.get_speed(), o.get_position())
                    };

                    if speed.abs() <= 0.01 && !self.captured_position {
                        self.target_pose = current_pos;
                        self.captured_position = true;
                    } else if self.captured_position {
                        let dist_to_target = self
                            .target_pose
                            .translation()
                            .distance(current_pos.translation());
                        if dist_to_target < 12.0 {
                            if let Some(mut fb) = self.drive_default_feedback.take() {
                                self.drive_to_point(
                                    self.target_pose.x(),
                                    self.target_pose.y(),
                                    DirectionType::Forward,
                                    &mut *fb,
                                    1.0,
                                    0.0,
                                );
                                self.drive_default_feedback = Some(fb);
                            }
                        } else {
                            self.target_pose = current_pos;
                            self.reset_auto();
                        }
                    } else {
                        self.zero_vel_pid.set_target(0.0);
                        let outp = self.zero_vel_pid.update(speed);
                        self.left_motors
                            .spin(DirectionType::Forward, outp, VoltageUnits::Volt);
                        self.right_motors
                            .spin(DirectionType::Forward, outp, VoltageUnits::Volt);
                    }
                }
            }
            _ => {
                self.stop();
            }
        }

        self.was_braking = should_brake;
    }

    /// Curvature / arcade drive control with deadband, slew rate limiting, and turn scaling.
    pub fn drive_arcade(
        &mut self,
        mut forward_back: f64,
        mut left_right: f64,
        power: i32,
        turn_power: i32,
        bt: BrakeType,
        deadband: f64,
        slew_rate: f64,
        scale_turn: bool,
    ) {
        if deadband > 0.0 && deadband < 1.0 {
            let temp_drive = clamp(forward_back, -1.0, 1.0);
            let temp_turn = clamp(left_right, -1.0, 1.0);

            let mag_drive = temp_drive.abs();
            let mag_turn = temp_turn.abs();

            let mag_drive_scaled = if mag_drive <= deadband {
                0.0
            } else {
                (mag_drive - deadband) / (1.0 - deadband)
            };
            let mag_turn_scaled = if mag_turn <= deadband {
                0.0
            } else {
                (mag_turn - deadband) / (1.0 - deadband)
            };

            forward_back = mag_drive_scaled.copysign(temp_drive);
            left_right = mag_turn_scaled.copysign(temp_turn);
        }

        let turn_scale_blend = clamp(forward_back.abs() / 0.10, 0.0, 1.0);
        forward_back = Self::modify_inputs(forward_back, power);
        left_right = Self::modify_inputs(left_right, turn_power);

        let turn_in_place = scale_turn && forward_back == 0.0 && left_right != 0.0;

        if turn_in_place {
            self.arcade_throttle = 0.0;
        } else if slew_rate > 0.0 {
            self.arcade_throttle +=
                clamp(forward_back - self.arcade_throttle, -slew_rate, slew_rate);
        } else {
            self.arcade_throttle = forward_back;
        }

        forward_back = self.arcade_throttle;
        if scale_turn && !turn_in_place {
            let throttle_turn_scale = forward_back.abs();
            let turn_scale = 1.0 + turn_scale_blend * (throttle_turn_scale - 1.0);
            left_right *= turn_scale;
        }

        let mut left = forward_back + left_right;
        let mut right = forward_back - left_right;
        if scale_turn {
            let magnitude = 1.0_f64.max(left.abs()).max(right.abs());
            left /= magnitude;
            right /= magnitude;
        }

        self.drive_tank(left, right, 1, bt);
    }

    /// Drives forward/reverse a distance in inches using closed-loop feedback and straight-line correction.
    pub fn drive_forward(
        &mut self,
        inches: f64,
        dir: DirectionType,
        feedback: &mut dyn Feedback,
        max_speed: f64,
        end_speed: f64,
    ) -> bool {
        if self.odometry.is_none() {
            eprintln!("Odometry is None. Unable to run drive_forward()");
            return true;
        }

        if !self.func_initialized {
            let cur_pos = self.get_position();
            let signed_inches = if dir == DirectionType::Reverse {
                -inches.abs()
            } else {
                inches.abs()
            };

            let delta_pos = Translation2d::from_polar(signed_inches, cur_pos.rotation());
            let target_vec = cur_pos.translation() + delta_pos;
            self.drive_forward_pt = Pose2d::new(target_vec.x(), target_vec.y(), cur_pos.rotation());
        }

        self.drive_to_point(
            self.drive_forward_pt.x(),
            self.drive_forward_pt.y(),
            dir,
            feedback,
            max_speed,
            end_speed,
        )
    }

    /// Turns in place by relative degrees.
    pub fn turn_degrees(
        &mut self,
        degrees: f64,
        feedback: &mut dyn Feedback,
        max_speed: f64,
        end_speed: f64,
    ) -> bool {
        if self.odometry.is_none() {
            eprintln!("Odometry is None. Unable to run turn_degrees()");
            return true;
        }

        if !self.func_initialized {
            let start_heading = self.get_position().rotation().degrees();
            self.target_heading = start_heading + degrees;
        }

        self.turn_to_heading(self.target_heading, feedback, max_speed, end_speed)
    }

    /// Drives to a global $(X, Y)$ coordinate point on the field using feedback control.
    pub fn drive_to_point(
        &mut self,
        x: f64,
        y: f64,
        dir: DirectionType,
        feedback: &mut dyn Feedback,
        max_speed: f64,
        end_speed: f64,
    ) -> bool {
        if self.odometry.is_none() {
            eprintln!("Odometry is None. Unable to run drive_to_point()");
            return true;
        }

        let current_pos = self.get_position();
        let end_trans = Translation2d::new(x, y);

        if !self.func_initialized {
            let initial_dist = current_pos.translation().distance(end_trans);
            self.correction_pid.init(0.0, 0.0);
            feedback.init(-initial_dist, 0.0);
            self.correction_pid.set_limits(-1.0, 1.0);
            feedback.set_limits(-1.0, 1.0);
            self.func_initialized = true;
        }

        let mut dist_left = current_pos.translation().distance(end_trans);
        let mut sign_dir = 1.0;

        let angle_to_point = (y - current_pos.y())
            .atan2(x - current_pos.x())
            .to_degrees();
        let mut angle = (current_pos.rotation().degrees() - angle_to_point) % 360.0;
        if angle > 360.0 {
            angle -= 360.0;
        }
        if angle < 0.0 {
            angle += 360.0;
        }

        if dir == DirectionType::Forward && angle > 90.0 && angle < 270.0 {
            sign_dir = -1.0;
        } else if dir == DirectionType::Reverse && (angle < 90.0 || angle > 270.0) {
            sign_dir = -1.0;
        }

        if dist_left.abs() < self.config.drive_correction_cutoff {
            dist_left *= angle.to_radians().cos().abs();
        }

        let heading = (y - current_pos.y())
            .atan2(x - current_pos.x())
            .to_degrees();
        let delta_heading = if dir == DirectionType::Forward {
            smallest_angle(current_pos.rotation().degrees(), heading)
        } else {
            smallest_angle(current_pos.rotation().degrees() - 180.0, heading)
        };

        self.correction_pid.update(delta_heading);
        feedback.update(-sign_dir * dist_left);

        let mut correction = 0.0;
        if self.is_pure_pursuit || dist_left.abs() > self.config.drive_correction_cutoff {
            correction = self.correction_pid.get();
        }

        let drive_val = if dir == DirectionType::Reverse {
            -feedback.get()
        } else {
            feedback.get()
        };

        let lside = clamp(drive_val + correction, -max_speed, max_speed);
        let rside = clamp(drive_val - correction, -max_speed, max_speed);

        self.drive_tank_raw(lside, rside);

        if feedback.is_on_target() {
            if end_speed == 0.0 {
                self.stop();
            }
            self.func_initialized = false;
            return true;
        }

        false
    }

    /// Turns in place to an absolute global heading in degrees.
    pub fn turn_to_heading(
        &mut self,
        heading_deg: f64,
        feedback: &mut dyn Feedback,
        max_speed: f64,
        _end_speed: f64,
    ) -> bool {
        if self.odometry.is_none() {
            eprintln!("Odometry is None. Unable to run turn_to_heading()");
            return true;
        }

        let cur_heading = self.get_position().rotation().degrees();

        if !self.func_initialized {
            let initial_delta = smallest_angle(cur_heading, heading_deg);
            feedback.init(-initial_delta, 0.0);
            feedback.set_limits(-max_speed.abs(), max_speed.abs());
            self.func_initialized = true;
        }

        let delta_heading = smallest_angle(cur_heading, heading_deg);
        feedback.update(-delta_heading);

        self.drive_tank_raw(-feedback.get(), feedback.get());

        if feedback.is_on_target() {
            self.func_initialized = false;
            self.stop();
            return true;
        }

        false
    }

    /// Pure pursuit path following algorithm.
    pub fn pure_pursuit(
        &mut self,
        path: &Path,
        dir: DirectionType,
        feedback: &mut dyn Feedback,
        max_speed: f64,
        _end_speed: f64,
    ) -> bool {
        let points = path.get_points();
        if points.is_empty() {
            return true;
        }

        let robot_pose = self.get_position();

        if !self.func_initialized {
            let total_len = estimate_path_length(points);
            if dir != DirectionType::Reverse {
                feedback.init(-total_len, 0.0);
            } else {
                feedback.init(total_len, 0.0);
            }
            self.is_pure_pursuit = true;
            self.func_initialized = true;
        }

        let lookahead = get_lookahead(points, robot_pose, path.get_radius());
        let localized = lookahead - robot_pose.translation();
        let last_point = points[points.len() - 1];
        let is_last_point = lookahead == last_point;

        let mut correction = 0.0;
        let mut dist_remaining = estimate_remaining_dist(points, robot_pose, path.get_radius());

        let target_angle_deg = localized.y().atan2(localized.x()).to_degrees();
        let angle_diff = if dir != DirectionType::Reverse {
            smallest_angle(robot_pose.rotation().degrees(), target_angle_deg)
        } else {
            smallest_angle(robot_pose.rotation().degrees() + 180.0, target_angle_deg)
        };

        if !(is_last_point
            && robot_pose.translation().distance(last_point) < self.config.drive_correction_cutoff)
        {
            self.correction_pid.update(angle_diff);
            correction = self.correction_pid.get();
        } else {
            dist_remaining *= (angle_diff * (PI / 180.0)).cos();
        }

        if dir != DirectionType::Reverse {
            feedback.update(-dist_remaining);
        } else {
            feedback.update(dist_remaining);
        }

        let max_speed = max_speed.abs();
        let mut left = clamp(feedback.get(), -max_speed, max_speed);
        let mut right = clamp(feedback.get(), -max_speed, max_speed);

        left += correction;
        right -= correction;

        self.drive_tank_raw(left, right);

        if is_last_point && feedback.is_on_target() {
            self.func_initialized = false;
            self.is_pure_pursuit = false;
            self.stop();
            return true;
        }

        false
    }
}
