//! Concrete VEX V5 Sensor drivers: Inertial Sensor (IMU) and Optical/Quadrature Encoders.

use std::sync::{Arc, Mutex};
pub use crate::core::units::RotationUnits;
pub use crate::mantle::sensor::{Encoder as EncoderTrait, InertialSensor as InertialTrait};

/// VEX V5 Inertial Sensor (IMU) providing 3-axis gyro and accelerometer data.
#[derive(Debug, Clone, Default)]
pub struct Inertial {
    port: i32,
    heading_deg: Arc<Mutex<f64>>,
    rotation_deg: Arc<Mutex<f64>>,
    is_calibrating: Arc<Mutex<bool>>,
}

/// Type alias for Inertial.
pub type InertialSensor = Inertial;

impl Inertial {
    /// Creates a new Inertial sensor instance on the specified port.
    pub fn new(port: i32) -> Self {
        Self {
            port,
            heading_deg: Arc::new(Mutex::new(0.0)),
            rotation_deg: Arc::new(Mutex::new(0.0)),
            is_calibrating: Arc::new(Mutex::new(false)),
        }
    }

    /// Returns the port index.
    pub fn port(&self) -> i32 {
        self.port
    }

    /// Calibrates the sensor.
    pub fn calibrate(&self) {
        *self.is_calibrating.lock().unwrap() = true;
        *self.is_calibrating.lock().unwrap() = false;
    }

    /// Returns whether the sensor is currently calibrating.
    pub fn is_calibrating(&self) -> bool {
        *self.is_calibrating.lock().unwrap()
    }

    /// Returns the heading in degrees, wrapped to [0.0, 360.0).
    pub fn heading(&self) -> f64 {
        let h = *self.heading_deg.lock().unwrap() % 360.0;
        if h < 0.0 {
            h + 360.0
        } else {
            h
        }
    }

    /// Returns cumulative rotation in degrees (unbounded).
    pub fn rotation(&self) -> f64 {
        *self.rotation_deg.lock().unwrap()
    }

    /// Sets the current rotation value.
    pub fn set_rotation(&self, deg: f64) {
        *self.rotation_deg.lock().unwrap() = deg;
        *self.heading_deg.lock().unwrap() = deg;
    }

    /// Resets rotation and heading to zero.
    pub fn reset_rotation(&self) {
        self.set_rotation(0.0);
    }
}

impl InertialTrait for Inertial {
    fn heading(&self) -> f64 {
        Inertial::heading(self)
    }

    fn rotation(&self) -> f64 {
        Inertial::rotation(self)
    }

    fn calibrate(&mut self) {
        Inertial::calibrate(self);
    }

    fn is_calibrating(&self) -> bool {
        Inertial::is_calibrating(self)
    }

    fn reset_rotation(&mut self) {
        Inertial::reset_rotation(self);
    }
}

/// VEX Optical / Quadrature Shaft Encoder.
#[derive(Debug, Clone)]
pub struct Encoder {
    port: i32,
    position_deg: Arc<Mutex<f64>>,
    velocity_dps: Arc<Mutex<f64>>,
}

impl Encoder {
    /// Creates a new encoder on a specific port.
    pub fn new(port: i32) -> Self {
        Self {
            port,
            position_deg: Arc::new(Mutex::new(0.0)),
            velocity_dps: Arc::new(Mutex::new(0.0)),
        }
    }

    /// Returns the port index.
    pub fn port(&self) -> i32 {
        self.port
    }

    /// Sets the recorded rotation.
    pub fn set_rotation(&self, val: f64, units: RotationUnits) {
        let deg = match units {
            RotationUnits::Deg => val,
            RotationUnits::Rev => val * 360.0,
            RotationUnits::Raw => val,
        };
        *self.position_deg.lock().unwrap() = deg;
    }

    /// Sets the recorded position.
    pub fn set_position(&self, val: f64, units: RotationUnits) {
        self.set_rotation(val, units);
    }

    /// Returns the current rotation.
    pub fn rotation(&self, units: RotationUnits) -> f64 {
        let deg = *self.position_deg.lock().unwrap();
        match units {
            RotationUnits::Deg => deg,
            RotationUnits::Rev => deg / 360.0,
            RotationUnits::Raw => deg,
        }
    }

    /// Returns the current position.
    pub fn position(&self, units: RotationUnits) -> f64 {
        self.rotation(units)
    }

    /// Returns the angular velocity in degrees per second.
    pub fn velocity_dps(&self) -> f64 {
        *self.velocity_dps.lock().unwrap()
    }

    /// Alias for angular velocity.
    pub fn velocity(&self) -> f64 {
        self.velocity_dps()
    }

    /// Sets the velocity for simulation purposes.
    pub fn set_velocity_dps(&self, dps: f64) {
        *self.velocity_dps.lock().unwrap() = dps;
    }

    /// Resets recorded position to zero.
    pub fn reset_position(&self) {
        self.set_rotation(0.0, RotationUnits::Deg);
    }
}

impl EncoderTrait for Encoder {
    fn position(&self, units: RotationUnits) -> f64 {
        Encoder::position(self, units)
    }

    fn velocity(&self) -> f64 {
        Encoder::velocity(self)
    }

    fn reset_position(&mut self) {
        Encoder::reset_position(self);
    }
}
