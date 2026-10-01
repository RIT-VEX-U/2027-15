//! VEX competition routines (autonomous and driver opcontrol).

pub mod autonomous;
pub mod opcontrol;

pub use autonomous::{autonomous, skills};
pub use opcontrol::opcontrol;
