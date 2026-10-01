//! 2D Pose representation (position and heading).

use super::rotation2d::Rotation2d;
use super::transform2d::Transform2d;
use super::translation2d::Translation2d;
use super::twist2d::Twist2d;
use nalgebra::Vector3;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

/// Represents a 2D pose with translational (x, y) and rotational (theta) components.
#[derive(Debug, Clone, Copy, Default)]
pub struct Pose2d {
    m_translation: Translation2d,
    m_rotation: Rotation2d,
}

impl Pose2d {
    /// Constructs a pose at the origin with zero rotation.
    pub const fn new_origin() -> Self {
        Self {
            m_translation: Translation2d::new_origin(),
            m_rotation: Rotation2d::ZERO,
        }
    }

    /// Constructs a pose with x, y, and rotation.
    pub const fn new(x: f64, y: f64, rotation: Rotation2d) -> Self {
        Self {
            m_translation: Translation2d::new(x, y),
            m_rotation: rotation,
        }
    }

    /// Constructs a pose with given translation and rotation.
    pub const fn from_translation(translation: Translation2d, rotation: Rotation2d) -> Self {
        Self {
            m_translation: translation,
            m_rotation: rotation,
        }
    }

    /// Constructs a pose with x, y, and rotation.
    pub const fn from_xy_rotation(x: f64, y: f64, rotation: Rotation2d) -> Self {
        Self::new(x, y, rotation)
    }

    /// Constructs a pose with rotation given in degrees.
    pub fn from_degrees(degrees: f64) -> Self {
        Self::new(0.0, 0.0, Rotation2d::from_degrees(degrees))
    }

    /// Sets the rotation component of this pose.
    pub fn set_rotation(&mut self, rotation: Rotation2d) {
        self.m_rotation = rotation;
    }

    /// Sets the translation component of this pose.
    pub fn set_translation(&mut self, translation: Translation2d) {
        self.m_translation = translation;
    }

    /// Constructs a pose with x, y, and radians.
    pub fn from_xy_radians(x: f64, y: f64, radians: f64) -> Self {
        Self {
            m_translation: Translation2d::new(x, y),
            m_rotation: Rotation2d::new(radians),
        }
    }

    /// Constructs a pose with translation and radians.
    pub fn from_translation_radians(translation: Translation2d, radians: f64) -> Self {
        Self {
            m_translation: translation,
            m_rotation: Rotation2d::new(radians),
        }
    }

    /// Constructs a pose from a 3-element vector `[x, y, theta]`.
    pub fn from_vector(v: Vector3<f64>) -> Self {
        Self {
            m_translation: Translation2d::new(v[0], v[1]),
            m_rotation: Rotation2d::new(v[2]),
        }
    }

    /// Returns the translational component.
    pub const fn translation(&self) -> Translation2d {
        self.m_translation
    }

    /// Returns the x component.
    pub const fn x(&self) -> f64 {
        self.m_translation.x()
    }

    /// Returns the y component.
    pub const fn y(&self) -> f64 {
        self.m_translation.y()
    }

    /// Returns the rotational component.
    pub const fn rotation(&self) -> Rotation2d {
        self.m_rotation
    }

    /// Sets the rotation in radians.
    pub fn set_rotation_rad(&mut self, rad: f64) {
        self.m_rotation = Rotation2d::new(rad);
    }

    /// Sets the rotation in degrees.
    pub fn set_rotation_deg(&mut self, deg: f64) {
        self.m_rotation = Rotation2d::from_degrees(deg);
    }

    /// Finds the pose equivalent to this pose relative to another arbitrary reference pose.
    pub fn relative_to(&self, other: &Pose2d) -> Pose2d {
        let diff_trans = (self.translation() - other.translation()).rotate_by(-other.rotation());
        let diff_rot = self.rotation() - other.rotation();
        Pose2d::from_translation(diff_trans, diff_rot)
    }

    /// Transforms the pose by a given transformation in the robot frame.
    pub fn transform_by(&self, transform: &Transform2d) -> Pose2d {
        Pose2d::from_translation(
            self.translation() + transform.translation().rotate_by(self.rotation()),
            self.rotation() + transform.rotation(),
        )
    }

    /// Applies a twist (pose delta) along an arc to this pose using Lie algebra exponential `exp(twist)`.
    pub fn exp(&self, twist: &Twist2d) -> Pose2d {
        let dx = twist.dx();
        let dy = twist.dy();
        let dtheta = twist.dtheta();

        let sin_theta = dtheta.sin();
        let cos_theta = dtheta.cos();

        let (s, c) = if dtheta.abs() < 1e-9 {
            (1.0 - 1.0 / 6.0 * dtheta * dtheta, 0.5 * dtheta)
        } else {
            (sin_theta / dtheta, (1.0 - cos_theta) / dtheta)
        };

        let transform = Transform2d::new(
            Translation2d::new(dx * s - dy * c, dx * c + dy * s),
            Rotation2d::from_xy(cos_theta, sin_theta),
        );

        *self + transform
    }

    /// The inverse of the pose exponential: finds the twist required to travel from `self` to `end_pose`.
    pub fn log(&self, end_pose: &Pose2d) -> Twist2d {
        let transform = end_pose.relative_to(self);
        let dtheta = transform.rotation().radians();
        let half_dtheta = dtheta / 2.0;

        let cos_minus_one = transform.rotation().f_cos() - 1.0;

        let half_theta_by_tan = if cos_minus_one.abs() < 1e-9 {
            1.0 - 1.0 / 12.0 * dtheta * dtheta
        } else {
            -(half_dtheta * transform.rotation().f_sin()) / cos_minus_one
        };

        let rot = Rotation2d::from_xy(half_theta_by_tan, -half_dtheta);
        let trans = transform.translation().rotate_by(rot) * half_theta_by_tan.hypot(half_dtheta);

        Twist2d::new(trans.x(), trans.y(), dtheta)
    }
}

impl Add<Transform2d> for Pose2d {
    type Output = Self;
    fn add(self, transform: Transform2d) -> Self {
        self.transform_by(&transform)
    }
}

impl Sub for Pose2d {
    type Output = Transform2d;
    fn sub(self, other: Self) -> Transform2d {
        let pose_diff = self.relative_to(&other);
        Transform2d::new(pose_diff.translation(), pose_diff.rotation())
    }
}

impl Mul<f64> for Pose2d {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self::from_translation(self.m_translation * scalar, self.m_rotation * scalar)
    }
}

impl Div<f64> for Pose2d {
    type Output = Self;
    fn div(self, scalar: f64) -> Self {
        self * (1.0 / scalar)
    }
}

impl PartialEq for Pose2d {
    fn eq(&self, other: &Self) -> bool {
        self.m_translation == other.m_translation && self.m_rotation == other.m_rotation
    }
}

impl fmt::Display for Pose2d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Pose2d[x: {}, y: {}, rad: {}, deg: {}]",
            self.x(),
            self.y(),
            self.rotation().radians(),
            self.rotation().degrees()
        )
    }
}

/// Calculates the mean of a slice of poses.
pub fn wrapped_mean(list: &[Pose2d]) -> Pose2d {
    if list.is_empty() {
        return Pose2d::new_origin();
    }
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_sin = 0.0;
    let mut sum_cos = 0.0;
    let len = list.len() as f64;

    for p in list {
        sum_x += p.x();
        sum_y += p.y();
        sum_sin += p.rotation().f_sin();
        sum_cos += p.rotation().f_cos();
    }

    Pose2d::from_translation(
        Translation2d::new(sum_x / len, sum_y / len),
        Rotation2d::from_xy(sum_cos / len, sum_sin / len),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pose2d_exp_log() {
        let p1 = Pose2d::new_origin();
        let twist = Twist2d::new(1.0, 0.0, 0.0);
        let p2 = p1.exp(&twist);
        assert!((p2.x() - 1.0).abs() < 1e-9);
        assert!((p2.y() - 0.0).abs() < 1e-9);

        let reconstructed_twist = p1.log(&p2);
        assert!((reconstructed_twist.dx() - twist.dx()).abs() < 1e-9);
        assert!((reconstructed_twist.dy() - twist.dy()).abs() < 1e-9);
        assert!((reconstructed_twist.dtheta() - twist.dtheta()).abs() < 1e-9);
    }
}
