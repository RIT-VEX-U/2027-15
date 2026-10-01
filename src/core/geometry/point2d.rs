//! Class representing a 2D integer lattice point.

use nalgebra::{Vector2, Vector2 as Vec2d};
use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

/// 2D integer lattice point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Point2d {
    xcoord: i32,
    ycoord: i32,
}

impl Point2d {
    /// Creates a lattice point at the origin (0, 0).
    pub const fn new_origin() -> Self {
        Self {
            xcoord: 0,
            ycoord: 0,
        }
    }

    /// Creates a lattice point at (x, y).
    pub const fn new(x: i32, y: i32) -> Self {
        Self {
            xcoord: x,
            ycoord: y,
        }
    }

    /// Creates a lattice point from a 2D integer vector.
    pub fn from_vector(vector: Vector2<i32>) -> Self {
        Self {
            xcoord: vector.x,
            ycoord: vector.y,
        }
    }

    /// Returns the X coordinate.
    pub const fn x(&self) -> i32 {
        self.xcoord
    }

    /// Sets the X coordinate.
    pub fn set_x(&mut self, x: i32) {
        self.xcoord = x;
    }

    /// Returns the Y coordinate.
    pub const fn y(&self) -> i32 {
        self.ycoord
    }

    /// Sets the Y coordinate.
    pub fn set_y(&mut self, y: i32) {
        self.ycoord = y;
    }

    /// Returns the point as an `nalgebra::Vector2<i32>`.
    pub fn as_vector(&self) -> Vector2<i32> {
        Vector2::new(self.xcoord, self.ycoord)
    }

    /// Returns a vector in the canonical basis corresponding to the point in the basis of X and Y.
    pub fn as_vector_basis(&self, x_basis: Vec2d<f64>, y_basis: Vec2d<f64>) -> Vec2d<f64> {
        Vec2d::new(
            (self.xcoord as f64) * x_basis.x + (self.ycoord as f64) * y_basis.x,
            (self.xcoord as f64) * x_basis.y + (self.ycoord as f64) * y_basis.y,
        )
    }

    /// Returns the Manhattan (L1) distance between two points.
    pub fn manhattan_distance(&self, other: &Point2d) -> i32 {
        (self.xcoord - other.xcoord).abs() + (self.ycoord - other.ycoord).abs()
    }

    /// Returns the Manhattan norm from the origin.
    pub fn manhattan_norm(&self) -> i32 {
        self.xcoord.abs() + self.ycoord.abs()
    }

    /// Returns Euclidean distance as `f64`.
    pub fn distance(&self, other: &Point2d) -> f64 {
        ((self.xcoord - other.xcoord) as f64).hypot((self.ycoord - other.ycoord) as f64)
    }

    /// Returns Euclidean norm from the origin.
    pub fn norm(&self) -> f64 {
        (self.xcoord as f64).hypot(self.ycoord as f64)
    }

    /// Returns the dot product of two points.
    pub fn dot(&self, other: &Point2d) -> i32 {
        (self.xcoord * other.xcoord) + (self.ycoord * other.ycoord)
    }
}

impl Add for Point2d {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            xcoord: self.xcoord + other.xcoord,
            ycoord: self.ycoord + other.ycoord,
        }
    }
}

impl Sub for Point2d {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            xcoord: self.xcoord - other.xcoord,
            ycoord: self.ycoord - other.ycoord,
        }
    }
}

impl Neg for Point2d {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            xcoord: -self.xcoord,
            ycoord: -self.ycoord,
        }
    }
}

impl Mul<i32> for Point2d {
    type Output = Self;
    fn mul(self, scalar: i32) -> Self {
        Self {
            xcoord: self.xcoord * scalar,
            ycoord: self.ycoord * scalar,
        }
    }
}

impl fmt::Display for Point2d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point2d[x: {}, y: {}]", self.xcoord, self.ycoord)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point2d() {
        let p1 = Point2d::new(3, 4);
        let p2 = Point2d::new(1, 2);

        assert_eq!(p1.x(), 3);
        assert_eq!(p1.y(), 4);
        assert_eq!(p1.norm(), 5.0);
        assert_eq!(p1.manhattan_norm(), 7);
        assert_eq!(p1.manhattan_distance(&p2), 4);
        assert_eq!(p1 + p2, Point2d::new(4, 6));
        assert_eq!(p1 - p2, Point2d::new(2, 2));
        assert_eq!(-p1, Point2d::new(-3, -4));
        assert_eq!(p1 * 2, Point2d::new(6, 8));
        assert_eq!(p1.dot(&p2), 11);
    }
}
