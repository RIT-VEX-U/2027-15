//! Consistent Overhead Byte Stuffing (COBS) serial communication framing and device.
//!
//! Mirrors `core/device/cobs_device.h` and `core/device/cobs_device.cpp`.

use std::sync::Mutex;

/// Decodes COBS-encoded data into an output buffer.
pub fn cobs_decode(input: &[u8], output: &mut [u8]) -> Result<usize, &'static str> {
    let mut in_idx = 0;
    let mut out_idx = 0;

    while in_idx < input.len() {
        let code = input[in_idx] as usize;
        in_idx += 1;
        if code == 0 {
            return Err("Zero byte found inside COBS packet");
        }

        for _ in 1..code {
            if in_idx >= input.len() {
                return Err("Unexpected end of COBS buffer");
            }
            if out_idx >= output.len() {
                return Err("Output buffer overflow");
            }
            output[out_idx] = input[in_idx];
            out_idx += 1;
            in_idx += 1;
        }

        if code < 0xFF && in_idx < input.len() {
            if out_idx >= output.len() {
                return Err("Output buffer overflow");
            }
            output[out_idx] = 0;
            out_idx += 1;
        }
    }

    Ok(out_idx)
}

/// Encodes raw bytes into a COBS-encoded vector (without trailing delimiter).
pub fn cobs_encode(input: &[u8]) -> Vec<u8> {
    if input.is_empty() {
        return Vec::new();
    }
    let mut output = Vec::with_capacity(input.len() + input.len() / 254 + 2);
    let mut code_idx = 0;
    output.push(0); // placeholder for code
    let mut code: u8 = 1;

    for &b in input {
        if b != 0 {
            output.push(b);
            code += 1;
            if code == 0xFF {
                output[code_idx] = code;
                code_idx = output.len();
                output.push(0);
                code = 1;
            }
        } else {
            output[code_idx] = code;
            code_idx = output.len();
            output.push(0);
            code = 1;
        }
    }

    output[code_idx] = code;
    output
}

/// Serial device that communicates over UART using COBS packets delimited by zero bytes.
pub struct CobssSerialDevice {
    port: i32,
    baud: i32,
    serial_mutex: Mutex<()>,
    last_decoded_packet: Mutex<Vec<u8>>,
    incoming_wire_packet: Mutex<Vec<u8>>,
}

impl CobssSerialDevice {
    /// Creates a new COBS serial device on the specified smart port and baud rate.
    pub fn new(port: i32, baud: i32) -> Self {
        Self {
            port,
            baud,
            serial_mutex: Mutex::new(()),
            last_decoded_packet: Mutex::new(Vec::new()),
            incoming_wire_packet: Mutex::new(Vec::new()),
        }
    }

    /// Returns the port index.
    pub fn port(&self) -> i32 {
        self.port
    }

    /// Returns the baud rate.
    pub fn baud(&self) -> i32 {
        self.baud
    }

    /// Encodes data using COBS with an optional start delimiter.
    pub fn encode_packet(data: &[u8], add_start_delimiter: bool) -> Vec<u8> {
        let mut wire = cobs_encode(data);
        if add_start_delimiter {
            wire.insert(0, 0);
        }
        wire.push(0); // trailing delimiter
        wire
    }

    /// Decodes a COBS wire packet (excluding delimiters).
    pub fn decode_packet(wire: &[u8]) -> Result<Vec<u8>, &'static str> {
        let mut output = vec![0u8; wire.len()];
        let len = cobs_decode(wire, &mut output)?;
        output.truncate(len);
        Ok(output)
    }

    /// Processes an incoming raw byte stream, returning true if a complete packet was framed and decoded.
    pub fn handle_incoming_byte(&self, byte: u8) -> bool {
        let mut wire = self.incoming_wire_packet.lock().unwrap();
        if byte == 0 {
            if wire.is_empty() {
                false
            } else {
                if let Ok(decoded) = Self::decode_packet(&wire) {
                    *self.last_decoded_packet.lock().unwrap() = decoded;
                }
                wire.clear();
                true
            }
        } else {
            wire.push(byte);
            false
        }
    }

    /// Returns the most recently decoded packet.
    pub fn get_last_decoded_packet(&self) -> Vec<u8> {
        self.last_decoded_packet.lock().unwrap().clone()
    }
}
