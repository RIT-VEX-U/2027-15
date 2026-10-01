//! # Crust Layer: Direct VEX Integration Layer
//!
//! The outermost hardware tier consuming concrete VEX V5 SDK interfaces.
//!
//! Provides concrete drivers for:
//! - V5 Smart Motors (`V5Motor` / `Motor`)
//! - V5 Sensors (Inertial / IMU, Optical Encoders)
//! - V5 Handheld Controller (`V5Controller`)
//! - V5 Brain LCD (`BrainScreen`, `Brain`)
//! - V5 Competition Control lifecycle management
//! - RTOS timers and system delays

pub mod brain;
pub mod competition;
pub mod controller;
pub mod fun;
pub mod motor;
pub mod sensor;
pub mod timer;

pub use brain::*;
pub use competition::*;
pub use controller::*;
pub use fun::*;
pub use motor::*;
pub use sensor::*;
pub use timer::*;
