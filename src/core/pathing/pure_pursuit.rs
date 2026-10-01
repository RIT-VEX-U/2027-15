//! Pure Pursuit path tracking, waypoint interpolation, spline generation, and geometric lookahead algorithms.

use crate::core::geometry::{Pose2d, Rotation2d, Translation2d};

/// A sequence of 2D translation waypoints paired with a lookahead radius.
#[derive(Debug, Clone)]
pub struct Path {
    points: Vec<Translation2d>,
    radius: f64,
    valid: bool,
}

impl Path {
    /// Constructs a Path and validates that consecutive segments do not intersect within `radius`.
    pub fn new(points: Vec<Translation2d>, radius: f64) -> Self {
        let mut valid = true;

        if points.len() >= 3 {
            for i in 0..(points.len() - 1) {
                for j in (i + 2)..(points.len() - 1) {
                    let mut seg_i_dist = points[i].distance(points[i + 1]);
                    if seg_i_dist == 0.0 {
                        seg_i_dist = 0.1;
                    }
                    let mut seg_j_dist = points[j].distance(points[j + 1]);
                    if seg_j_dist == 0.0 {
                        seg_j_dist = 0.1;
                    }

                    let step_i = radius / seg_i_dist;
                    let step_j = radius / seg_j_dist;

                    let mut t1 = 0.0;
                    while t1 <= 1.0 {
                        let p1 = Translation2d::new(
                            points[i].x() + t1 * (points[i + 1].x() - points[i].x()),
                            points[i].y() + t1 * (points[i + 1].y() - points[i].y()),
                        );

                        let mut t2 = 0.0;
                        while t2 <= 1.0 {
                            let p2 = Translation2d::new(
                                points[j].x() + t2 * (points[j + 1].x() - points[j].x()),
                                points[j].y() + t2 * (points[j + 1].y() - points[j].y()),
                            );

                            if p1.distance(p2) < radius {
                                valid = false;
                                return Self {
                                    points,
                                    radius,
                                    valid,
                                };
                            }
                            t2 += step_j.max(0.01);
                        }
                        t1 += step_i.max(0.01);
                    }
                }
            }
        }

        Self {
            points,
            radius,
            valid,
        }
    }

    /// Returns the points in the path.
    pub fn get_points(&self) -> &[Translation2d] {
        &self.points
    }

    /// Returns the lookahead radius.
    pub fn get_radius(&self) -> f64 {
        self.radius
    }

    /// Returns whether the path is valid.
    pub fn is_valid(&self) -> bool {
        self.valid
    }
}

/// Point on a Hermite spline with heading direction and tangent magnitude.
#[derive(Debug, Clone, Copy)]
pub struct HermitePoint {
    pub x: f64,
    pub y: f64,
    pub dir: f64,
    pub mag: f64,
}

impl HermitePoint {
    /// Returns the translation coordinate of the point.
    pub fn get_point(&self) -> Translation2d {
        Translation2d::new(self.x, self.y)
    }

    /// Returns the tangent vector of the point.
    pub fn get_tangent(&self) -> Translation2d {
        Translation2d::from_polar(self.mag, Rotation2d::new(self.dir))
    }
}

/// Computes intersections between a line segment `(point1, point2)` and a circle centered at `center` with radius `r`.
pub fn line_circle_intersections(
    center: Translation2d,
    r: f64,
    point1: Translation2d,
    point2: Translation2d,
) -> Vec<Translation2d> {
    let mut intersections = Vec::new();

    let p1 = point1 - center;
    let p2 = point2 - center;

    let (x1, y1, x2, y2) = if (p1.x() - p2.x()).abs() < 1e-12 {
        let x1 = p1.x();
        let r2_minus_x2 = r * r - x1 * x1;
        if r2_minus_x2 < 0.0 {
            return intersections;
        }
        let y_root = r2_minus_x2.sqrt();
        (x1, y_root, x1, -y_root)
    } else {
        let m = (p1.y() - p2.y()) / (p1.x() - p2.x());
        let b = p1.y() - (m * p1.x());

        let disc = r * r + (m * m * r * r) - (b * b);
        if disc < 0.0 {
            return intersections;
        }
        let disc_sqrt = disc.sqrt();
        let denom = 1.0 + m * m;

        let x1 = ((-m * b) + disc_sqrt) / denom;
        let y1 = m * x1 + b;
        let x2 = ((-m * b) - disc_sqrt) / denom;
        let y2 = m * x2 + b;

        (x1, y1, x2, y2)
    };

    let min_x = p1.x().min(p2.x()) - 1e-9;
    let max_x = p1.x().max(p2.x()) + 1e-9;
    let min_y = p1.y().min(p2.y()) - 1e-9;
    let max_y = p1.y().max(p2.y()) + 1e-9;

    if x1 >= min_x && x1 <= max_x && y1 >= min_y && y1 <= max_y {
        intersections.push(Translation2d::new(x1 + center.x(), y1 + center.y()));
    }

    if x2 >= min_x && x2 <= max_x && y2 >= min_y && y2 <= max_y {
        let pt2 = Translation2d::new(x2 + center.x(), y2 + center.y());
        if intersections.is_empty() || intersections[0].distance(pt2) > 1e-9 {
            intersections.push(pt2);
        }
    }

    intersections
}

/// Selects the optimal pure pursuit lookahead point along the path from the current robot pose.
pub fn get_lookahead(path: &[Translation2d], robot_loc: Pose2d, radius: f64) -> Translation2d {
    if path.is_empty() {
        return robot_loc.translation();
    }
    let target = *path.last().unwrap();
    if target.distance(robot_loc.translation()) <= radius {
        return target;
    }

    let mut best_target = target;

    for i in 0..(path.len() - 1) {
        let start = path[i];
        let end = path[i + 1];

        let intersections = line_circle_intersections(robot_loc.translation(), radius, start, end);
        for inter in intersections {
            if inter.distance(end) < best_target.distance(end) {
                best_target = inter;
            }
        }
    }

    best_target
}

/// Injects intermediate points into a path with specified spacing.
pub fn inject_path(path: &[Translation2d], spacing: f64) -> Vec<Translation2d> {
    if path.len() < 2 {
        return path.to_vec();
    }
    let mut new_path = Vec::new();

    for i in 0..(path.len() - 1) {
        let start = path[i];
        let end = path[i + 1];
        let diff = end - start;
        let num_points = (diff.norm() / spacing).ceil() as usize;

        let unit_step = diff.normalize() * spacing;

        for j in 0..num_points {
            new_path.push(start + unit_step * (j as f64));
        }
    }
    new_path.push(*path.last().unwrap());
    new_path
}

/// Smooths a path iteratively while anchoring the start and end points.
pub fn smooth_path(
    path: &[Translation2d],
    weight_data: f64,
    weight_smooth: f64,
    tolerance: f64,
) -> Vec<Translation2d> {
    if path.len() < 3 {
        return path.to_vec();
    }

    let mut new_path = path.to_vec();
    let mut change = tolerance;

    while change >= tolerance {
        change = 0.0;
        for i in 1..(path.len() - 1) {
            let x_i = path[i];
            let y_i = new_path[i];
            let y_prev = new_path[i - 1];
            let y_next = new_path[i + 1];

            let y_i_saved = y_i;

            let updated = Translation2d::new(
                y_i.x()
                    + weight_data * (x_i.x() - y_i.x())
                    + weight_smooth * (y_next.x() + y_prev.x() - 2.0 * y_i.x()),
                y_i.y()
                    + weight_data * (x_i.y() - y_i.y())
                    + weight_smooth * (y_next.y() + y_prev.y() - 2.0 * y_i.y()),
            );

            new_path[i] = updated;
            change += updated.distance(y_i_saved);
        }
    }

    new_path
}

/// Interpolates waypoints using cubic Hermite splines.
pub fn smooth_path_hermite(path: &[HermitePoint], steps: usize) -> Vec<Translation2d> {
    if path.is_empty() {
        return Vec::new();
    }
    let mut new_path = Vec::new();
    let steps_f = steps as f64;

    for i in 0..(path.len() - 1) {
        let p1 = path[i].get_point();
        let p2 = path[i + 1].get_point();
        let t1 = path[i].get_tangent();
        let t2 = path[i + 1].get_tangent();

        for t in 0..steps {
            let s = (t as f64) / steps_f;
            let s2 = s * s;
            let s3 = s2 * s;

            let h1 = 2.0 * s3 - 3.0 * s2 + 1.0;
            let h2 = -2.0 * s3 + 3.0 * s2;
            let h3 = s3 - 2.0 * s2 + s;
            let h4 = s3 - s2;

            let pv = p1 * h1 + p2 * h2 + t1 * h3 + t2 * h4;
            new_path.push(pv);
        }
    }

    new_path.push(path.last().unwrap().get_point());
    new_path
}

/// Estimates remaining distance to the path terminus.
pub fn estimate_remaining_dist(path: &[Translation2d], robot_pose: Pose2d, radius: f64) -> f64 {
    if path.is_empty() {
        return 0.0;
    }
    let lookahead = get_lookahead(path, robot_pose, radius);
    if lookahead == *path.last().unwrap() {
        return robot_pose.translation().distance(lookahead);
    }

    let mut dist = 0.0;
    for i in (1..path.len()).rev() {
        let pts = line_circle_intersections(robot_pose.translation(), radius, path[i - 1], path[i]);
        if !pts.is_empty() {
            dist += robot_pose.translation().distance(path[i]);
            return dist;
        }
        dist += path[i - 1].distance(path[i]);
    }

    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_circle_intersections() {
        let center = Translation2d::new_origin();
        let r = 5.0;
        let p1 = Translation2d::new(-10.0, 0.0);
        let p2 = Translation2d::new(10.0, 0.0);

        let inters = line_circle_intersections(center, r, p1, p2);
        assert_eq!(inters.len(), 2);
    }
}
