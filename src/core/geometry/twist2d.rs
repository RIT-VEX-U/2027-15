//! 2D Twist representation (linear dx, dy and angular dtheta along an arc).

use nalgebra::Vector3;
use std::fmt;
use std::ops::{Div, Mul};

/// Difference between two poses along an arc (Lie algebra se(2) element).
#[derive(Debug, Clone, Copy, Default)]
pub struct Twist2d {
    m_dx: f64,
    m_dy: f64,
    m_dtheta: f64,
}

impl Twist2d {
    /// Constructs a zero twist.
    pub const fn new_zero() -> Self {
        Self {
            m_dx: 0.0,
            m_dy: 0.0,
            m_dtheta: 0.0,
        }
    }

    /// Constructs a twist with specified dx, dy, and dtheta components.
    pub const fn new(dx: f64, dy: f64, dtheta: f64) -> Self {
        Self {
            m_dx: dx,
            m_dy: dy,
            m_dtheta: dtheta,
        }
    }

    /// Constructs a twist from a 3-element vector `[dx, dy, dtheta]`.
    pub fn from_vector(v: Vector3<f64>) -> Self {
        Self {
            m_dx: v[0],
            m_dy: v[1],
            m_dtheta: v[2],
        }
    }

    /// Returns linear dx component.
    pub const fn dx(&self) -> f64 {
        self.m_dx
    }

    /// Returns linear dy component.
    pub const fn dy(&self) -> f64 {
        self.m_dy
    }

    /// Returns angular dtheta component in radians.
    pub const fn dtheta(&self) -> f64 {
        self.m_dtheta
    }

    /// Returns the twist as a 3D vector.
    pub fn as_vector(&self) -> Vector3<f64> {
        Vector3::new(self.m_dx, self.m_dy, self.m_dtheta)
    }
}

impl Mul<f64> for Twist2d {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self {
            m_dx: self.m_dx * scalar,
            m_dy: self.m_dy * scalar,
            m_dtheta: self.m_dtheta * scalar,
        }
    }
}

impl Div<f64> for Twist2d {
    type Output = Self;
    fn div(self, scalar: f64) -> Self {
        Self {
            m_dx: self.m_dx / scalar,
            m_dy: self.m_dy / scalar,
            m_dtheta: self.m_dtheta / scalar,
        }
    }
}

impl PartialEq for Twist2d {
    fn eq(&self, other: &Self) -> bool {
        (self.m_dx - other.m_dx).abs() < 1e-9
            && (self.m_dy - other.m_dy).abs() < 1e-9
            && (self.m_dtheta - other.m_dtheta).abs() < 1e-9
    }
}

impl fmt::Display for Twist2d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Twist2d[dx: {}, dy: {}, drad: {}]",
            self.m_dx, self.m_dy, self.m_dtheta
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twist2d() {
        let tw = Twist2d::new(1.0, 2.0, 0.5);
        assert_eq!(tw.dx(), 1.0);
        assert_eq!(tw.dy(), 2.0);
        assert_eq!(tw.dtheta(), 0.5);
        assert_eq!(tw * 2.0, Twist2d::new(2.0, 4.0, 1.0));
    }
}
