//! Signal filters and noise reduction algorithms.

pub mod moving_average;

pub use moving_average::{ExponentialMovingAverage, Filter, MovingAverage};
