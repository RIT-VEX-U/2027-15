//! Odometry positioning, tracking wheel kinematics, and sensor fusion.

pub mod odometry_3wheel;
pub mod odometry_base;
pub mod odometry_nwheel;
pub mod odometry_serial;
pub mod odometry_tank;

pub use odometry_3wheel::{Odometry3Wheel, Odometry3WheelConfig};
pub use odometry_base::{smallest_angle, Odometry, OdometryState};
pub use odometry_nwheel::{OdometryNWheel, TrackingWheelConfig};
pub use odometry_serial::OdometrySerial;
pub use odometry_tank::{OdometryTank, TankWheelSensors};
