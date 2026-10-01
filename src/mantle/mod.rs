//! # Mantle Layer: Hardware Abstraction Layer (HAL)
//!
//! The intermediate tier bridging the abstract, platform-agnostic `core` engine
//! to the concrete hardware implementations in `crust`.
//!
//! Exposes mockable traits, hardware-independent abstractions, communication buses,
//! composite motor groups, multi-sensor odometry fusion, and subsystem controllers.

pub mod commands;
pub mod comm;
pub mod controller;
pub mod display;
pub mod initializer;
pub mod motor;
pub mod sensor;
pub mod subsystems;

pub use commands::*;
pub use comm::*;
pub use controller::*;
pub use display::*;
pub use initializer::*;
pub use motor::*;
pub use sensor::*;
pub use subsystems::*;
