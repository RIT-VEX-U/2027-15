//! VEX Database Protocol (VDP) packet framing, header decoding, and binary reading/writing.
//!
//! Mirrors `core/device/vdb/protocol.hpp` and `protocol.cpp`.

use super::crc32::Crc32;

/// Maximum number of communication channels supported.
pub const MAX_CHANNELS: usize = 256;

/// Primary packet class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketType {
    Broadcast = 0x00,
    Data = 0x80,
}

/// Operation requested or conveyed by the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketFunction {
    Send = 0x00,
    Acknowledge = 0x20,
    Response = 0x40,
    Request = 0x60,
}

/// Packet header containing packet type and function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PacketHeader {
    pub packet_type: PacketType,
    pub function: PacketFunction,
}

/// Packet validity check result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketValidity {
    Ok,
    BadChecksum,
    TooSmall,
}

/// Encodes a header struct into a single header byte.
pub fn make_header_byte(header: PacketHeader) -> u8 {
    (header.packet_type as u8) | (header.function as u8)
}

/// Decodes a header byte into its corresponding packet type and function.
pub fn decode_header_byte(byte: u8) -> PacketHeader {
    let packet_type = if (byte & 0x80) != 0 {
        PacketType::Data
    } else {
        PacketType::Broadcast
    };

    let function = match byte & 0x60 {
        0x20 => PacketFunction::Acknowledge,
        0x40 => PacketFunction::Response,
        0x60 => PacketFunction::Request,
        _ => PacketFunction::Send,
    };

    PacketHeader {
        packet_type,
        function,
    }
}

/// Validates packet size and CRC-32 checksum integrity.
pub fn validate_packet(packet: &[u8]) -> PacketValidity {
    if packet.len() < 5 {
        return PacketValidity::TooSmall;
    }

    let payload_len = packet.len() - 4;
    let expected_crc = u32::from_le_bytes([
        packet[payload_len],
        packet[payload_len + 1],
        packet[payload_len + 2],
        packet[payload_len + 3],
    ]);

    let calculated_crc = Crc32::calculate(&packet[..payload_len]);
    if expected_crc == calculated_crc {
        PacketValidity::Ok
    } else {
        PacketValidity::BadChecksum
    }
}

/// VDP data types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Record = 0,
    String = 1,
    Double = 3,
    Float = 4,
    Uint8 = 5,
    Uint16 = 6,
    Uint32 = 7,
    Uint64 = 8,
    Int8 = 9,
    Int16 = 10,
    Int32 = 11,
    Int64 = 12,
}

impl DataType {
    /// Converts a byte to a DataType if valid.
    pub fn from_u8(b: u8) -> Option<Self> {
        match b {
            0 => Some(DataType::Record),
            1 => Some(DataType::String),
            3 => Some(DataType::Double),
            4 => Some(DataType::Float),
            5 => Some(DataType::Uint8),
            6 => Some(DataType::Uint16),
            7 => Some(DataType::Uint32),
            8 => Some(DataType::Uint64),
            9 => Some(DataType::Int8),
            10 => Some(DataType::Int16),
            11 => Some(DataType::Int32),
            12 => Some(DataType::Int64),
            _ => None,
        }
    }
}

/// Sequential binary packet reader.
pub struct PacketReader {
    packet: Vec<u8>,
    read_head: usize,
}

impl PacketReader {
    /// Creates a reader for the given packet starting at index 0.
    pub fn new(packet: Vec<u8>) -> Self {
        Self {
            packet,
            read_head: 0,
        }
    }

    /// Reads a single byte.
    pub fn get_byte(&mut self) -> u8 {
        if self.read_head < self.packet.len() {
            let b = self.packet[self.read_head];
            self.read_head += 1;
            b
        } else {
            0
        }
    }

    /// Reads a DataType.
    pub fn get_type(&mut self) -> Option<DataType> {
        let b = self.get_byte();
        DataType::from_u8(b)
    }

    /// Reads a null-terminated UTF-8 string.
    pub fn get_string(&mut self) -> String {
        let mut bytes = Vec::new();
        while self.read_head < self.packet.len() {
            let b = self.get_byte();
            if b == 0 {
                break;
            }
            bytes.push(b);
        }
        String::from_utf8_lossy(&bytes).to_string()
    }

    /// Reads a little-endian float.
    pub fn get_f32(&mut self) -> f32 {
        if self.read_head + 4 <= self.packet.len() {
            let bytes = [
                self.packet[self.read_head],
                self.packet[self.read_head + 1],
                self.packet[self.read_head + 2],
                self.packet[self.read_head + 3],
            ];
            self.read_head += 4;
            f32::from_le_bytes(bytes)
        } else {
            0.0
        }
    }

    /// Reads a little-endian double.
    pub fn get_f64(&mut self) -> f64 {
        if self.read_head + 8 <= self.packet.len() {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&self.packet[self.read_head..self.read_head + 8]);
            self.read_head += 8;
            f64::from_le_bytes(bytes)
        } else {
            0.0
        }
    }

    /// Reads a little-endian 32-bit integer.
    pub fn get_i32(&mut self) -> i32 {
        if self.read_head + 4 <= self.packet.len() {
            let bytes = [
                self.packet[self.read_head],
                self.packet[self.read_head + 1],
                self.packet[self.read_head + 2],
                self.packet[self.read_head + 3],
            ];
            self.read_head += 4;
            i32::from_le_bytes(bytes)
        } else {
            0
        }
    }

    /// Reads a little-endian 64-bit integer.
    pub fn get_i64(&mut self) -> i64 {
        if self.read_head + 8 <= self.packet.len() {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&self.packet[self.read_head..self.read_head + 8]);
            self.read_head += 8;
            i64::from_le_bytes(bytes)
        } else {
            0
        }
    }
}

/// Binary packet writer for building VDP messages.
pub struct PacketWriter {
    buffer: Vec<u8>,
}

impl Default for PacketWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl PacketWriter {
    /// Creates an empty packet writer.
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    /// Clears the writer buffer.
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Returns the length of the written packet.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Writes a single byte.
    pub fn write_byte(&mut self, b: u8) {
        self.buffer.push(b);
    }

    /// Writes a DataType tag.
    pub fn write_type(&mut self, t: DataType) {
        self.write_byte(t as u8);
    }

    /// Writes a null-terminated UTF-8 string.
    pub fn write_string(&mut self, s: &str) {
        self.buffer.extend_from_slice(s.as_bytes());
        self.buffer.push(0);
    }

    /// Writes a 32-bit float in little-endian format.
    pub fn write_f32(&mut self, val: f32) {
        self.buffer.extend_from_slice(&val.to_le_bytes());
    }

    /// Writes a 64-bit float in little-endian format.
    pub fn write_f64(&mut self, val: f64) {
        self.buffer.extend_from_slice(&val.to_le_bytes());
    }

    /// Writes a 32-bit integer in little-endian format.
    pub fn write_i32(&mut self, val: i32) {
        self.buffer.extend_from_slice(&val.to_le_bytes());
    }

    /// Finalizes the packet by computing and appending the CRC-32 checksum.
    pub fn finalize_with_crc(mut self) -> Vec<u8> {
        let crc = Crc32::calculate(&self.buffer);
        self.buffer.extend_from_slice(&crc.to_le_bytes());
        self.buffer
    }

    /// Returns a slice of the written bytes so far.
    pub fn as_slice(&self) -> &[u8] {
        &self.buffer
    }
}

/// Abstract hardware or virtual device interface for transmitting and receiving VDP packets.
pub trait AbstractDevice: Send + Sync {
    /// Sends a packet over the transport medium.
    fn send_packet(&mut self, packet: &[u8]) -> bool;
    /// Registers a callback invoked upon packet reception.
    fn register_receive_callback(&mut self, callback: Box<dyn Fn(&[u8]) + Send + Sync>);
}
