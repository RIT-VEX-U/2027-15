//! 2D Translation / vector representation.

use super::rotation2d::Rotation2d;
use nalgebra::Vector2;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// 2D Translation representing a point or displacement in continuous 2D space.
#[derive(Debug, Clone, Copy, Default)]
pub struct Translation2d {
    m_x: f64,
    m_y: f64,
}

impl Translation2d {
    /// Constructs a translation at the origin (0.0, 0.0).
    pub const fn new_origin() -> Self {
        Self { m_x: 0.0, m_y: 0.0 }
    }

    /// Constructs a translation with given x and y components.
    pub const fn new(x: f64, y: f64) -> Self {
        Self { m_x: x, m_y: y }
    }

    /// Constructs a translation from polar coordinates (magnitude r and direction angle).
    pub fn from_polar(r: f64, theta: Rotation2d) -> Self {
        Self {
            m_x: r * theta.f_cos(),
            m_y: r * theta.f_sin(),
        }
    }

    /// Constructs a translation from an `nalgebra::Vector2<f64>`.
    pub fn from_vector(v: Vector2<f64>) -> Self {
        Self { m_x: v.x, m_y: v.y }
    }

    /// Returns the x component.
    pub const fn x(&self) -> f64 {
        self.m_x
    }

    /// Sets the x component.
    pub fn set_x(&mut self, x: f64) {
        self.m_x = x;
    }

    /// Returns the y component.
    pub const fn y(&self) -> f64 {
        self.m_y
    }

    /// Sets the y component.
    pub fn set_y(&mut self, y: f64) {
        self.m_y = y;
    }

    /// Returns the angle of the translation vector relative to the +X axis.
    pub fn theta(&self) -> Rotation2d {
        Rotation2d::from_xy(self.m_x, self.m_y)
    }

    /// Returns the translation as an `nalgebra::Vector2<f64>`.
    pub fn as_vector(&self) -> Vector2<f64> {
        Vector2::new(self.m_x, self.m_y)
    }

    /// Returns the Euclidean norm (magnitude / distance from origin).
    pub fn norm(&self) -> f64 {
        self.m_x.hypot(self.m_y)
    }

    /// Returns a unit-vector translation in the same direction, or origin if zero.
    pub fn normalize(&self) -> Self {
        let n = self.norm();
        if n > 1e-9 {
            Self {
                m_x: self.m_x / n,
                m_y: self.m_y / n,
            }
        } else {
            Self::new_origin()
        }
    }

    /// Returns Euclidean distance to another translation.
    pub fn distance(&self, other: Translation2d) -> f64 {
        (self.m_x - other.m_x).hypot(self.m_y - other.m_y)
    }

    /// Rotates the translation around the origin by the given rotation.
    pub fn rotate_by(&self, rotation: Rotation2d) -> Self {
        Self {
            m_x: self.m_x * rotation.f_cos() - self.m_y * rotation.f_sin(),
            m_y: self.m_x * rotation.f_sin() + self.m_y * rotation.f_cos(),
        }
    }

    /// Rotates the translation around another given point by the specified rotation.
    pub fn rotate_around(&self, center: Translation2d, rotation: Rotation2d) -> Self {
        (*self - center).rotate_by(rotation) + center
    }

    /// Returns scalar dot product.
    pub fn dot(&self, other: Translation2d) -> f64 {
        self.m_x * other.m_x + self.m_y * other.m_y
    }
}

impl Add for Translation2d {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            m_x: self.m_x + other.m_x,
            m_y: self.m_y + other.m_y,
        }
    }
}

impl Sub for Translation2d {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            m_x: self.m_x - other.m_x,
            m_y: self.m_y - other.m_y,
        }
    }
}

impl Neg for Translation2d {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            m_x: -self.m_x,
            m_y: -self.m_y,
        }
    }
}

impl Mul<f64> for Translation2d {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self {
            m_x: self.m_x * scalar,
            m_y: self.m_y * scalar,
        }
    }
}

impl Div<f64> for Translation2d {
    type Output = Self;
    fn div(self, scalar: f64) -> Self {
        Self {
            m_x: self.m_x / scalar,
            m_y: self.m_y / scalar,
        }
    }
}

impl PartialEq for Translation2d {
    fn eq(&self, other: &Self) -> bool {
        (self.m_x - other.m_x).abs() < 1e-9 && (self.m_y - other.m_y).abs() < 1e-9
    }
}

impl fmt::Display for Translation2d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Translation2d[x: {}, y: {}]", self.m_x, self.m_y)
    }
}

/// Calculates the mean of a slice of translations.
pub fn mean(list: &[Translation2d]) -> Translation2d {
    if list.is_empty() {
        return Translation2d::new_origin();
    }
    let sum_x: f64 = list.iter().map(|p| p.x()).sum();
    let sum_y: f64 = list.iter().map(|p| p.y()).sum();
    let len = list.len() as f64;
    Translation2d::new(sum_x / len, sum_y / len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation2d() {
        let t1 = Translation2d::new(3.0, 4.0);
        let t2 = Translation2d::new(1.0, 2.0);

        assert_eq!(t1.norm(), 5.0);
        assert_eq!(t1.distance(t2), (2.0f64).hypot(2.0));
        assert_eq!(t1 + t2, Translation2d::new(4.0, 6.0));
        assert_eq!(t1 - t2, Translation2d::new(2.0, 2.0));
        assert_eq!(t1 * 2.0, Translation2d::new(6.0, 8.0));
        assert_eq!(t1 / 2.0, Translation2d::new(1.5, 2.0));

        let rotated = Translation2d::new(1.0, 0.0).rotate_by(Rotation2d::from_degrees(90.0));
        assert_eq!(rotated, Translation2d::new(0.0, 1.0));
    }
}
