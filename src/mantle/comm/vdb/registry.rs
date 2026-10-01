//! VDP channel registry controller and listener.
//!
//! Mirrors `core/device/vdb/registry-controller.hpp` and `registry-listener.hpp`.

use std::collections::HashMap;
use super::protocol::{make_header_byte, PacketFunction, PacketHeader, PacketReader, PacketType, PacketWriter};
use super::types::Part;

/// Manages registered telemetry and control channels.
pub struct RegistryController {
    channels: HashMap<u8, Box<dyn Part>>,
}

impl Default for RegistryController {
    fn default() -> Self {
        Self::new()
    }
}

impl RegistryController {
    /// Creates an empty registry controller.
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
        }
    }

    /// Registers a part to a specific channel ID (0..255).
    pub fn register_channel(&mut self, channel_id: u8, part: Box<dyn Part>) {
        self.channels.insert(channel_id, part);
    }

    /// Encodes a broadcast packet containing the schema of all registered channels.
    pub fn build_broadcast_packet(&self, channel_id: u8) -> Option<Vec<u8>> {
        let part = self.channels.get(&channel_id)?;
        let mut writer = PacketWriter::new();
        let header = PacketHeader {
            packet_type: PacketType::Broadcast,
            function: PacketFunction::Send,
        };
        writer.write_byte(make_header_byte(header));
        writer.write_byte(channel_id);
        part.write_schema(&mut writer);
        Some(writer.finalize_with_crc())
    }

    /// Encodes a data telemetry packet for the specified channel.
    pub fn build_data_packet(&mut self, channel_id: u8) -> Option<Vec<u8>> {
        let part = self.channels.get_mut(&channel_id)?;
        part.fetch();
        let mut writer = PacketWriter::new();
        let header = PacketHeader {
            packet_type: PacketType::Data,
            function: PacketFunction::Send,
        };
        writer.write_byte(make_header_byte(header));
        writer.write_byte(channel_id);
        part.write_message(&mut writer);
        Some(writer.finalize_with_crc())
    }

    /// Handles an incoming data packet, updating the corresponding channel's values.
    pub fn handle_incoming_data(&mut self, channel_id: u8, payload: Vec<u8>) -> bool {
        if let Some(part) = self.channels.get_mut(&channel_id) {
            let mut reader = PacketReader::new(payload);
            part.read_data_from_message(&mut reader);
            part.response();
            true
        } else {
            false
        }
    }
}
