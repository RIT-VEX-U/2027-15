//! # Config: Robot Configuration and Hardware Map
//!
//! Outer layer defining physical parameters, smart port maps, control gains,
//! and wiring up the concrete robot instance.

pub mod gains;
pub mod ports;
pub mod robot;
pub mod specs;

pub use gains::*;
pub use ports::*;
pub use robot::*;
pub use specs::*;
