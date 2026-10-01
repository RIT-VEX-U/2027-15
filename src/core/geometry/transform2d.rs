//! 2D Transformation representation (translation and rotation change).

use super::rotation2d::Rotation2d;
use super::translation2d::Translation2d;
use nalgebra::Vector3;
use std::fmt;
use std::ops::{Div, Mul, Neg};

/// Represents a rigid transformation in 2D space (SE(2) group element).
#[derive(Debug, Clone, Copy, Default)]
pub struct Transform2d {
    m_translation: Translation2d,
    m_rotation: Rotation2d,
}

impl Transform2d {
    /// Constructs an identity transformation.
    pub fn new_identity() -> Self {
        Self {
            m_translation: Translation2d::new_origin(),
            m_rotation: Rotation2d::default(),
        }
    }

    /// Constructs a transform from translation and rotation.
    pub fn new(translation: Translation2d, rotation: Rotation2d) -> Self {
        Self {
            m_translation: translation,
            m_rotation: rotation,
        }
    }

    /// Constructs a transform from x, y, and rotation.
    pub fn from_xy_rotation(x: f64, y: f64, rotation: Rotation2d) -> Self {
        Self {
            m_translation: Translation2d::new(x, y),
            m_rotation: rotation,
        }
    }

    /// Constructs a transform from x, y, and radians.
    pub fn from_xy_radians(x: f64, y: f64, radians: f64) -> Self {
        Self {
            m_translation: Translation2d::new(x, y),
            m_rotation: Rotation2d::new(radians),
        }
    }

    /// Constructs a transform from a translation and radians.
    pub fn from_translation_radians(translation: Translation2d, radians: f64) -> Self {
        Self {
            m_translation: translation,
            m_rotation: Rotation2d::new(radians),
        }
    }

    /// Constructs a transform from a 3-element vector `[x, y, theta]`.
    pub fn from_vector(v: Vector3<f64>) -> Self {
        Self {
            m_translation: Translation2d::new(v[0], v[1]),
            m_rotation: Rotation2d::new(v[2]),
        }
    }

    /// Returns the translational component.
    pub fn translation(&self) -> Translation2d {
        self.m_translation
    }

    /// Returns the x component.
    pub fn x(&self) -> f64 {
        self.m_translation.x()
    }

    /// Returns the y component.
    pub fn y(&self) -> f64 {
        self.m_translation.y()
    }

    /// Returns the rotational component.
    pub fn rotation(&self) -> Rotation2d {
        self.m_rotation
    }

    /// Inverts the transformation.
    pub fn inverse(&self) -> Self {
        Self {
            m_translation: (-self.m_translation).rotate_by(-self.m_rotation),
            m_rotation: -self.m_rotation,
        }
    }
}

impl Mul<f64> for Transform2d {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self {
            m_translation: self.m_translation * scalar,
            m_rotation: self.m_rotation * scalar,
        }
    }
}

impl Div<f64> for Transform2d {
    type Output = Self;
    fn div(self, scalar: f64) -> Self {
        Self {
            m_translation: self.m_translation / scalar,
            m_rotation: self.m_rotation / scalar,
        }
    }
}

impl Neg for Transform2d {
    type Output = Self;
    fn neg(self) -> Self {
        self.inverse()
    }
}

impl PartialEq for Transform2d {
    fn eq(&self, other: &Self) -> bool {
        self.m_translation == other.m_translation && self.m_rotation == other.m_rotation
    }
}

impl fmt::Display for Transform2d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Transform2d[dx: {}, dy: {}, drad: {}, ddeg: {}]",
            self.x(),
            self.y(),
            self.rotation().radians(),
            self.rotation().degrees()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform2d() {
        let t = Transform2d::from_xy_radians(1.0, 2.0, 0.0);
        let inv = t.inverse();
        assert_eq!(inv.x(), -1.0);
        assert_eq!(inv.y(), -2.0);
        assert_eq!(inv.rotation().radians(), 0.0);
    }
}
