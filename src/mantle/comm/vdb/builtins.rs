//! Standard built-in VDP telemetry and diagnostic records.
//!
//! Mirrors `core/device/vdb/builtins.hpp` and `builtins.cpp`.

use super::types::{FloatPart, Part, Record};
use crate::core::controls::pid::Pid;
use crate::core::units::{RotationUnits, VelocityUnits};
use crate::mantle::motor::Motor;
use crate::mantle::subsystems::odometry::Odometry;
use std::sync::{Arc, Mutex};

/// Helper for constructing standard VDP telemetry records.
pub struct Builtins;

impl Builtins {
    /// Builds a motor telemetry record with position, velocity, temperature, voltage, and current.
    pub fn create_motor_record(name: impl Into<String>, motor: Arc<Mutex<Box<dyn Motor>>>) -> Record {
        let n = name.into();
        let m1 = Arc::clone(&motor);
        let m2 = Arc::clone(&motor);
        let m3 = Arc::clone(&motor);
        let m4 = Arc::clone(&motor);
        let m5 = Arc::clone(&motor);

        let fields: Vec<Box<dyn Part>> = vec![
            Box::new(FloatPart::new("position", Some(Box::new(move || m1.lock().unwrap().position(RotationUnits::Rev) as f32)))),
            Box::new(FloatPart::new("velocity", Some(Box::new(move || m2.lock().unwrap().velocity(VelocityUnits::Rpm) as f32)))),
            Box::new(FloatPart::new("temperature", Some(Box::new(move || m3.lock().unwrap().temperature() as f32)))),
            Box::new(FloatPart::new("voltage", Some(Box::new(move || m4.lock().unwrap().voltage() as f32)))),
            Box::new(FloatPart::new("current", Some(Box::new(move || m5.lock().unwrap().current() as f32)))),
        ];

        Record::with_fields(n, fields)
    }

    /// Builds an odometry state record with X, Y, and rotation angle.
    pub fn create_odometry_record(name: impl Into<String>, odom: Arc<Mutex<dyn Odometry>>) -> Record {
        let n = name.into();
        let o1 = Arc::clone(&odom);
        let o2 = Arc::clone(&odom);
        let o3 = Arc::clone(&odom);

        let fields: Vec<Box<dyn Part>> = vec![
            Box::new(FloatPart::new("x", Some(Box::new(move || o1.lock().unwrap().get_position().x() as f32)))),
            Box::new(FloatPart::new("y", Some(Box::new(move || o2.lock().unwrap().get_position().y() as f32)))),
            Box::new(FloatPart::new("rot", Some(Box::new(move || o3.lock().unwrap().get_position().rotation().degrees() as f32)))),
        ];

        Record::with_fields(n, fields)
    }

    /// Builds a PID gains and telemetry record.
    pub fn create_pid_record(name: impl Into<String>, pid: Arc<Mutex<Pid>>) -> Record {
        let n = name.into();
        let p1 = Arc::clone(&pid);
        let p2 = Arc::clone(&pid);
        let p3 = Arc::clone(&pid);
        let p4 = Arc::clone(&pid);

        let fields: Vec<Box<dyn Part>> = vec![
            Box::new(FloatPart::new("kp", Some(Box::new(move || p1.lock().unwrap().config.p as f32)))),
            Box::new(FloatPart::new("ki", Some(Box::new(move || p2.lock().unwrap().config.i as f32)))),
            Box::new(FloatPart::new("kd", Some(Box::new(move || p3.lock().unwrap().config.d as f32)))),
            Box::new(FloatPart::new("output", Some(Box::new(move || p4.lock().unwrap().get() as f32)))),
        ];

        Record::with_fields(n, fields)
    }
}
