//! Sensor hardware abstractions, traits, custom encoders, and mock devices.

use std::fmt::Debug;
pub use crate::core::units::RotationUnits;

/// Abstract rotary encoder interface.
pub trait Encoder: Send + Sync + Debug {
    /// Reads position in specified units.
    fn position(&self, units: RotationUnits) -> f64;

    /// Reads velocity in RPM or DPS.
    fn velocity(&self) -> f64;

    /// Resets the encoder position to zero.
    fn reset_position(&mut self);
}

/// Abstract Inertial Measurement Unit (IMU) / Gyroscope interface.
pub trait InertialSensor: Send + Sync + Debug {
    /// Returns compass heading in degrees [0, 360).
    fn heading(&self) -> f64;

    /// Returns cumulative rotation angle in degrees (unbounded).
    fn rotation(&self) -> f64;

    /// Calibrates the gyroscope.
    fn calibrate(&mut self);

    /// Checks if calibration is currently active.
    fn is_calibrating(&self) -> bool;

    /// Resets the heading/rotation to zero.
    fn reset_rotation(&mut self);
}

/// Simulated mock encoder for testing.
#[derive(Debug, Clone, Default)]
pub struct MockEncoder {
    pub position_rev: f64,
    pub velocity_rpm: f64,
}

impl Encoder for MockEncoder {
    fn position(&self, units: RotationUnits) -> f64 {
        match units {
            RotationUnits::Rev => self.position_rev,
            RotationUnits::Deg | RotationUnits::Raw => self.position_rev * 360.0,
        }
    }

    fn velocity(&self) -> f64 {
        self.velocity_rpm
    }

    fn reset_position(&mut self) {
        self.position_rev = 0.0;
    }
}

/// Simulated mock IMU for testing.
#[derive(Debug, Clone, Default)]
pub struct MockInertialSensor {
    pub angle_deg: f64,
    pub calibrating: bool,
}

impl InertialSensor for MockInertialSensor {
    fn heading(&self) -> f64 {
        ((self.angle_deg % 360.0) + 360.0) % 360.0
    }

    fn rotation(&self) -> f64 {
        self.angle_deg
    }

    fn calibrate(&mut self) {
        self.calibrating = false;
    }

    fn is_calibrating(&self) -> bool {
        self.calibrating
    }

    fn reset_rotation(&mut self) {
        self.angle_deg = 0.0;
    }
}

/// High-resolution custom quadrature encoder wrapper for tracking wheels.
#[derive(Debug, Clone)]
pub struct CustomEncoder<E: Encoder> {
    encoder: E,
    tick_scalar: f64,
}

impl<E: Encoder> CustomEncoder<E> {
    /// Creates a custom encoder with a specified ticks-per-revolution count.
    pub fn new(encoder: E, ticks_per_rev: f64) -> Self {
        let tick_scalar = if ticks_per_rev.abs() < 1e-6 {
            1.0
        } else {
            ticks_per_rev / 360.0
        };
        Self {
            encoder,
            tick_scalar,
        }
    }

    /// Reads rotation scaled to specified units.
    pub fn rotation(&self, units: RotationUnits) -> f64 {
        self.encoder.position(units) / self.tick_scalar
    }

    /// Reads position scaled to specified units.
    pub fn position(&self, units: RotationUnits) -> f64 {
        self.encoder.position(units) / self.tick_scalar
    }

    /// Reads velocity scaled to standard units.
    pub fn velocity(&self) -> f64 {
        self.encoder.velocity() / self.tick_scalar
    }

    /// Resets the encoder position.
    pub fn reset_position(&mut self) {
        self.encoder.reset_position();
    }
}
