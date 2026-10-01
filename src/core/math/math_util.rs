//! General mathematical functions and statistical utilities.

use crate::core::geometry::Translation2d;
use std::f64::consts::PI;

/// Constrains the input between a minimum and a maximum value.
pub fn clamp(val: f64, low: f64, high: f64) -> f64 {
    if val < low {
        low
    } else if val > high {
        high
    } else {
        val
    }
}

/// Linearly interpolates between values `a` and `b` according to `t` in `[0, 1]`.
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a * (1.0 - t) + b * t
}

/// Returns the sign of a number (`+1.0` or `-1.0`).
/// For compatibility with the original C++ codebase, 0.0 returns `+1.0`.
pub fn sign(x: f64) -> f64 {
    if x < 0.0 {
        -1.0
    } else {
        1.0
    }
}

/// Wraps an angle in degrees into `[0.0, 360.0)`.
pub fn wrap_angle_deg(input: f64) -> f64 {
    let mut angle = input % 360.0;
    if angle < 0.0 {
        angle += 360.0;
    }
    angle
}

/// Wraps an angle in radians into `[0.0, 2pi)`.
pub fn wrap_angle_rad(input: f64) -> f64 {
    let mut angle = input % (2.0 * PI);
    if angle < 0.0 {
        angle += 2.0 * PI;
    }
    angle
}

/// Calculates the arithmetic mean of a slice of floats.
pub fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let total: f64 = values.iter().sum();
    total / (values.len() as f64)
}

/// Calculates sample variance: `sum((x - mean)^2) / (N - 1)`.
pub fn variance(values: &[f64], mean_val: f64) -> f64 {
    if values.len() <= 1 {
        return 0.0;
    }
    let total: f64 = values.iter().map(|&x| (x - mean_val).powi(2)).sum();
    total / ((values.len() - 1) as f64)
}

/// Calculates sum of cross-products for covariance: `sum((x - mean_x) * (y - mean_y))`.
pub fn covariance(points: &[(f64, f64)], mean_x: f64, mean_y: f64) -> f64 {
    points
        .iter()
        .map(|&(x, y)| (x - mean_x) * (y - mean_y))
        .sum()
}

/// Calculates the slope and y-intercept `(m, b)` of the best fit line `y = m * x + b`.
pub fn calculate_linear_regression(points: &[(f64, f64)]) -> (f64, f64) {
    if points.is_empty() {
        return (0.0, 0.0);
    }
    let xs: Vec<f64> = points.iter().map(|&(x, _)| x).collect();
    let ys: Vec<f64> = points.iter().map(|&(_, y)| y).collect();

    let mean_x = mean(&xs);
    let mean_y = mean(&ys);

    let var_x = variance(&xs, mean_x);
    let cov = covariance(points, mean_x, mean_y);

    let slope = if var_x.abs() > 1e-12 {
        cov / var_x
    } else {
        0.0
    };
    let y_intercept = mean_y - slope * mean_x;

    (slope, y_intercept)
}

/// Estimates the total path length by summing consecutive Euclidean distances.
pub fn estimate_path_length(points: &[Translation2d]) -> f64 {
    if points.len() < 2 {
        return 0.0;
    }
    let mut dist = 0.0;
    let mut last_p = points[0];

    for &p in &points[1..] {
        if p == last_p {
            continue;
        }
        dist += p.distance(last_p);
        last_p = p;
    }
    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_util() {
        assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
        assert_eq!(clamp(-5.0, 0.0, 10.0), 0.0);
        assert_eq!(clamp(15.0, 0.0, 10.0), 10.0);

        assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
        assert_eq!(sign(-3.0), -1.0);
        assert_eq!(sign(3.0), 1.0);
        assert_eq!(sign(0.0), 1.0);

        assert_eq!(wrap_angle_deg(-90.0), 270.0);
        assert_eq!(wrap_angle_deg(450.0), 90.0);

        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let m = mean(&data);
        assert_eq!(m, 3.0);
        let v = variance(&data, m);
        assert_eq!(v, 2.5);
    }
}
