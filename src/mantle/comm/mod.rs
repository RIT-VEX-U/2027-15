//! Communication buses, framing, and diagnostic protocols.

pub mod cobs;
pub mod vdb;
pub mod wrapper_device;

pub use cobs::{cobs_decode, cobs_encode, CobssSerialDevice};
pub use vdb::*;
pub use wrapper_device::VdbDevice;
