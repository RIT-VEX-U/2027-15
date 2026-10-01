//! # Core Layer: Platform-Agnostic Inner Robotics Engine
//!
//! The innermost tier of the 3-tier planetary architecture.
//!
//! **STRICT INVARIANT**: Zero dependencies on VEX, PROS, or vendor SDKs.
//! Pure mathematical, geometric, kinematic, control, and state machine algorithms.

pub mod controls;
pub mod filter;
pub mod formatting;
pub mod geometry;
pub mod interpolating_map;
pub mod logger;
pub mod math;
pub mod pathing;
pub mod state_machine;
pub mod time;
pub mod units;

pub use controls::*;
pub use filter::*;
pub use formatting::*;
pub use geometry::*;
pub use interpolating_map::InterpolatingMap;
pub use logger::*;
pub use math::*;
pub use pathing::*;
pub use state_machine::*;
pub use time::Timer;
pub use units::*;
