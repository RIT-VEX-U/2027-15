//! Advanced mathematical estimators, numerical solvers, and utilities.

pub mod kalman_filter;
pub mod math_util;
pub mod numerical_integration;
pub mod unscented_kalman_filter;

pub use kalman_filter::KalmanFilter;
pub use math_util::*;
pub use numerical_integration::*;
pub use unscented_kalman_filter::{ScaledSphericalSimplexSigmaPoints, UnscentedKalmanFilter};
