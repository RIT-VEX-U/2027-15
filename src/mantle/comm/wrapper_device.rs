//! VDB serial wrapper device bridging VDP packets to COBS serial.
//!
//! Mirrors `core/device/wrapper_device.hpp` and `wrapper_device.cpp`.

use super::cobs::CobssSerialDevice;
use super::vdb::protocol::AbstractDevice;
use std::collections::VecDeque;
use std::sync::Mutex;

/// Bridges VDP abstract packet sending to a COBS serial interface.
pub struct VdbDevice {
    serial: CobssSerialDevice,
    outbound_queue: Mutex<VecDeque<Vec<u8>>>,
    receive_callback: Option<Box<dyn Fn(&[u8]) + Send + Sync>>,
}

impl VdbDevice {
    /// Creates a new VDB serial device.
    pub fn new(port: i32, baud: i32) -> Self {
        Self {
            serial: CobssSerialDevice::new(port, baud),
            outbound_queue: Mutex::new(VecDeque::new()),
            receive_callback: None,
        }
    }

    /// Queues a packet for transmission.
    pub fn queue_packet(&self, packet: Vec<u8>) {
        let mut queue = self.outbound_queue.lock().unwrap();
        if queue.len() < 50 {
            queue.push_back(packet);
        }
    }
}

impl AbstractDevice for VdbDevice {
    fn send_packet(&mut self, packet: &[u8]) -> bool {
        self.queue_packet(packet.to_vec());
        true
    }

    fn register_receive_callback(&mut self, callback: Box<dyn Fn(&[u8]) + Send + Sync>) {
        self.receive_callback = Some(callback);
    }
}
