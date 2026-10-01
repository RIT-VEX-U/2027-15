//! # Robot 2027-15
//!
//! Autonomous robotics control system structured using a 3-tier planetary architecture.
//!
//! ## Architecture Overview
//! - [`core`]: Platform-agnostic inner engine: pure math, geometry, kinematics, pathing, controls, filters, state machines, and units.
//!   Zero VEX / hardware dependencies.
//! - [`mantle`]: Hardware Abstraction Layer (HAL): generic motor/sensor traits, mock devices, display/graphing interfaces,
//!   COBS framing, VDB protocol, and high-level subsystems (TankDrive, Flywheel, Lift, Odometry).
//! - [`crust`]: Direct VEX integration layer: concrete drivers consuming V5 smart ports, brain display, controller, and competition lifecycle.
//! - [`config`]: Robot configuration and hardware map: physical dimensions, track widths, PID tuning gains, and port assignments.
//! - [`competition`]: Autonomous routines and driver control loops.

#![allow(dead_code)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::if_same_then_else)]

pub mod competition;
pub mod config;
pub mod core;
pub mod crust;
pub mod mantle;

pub use config::RobotHardware;
