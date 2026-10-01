//! 2D Rotation representation with trigonometric caching and angle wrapping.

use nalgebra::Matrix2;
use std::f64::consts::PI;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub const TWO_PI: f64 = 2.0 * PI;

/// Converts degrees to radians.
pub fn deg2rad(deg: f64) -> f64 {
    deg * (PI / 180.0)
}

/// Converts radians to degrees.
pub fn rad2deg(rad: f64) -> f64 {
    rad * (180.0 / PI)
}

/// Wraps a radian angle into `[-pi, pi)`.
pub fn wrap_radians_180(angle: f64) -> f64 {
    let mut x = (angle + PI) % TWO_PI;
    if x < 0.0 {
        x += TWO_PI;
    }
    x - PI
}

/// Wraps a degree angle into `[-180.0, 180.0)`.
pub fn wrap_degrees_180(angle: f64) -> f64 {
    let mut x = (angle + 180.0) % 360.0;
    if x < 0.0 {
        x += 360.0;
    }
    x - 180.0
}

/// Wraps a revolution angle into `[-0.5, 0.5)`.
pub fn wrap_revolutions_180(angle: f64) -> f64 {
    let mut x = (angle + 0.5) % 1.0;
    if x < 0.0 {
        x += 1.0;
    }
    x - 0.5
}

/// Wraps a radian angle into `[0.0, 2pi)`.
pub fn wrap_radians_360(angle: f64) -> f64 {
    let mut x = angle % TWO_PI;
    if x < 0.0 {
        x += TWO_PI;
    }
    x
}

/// Wraps a degree angle into `[0.0, 360.0)`.
pub fn wrap_degrees_360(angle: f64) -> f64 {
    let mut x = angle % 360.0;
    if x < 0.0 {
        x += 360.0;
    }
    x
}

/// Wraps a revolution angle into `[0.0, 1.0)`.
pub fn wrap_revolutions_360(angle: f64) -> f64 {
    let mut x = angle % 1.0;
    if x < 0.0 {
        x += 1.0;
    }
    x
}

/// Class representing a rotation in 2D space.
/// Stores theta in radians, along with cached cosine and sine values.
#[derive(Debug, Clone, Copy)]
pub struct Rotation2d {
    m_radians: f64,
    m_cos: f64,
    m_sin: f64,
}

impl Default for Rotation2d {
    fn default() -> Self {
        Self::new(0.0)
    }
}

impl Rotation2d {
    /// Zero rotation constant (0 radians, cos=1, sin=0).
    pub const ZERO: Self = Self {
        m_radians: 0.0,
        m_cos: 1.0,
        m_sin: 0.0,
    };

    /// Constructs a rotation with given value in radians.
    pub fn new(radians: f64) -> Self {
        Self {
            m_radians: radians,
            m_cos: radians.cos(),
            m_sin: radians.sin(),
        }
    }

    /// Constructs a rotation given x and y components: `theta = atan2(y, x)`.
    pub fn from_xy(x: f64, y: f64) -> Self {
        let magnitude = x.hypot(y);
        if magnitude > 1e-9 {
            let cos = x / magnitude;
            let sin = y / magnitude;
            Self {
                m_radians: sin.atan2(cos),
                m_cos: cos,
                m_sin: sin,
            }
        } else {
            Self {
                m_radians: 0.0,
                m_cos: 1.0,
                m_sin: 0.0,
            }
        }
    }

    /// Constructs a rotation from radians.
    pub fn from_radians(radians: f64) -> Self {
        Self::new(radians)
    }

    /// Constructs a rotation from degrees.
    pub fn from_degrees(degrees: f64) -> Self {
        Self::new(deg2rad(degrees))
    }

    /// Constructs a rotation from revolutions.
    pub fn from_revolutions(revolutions: f64) -> Self {
        Self::new(revolutions * TWO_PI)
    }

    /// Returns the radian angle value.
    pub fn radians(&self) -> f64 {
        self.m_radians
    }

    /// Returns the degree angle value.
    pub fn degrees(&self) -> f64 {
        rad2deg(self.m_radians)
    }

    /// Returns the revolution angle value.
    pub fn revolutions(&self) -> f64 {
        self.m_radians / TWO_PI
    }

    /// Returns cosine.
    pub fn f_cos(&self) -> f64 {
        self.m_cos
    }

    /// Returns sine.
    pub fn f_sin(&self) -> f64 {
        self.m_sin
    }

    /// Returns tangent.
    pub fn f_tan(&self) -> f64 {
        self.m_sin / self.m_cos
    }

    /// Returns the 2x2 rotation matrix `[[cos, -sin], [sin, cos]]`.
    pub fn rotation_matrix(&self) -> Matrix2<f64> {
        Matrix2::new(self.m_cos, -self.m_sin, self.m_sin, self.m_cos)
    }

    /// Returns wrapped radians in `[-pi, pi)`.
    pub fn wrapped_radians_180(&self) -> f64 {
        wrap_radians_180(self.m_radians)
    }

    /// Returns wrapped degrees in `[-180.0, 180.0)`.
    pub fn wrapped_degrees_180(&self) -> f64 {
        wrap_radians_180(self.m_radians) * (180.0 / PI)
    }

    /// Returns wrapped revolutions in `[-0.5, 0.5)`.
    pub fn wrapped_revolutions_180(&self) -> f64 {
        wrap_radians_180(self.m_radians) / TWO_PI
    }

    /// Returns wrapped radians in `[0.0, 2pi)`.
    pub fn wrapped_radians_360(&self) -> f64 {
        wrap_radians_360(self.m_radians)
    }

    /// Returns wrapped degrees in `[0.0, 360.0)`.
    pub fn wrapped_degrees_360(&self) -> f64 {
        wrap_radians_360(self.m_radians) * (180.0 / PI)
    }

    /// Returns wrapped revolutions in `[0.0, 1.0)`.
    pub fn wrapped_revolutions_360(&self) -> f64 {
        wrap_radians_360(self.m_radians) / TWO_PI
    }
}

impl Add for Rotation2d {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::from_xy(
            self.m_cos * other.m_cos - self.m_sin * other.m_sin,
            self.m_cos * other.m_sin + self.m_sin * other.m_cos,
        )
    }
}

impl Sub for Rotation2d {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

impl Neg for Rotation2d {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.m_radians)
    }
}

impl Mul<f64> for Rotation2d {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self::new(self.m_radians * scalar)
    }
}

impl Div<f64> for Rotation2d {
    type Output = Self;
    fn div(self, scalar: f64) -> Self {
        Self::new(self.m_radians / scalar)
    }
}

impl PartialEq for Rotation2d {
    fn eq(&self, other: &Self) -> bool {
        (self.m_radians - other.m_radians).abs() < 1e-9
    }
}

impl fmt::Display for Rotation2d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Rotation2d[rad: {}, deg: {}]",
            self.radians(),
            self.degrees()
        )
    }
}

/// Calculates the unwrapped arithmetic mean of a slice of rotations.
pub fn unwrapped_mean(list: &[Rotation2d]) -> Rotation2d {
    if list.is_empty() {
        return Rotation2d::default();
    }
    let sum: f64 = list.iter().map(|r| r.radians()).sum();
    Rotation2d::new(sum / (list.len() as f64))
}

/// Calculates the angular mean of a slice of rotations taking angle wrapping into account.
pub fn wrapped_mean(list: &[Rotation2d]) -> Rotation2d {
    if list.is_empty() {
        return Rotation2d::default();
    }
    let sum_sin: f64 = list.iter().map(|r| r.f_sin()).sum();
    let sum_cos: f64 = list.iter().map(|r| r.f_cos()).sum();
    Rotation2d::from_xy(sum_cos, sum_sin)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation2d() {
        let r0 = Rotation2d::from_degrees(0.0);
        let r90 = Rotation2d::from_degrees(90.0);
        let r180 = Rotation2d::from_degrees(180.0);

        assert!((r90.f_cos() - 0.0).abs() < 1e-9);
        assert!((r90.f_sin() - 1.0).abs() < 1e-9);

        let sum = r0 + r90;
        assert_eq!(sum, r90);

        let diff = r180 - r90;
        assert_eq!(diff, r90);

        assert_eq!(wrap_degrees_180(190.0), -170.0);
        assert_eq!(wrap_degrees_360(-10.0), 350.0);
    }
}
