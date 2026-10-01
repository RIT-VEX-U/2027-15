//! VEX Database Protocol (VDP) module.

pub mod builtins;
pub mod crc32;
pub mod protocol;
pub mod registry;
pub mod types;

pub use builtins::Builtins;
pub use crc32::Crc32;
pub use protocol::{
    decode_header_byte, make_header_byte, validate_packet, AbstractDevice, DataType, PacketFunction,
    PacketHeader, PacketReader, PacketType, PacketValidity, PacketWriter,
};
pub use registry::RegistryController;
pub use types::{DoublePart, FloatPart, Part, Record, StringPart};
