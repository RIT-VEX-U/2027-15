//! Serial coprocessor odometry implementation with COBS framing.
//!
//! Mirrors `core/subsystems/odometry/odometry_serial.h` and `core/subsystems/odometry/odometry_serial.cpp`.

use super::odometry_base::{Odometry, OdometryState};
use crate::core::geometry::{Pose2d, Rotation2d};

/// Decodes a COBS-encoded slice into raw bytes. Returns output byte count.
pub fn cobs_decode(input: &[u8], output: &mut [u8]) -> Result<usize, &'static str> {
    if input.is_empty() {
        return Ok(0);
    }
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
        }
        if b == 0 || code == 0xFF {
            output[code_idx] = code;
            code = 1;
            code_idx = output.len();
            output.push(0);
        }
    }
    output[code_idx] = code;
    output
}

/// Serial coprocessor odometry receiver.
pub struct OdometrySerial {
    port: i32,
    calc_vel_acc_on_brain: bool,
    pose: Pose2d,
    pose_offset: Pose2d,
    state: OdometryState,
    reader_fn: Option<Box<dyn FnMut(&mut [u8]) -> usize + Send + Sync>>,
}

impl OdometrySerial {
    /// Creates a new serial odometry receiver on the given port.
    pub fn new(
        port: i32,
        calc_vel_acc_on_brain: bool,
        initial_pose: Pose2d,
        sensor_offset: Pose2d,
        reader_fn: Option<Box<dyn FnMut(&mut [u8]) -> usize + Send + Sync>>,
    ) -> Self {
        let _ = (initial_pose, sensor_offset);
        Self {
            port,
            calc_vel_acc_on_brain,
            pose: Pose2d::new(0.0, 0.0, Rotation2d::new(0.0)),
            pose_offset: Pose2d::new(0.0, 0.0, Rotation2d::new(0.0)),
            state: OdometryState::new(),
            reader_fn,
        }
    }

    /// Parses a 28-byte payload representing seven 32-bit floats.
    pub fn parse_telemetry_payload(&mut self, payload: &[u8]) -> Result<Pose2d, &'static str> {
        if payload.len() < 28 {
            return Err("Payload too small for 7 floats");
        }

        let read_f32 = |offset: usize| {
            let bytes = [
                payload[offset],
                payload[offset + 1],
                payload[offset + 2],
                payload[offset + 3],
            ];
            f32::from_le_bytes(bytes) as f64
        };

        let x = read_f32(0);
        let y = read_f32(4);
        let rot_deg = read_f32(8);
        let speed = read_f32(12);
        let accel = read_f32(16);
        let ang_speed_deg = read_f32(20);
        let ang_accel_deg = read_f32(24);

        let updated_pose = Pose2d::new(x, y, Rotation2d::from_degrees(rot_deg));
        self.pose = updated_pose;
        self.state.speed = speed;
        self.state.accel = accel;
        self.state.ang_speed_deg = ang_speed_deg;
        self.state.ang_accel_deg = ang_accel_deg;

        Ok(self.get_position())
    }
}

impl Odometry for OdometrySerial {
    fn get_position(&self) -> Pose2d {
        self.pose.relative_to(&self.pose_offset)
    }

    fn set_position(&mut self, new_pos: Pose2d) {
        self.pose_offset = new_pos;
    }

    fn update(&mut self) -> Pose2d {
        if let Some(ref mut reader) = self.reader_fn {
            let mut raw_buf = [0u8; 64];
            let read_bytes = reader(&mut raw_buf);
            if read_bytes >= 28 {
                let mut decoded = [0u8; 64];
                if let Ok(len) = cobs_decode(&raw_buf[..read_bytes], &mut decoded) {
                    let _ = self.parse_telemetry_payload(&decoded[..len]);
                }
            }
        }
        self.get_position()
    }

    fn get_speed(&self) -> f64 {
        self.state.speed
    }

    fn get_accel(&self) -> f64 {
        self.state.accel
    }

    fn get_angular_speed_deg(&self) -> f64 {
        self.state.ang_speed_deg
    }

    fn get_angular_accel_deg(&self) -> f64 {
        self.state.ang_accel_deg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cobs_roundtrip() {
        let original = vec![1, 2, 0, 4, 5, 0, 7];
        let encoded = cobs_encode(&original);
        assert!(!encoded.contains(&0));

        let mut decoded = vec![0u8; original.len()];
        let len = cobs_decode(&encoded, &mut decoded).unwrap();
        assert_eq!(len, original.len());
        assert_eq!(&decoded[..len], &original[..]);
    }
}
