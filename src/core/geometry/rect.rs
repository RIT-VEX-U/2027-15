//! 2D Axis-aligned bounding rectangle.
//!
//! Mirrors `core/utils/geometry.h`.

use super::translation2d::Translation2d;

/// 2D Axis-Aligned Bounding Box (AABB) Rectangle defined by position `(x, y)` and dimensions `(width, height)`.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    /// Constructs a rectangle from x, y, width, and height.
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Constructs a rectangle from minimum corner and size dimensions.
    pub fn from_min_and_size(min: Translation2d, size: Translation2d) -> Self {
        Self {
            x: min.x(),
            y: min.y(),
            width: size.x(),
            height: size.y(),
        }
    }

    /// Returns the dimensions `(width, height)` as a Translation2d.
    pub fn dimensions(&self) -> Translation2d {
        Translation2d::new(self.width, self.height)
    }

    /// Returns the center point of the rectangle.
    pub fn center(&self) -> Translation2d {
        Translation2d::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    /// Returns width along the X axis.
    pub fn width(&self) -> f64 {
        self.width
    }

    /// Returns height along the Y axis.
    pub fn height(&self) -> f64 {
        self.height
    }

    /// Tests whether coordinate `(px, py)` lies inside the rectangle.
    pub fn contains(&self, px: f64, py: f64) -> bool {
        px >= self.x && px <= (self.x + self.width) && py >= self.y && py <= (self.y + self.height)
    }

    /// Tests whether a point lies strictly inside the rectangle.
    pub fn contains_point(&self, p: Translation2d) -> bool {
        self.contains(p.x(), p.y())
    }
}
