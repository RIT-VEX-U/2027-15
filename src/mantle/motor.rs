//! Motor hardware abstraction, traits, motor groups, and mock interfaces.

use std::fmt::Debug;
use std::sync::{Arc, Mutex};
pub use crate::core::units::{BrakeMode, BrakeType, Direction, DirectionType, RotationUnits, VelocityUnits, VoltageUnits};

/// Motor internal gear cartridge setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GearSetting {
    Ratio36_1, // Red cartridge (100 RPM)
    #[default]
    Ratio18_1, // Green cartridge (200 RPM)
    Ratio6_1,  // Blue cartridge (600 RPM)
}

/// Abstract hardware motor interface.
pub trait Motor: Send + Sync + Debug {
    /// Spins the motor with specified direction and voltage.
    fn spin(&mut self, direction: DirectionType, voltage: f64, units: VoltageUnits);

    /// Spins the motor with velocity units.
    fn spin_velocity(&mut self, direction: DirectionType, velocity: f64, units: VelocityUnits);

    /// Stops the motor using default or specified braking mode.
    fn stop(&mut self, brake_type: BrakeType);

    /// Sets the default braking mode.
    fn set_brake(&mut self, brake_type: BrakeType);

    /// Returns recorded position in specified units.
    fn position(&self, units: RotationUnits) -> f64;

    /// Returns recorded velocity in specified units.
    fn velocity(&self, units: VelocityUnits) -> f64;

    /// Returns motor temperature in Celsius.
    fn temperature(&self) -> f64;

    /// Returns motor current in Amperes.
    fn current(&self) -> f64;

    /// Returns motor electrical voltage in Volts.
    fn voltage(&self) -> f64;

    /// Resets recorded position counter to zero.
    fn reset_position(&mut self);
}

/// A simulated mock motor for testing and simulation without hardware.
#[derive(Debug, Clone)]
pub struct MockMotor {
    pub port: i32,
    pub reversed: bool,
    pub gear_ratio: GearSetting,
    pub brake_mode: BrakeType,
    pub commanded_voltage: f64,
    pub position_rev: f64,
    pub velocity_rpm: f64,
}

impl MockMotor {
    pub fn new(port: i32, reversed: bool) -> Self {
        Self {
            port,
            reversed,
            gear_ratio: GearSetting::Ratio18_1,
            brake_mode: BrakeType::Brake,
            commanded_voltage: 0.0,
            position_rev: 0.0,
            velocity_rpm: 0.0,
        }
    }
}

impl Motor for MockMotor {
    fn spin(&mut self, direction: DirectionType, voltage: f64, units: VoltageUnits) {
        let v_norm = match units {
            VoltageUnits::Volt => voltage,
            VoltageUnits::Mv => voltage / 1000.0,
        };
        let sign = direction.sign();
        self.commanded_voltage = v_norm * sign;
        self.velocity_rpm = (self.commanded_voltage / 12.0) * 200.0;
    }

    fn spin_velocity(&mut self, direction: DirectionType, velocity: f64, units: VelocityUnits) {
        let rpm = match units {
            VelocityUnits::Rpm => velocity,
            VelocityUnits::Pct => (velocity / 100.0) * 200.0,
            VelocityUnits::Dps => velocity / 6.0,
        };
        let sign = direction.sign();
        self.velocity_rpm = rpm * sign;
        self.commanded_voltage = (self.velocity_rpm / 200.0) * 12.0;
    }

    fn stop(&mut self, brake_type: BrakeType) {
        self.brake_mode = brake_type;
        self.commanded_voltage = 0.0;
        self.velocity_rpm = 0.0;
    }

    fn set_brake(&mut self, brake_type: BrakeType) {
        self.brake_mode = brake_type;
    }

    fn position(&self, units: RotationUnits) -> f64 {
        match units {
            RotationUnits::Rev => self.position_rev,
            RotationUnits::Deg | RotationUnits::Raw => self.position_rev * 360.0,
        }
    }

    fn velocity(&self, units: VelocityUnits) -> f64 {
        match units {
            VelocityUnits::Rpm => self.velocity_rpm,
            VelocityUnits::Pct => (self.velocity_rpm / 200.0) * 100.0,
            VelocityUnits::Dps => self.velocity_rpm * 6.0,
        }
    }

    fn temperature(&self) -> f64 {
        25.0
    }

    fn current(&self) -> f64 {
        (self.commanded_voltage.abs() / 12.0) * 2.5
    }

    fn voltage(&self) -> f64 {
        self.commanded_voltage
    }

    fn reset_position(&mut self) {
        self.position_rev = 0.0;
    }
}

/// Synchronized group of multiple motors acting as a single composite actuator.
#[derive(Clone, Default)]
pub struct MotorGroup {
    motors: Vec<Arc<Mutex<Box<dyn Motor>>>>,
}

impl MotorGroup {
    /// Creates an empty motor group.
    pub fn new() -> Self {
        Self { motors: Vec::new() }
    }

    /// Creates a motor group from existing motor references.
    pub fn new_with_motors(motors: Vec<Arc<Mutex<Box<dyn Motor>>>>) -> Self {
        Self { motors }
    }

    /// Adds a motor to the group.
    pub fn add(&mut self, motor: Arc<Mutex<Box<dyn Motor>>>) {
        self.motors.push(motor);
    }

    /// Spins all motors in the group with specified voltage.
    pub fn spin(&self, dir: DirectionType, voltage: f64, units: VoltageUnits) {
        for m in &self.motors {
            if let Ok(mut lock) = m.lock() {
                lock.spin(dir, voltage, units);
            }
        }
    }

    /// Spins all motors with velocity units.
    pub fn spin_velocity(&self, dir: DirectionType, val: f64, units: VelocityUnits) {
        for m in &self.motors {
            if let Ok(mut lock) = m.lock() {
                lock.spin_velocity(dir, val, units);
            }
        }
    }

    /// Stops all motors with a specified brake mode.
    pub fn stop_with_brake(&self, brake: BrakeType) {
        for m in &self.motors {
            if let Ok(mut lock) = m.lock() {
                lock.stop(brake);
            }
        }
    }

    /// Stops all motors using their default brake mode.
    pub fn stop(&self) {
        self.stop_with_brake(BrakeType::Coast);
    }

    /// Sets the default brake mode on all motors.
    pub fn set_brake(&self, brake: BrakeType) {
        for m in &self.motors {
            if let Ok(mut lock) = m.lock() {
                lock.set_brake(brake);
            }
        }
    }

    /// Returns average position across all motors in specified units.
    pub fn position(&self, units: RotationUnits) -> f64 {
        if self.motors.is_empty() {
            return 0.0;
        }
        let mut total = 0.0;
        let mut count = 0;
        for m in &self.motors {
            if let Ok(lock) = m.lock() {
                total += lock.position(units);
                count += 1;
            }
        }
        if count > 0 {
            total / (count as f64)
        } else {
            0.0
        }
    }

    /// Returns average velocity in RPM.
    pub fn velocity_rpm(&self) -> f64 {
        self.velocity_units(VelocityUnits::Rpm)
    }

    /// Returns average velocity in specified units.
    pub fn velocity_units(&self, units: VelocityUnits) -> f64 {
        if self.motors.is_empty() {
            return 0.0;
        }
        let mut total = 0.0;
        let mut count = 0;
        for m in &self.motors {
            if let Ok(lock) = m.lock() {
                total += lock.velocity(units);
                count += 1;
            }
        }
        if count > 0 {
            total / (count as f64)
        } else {
            0.0
        }
    }

    /// Returns average velocity in RPM.
    pub fn velocity(&self) -> f64 {
        self.velocity_rpm()
    }

    /// Returns average temperature in Celsius.
    pub fn temperature(&self) -> f64 {
        if self.motors.is_empty() {
            return 25.0;
        }
        let mut total = 0.0;
        let mut count = 0;
        for m in &self.motors {
            if let Ok(lock) = m.lock() {
                total += lock.temperature();
                count += 1;
            }
        }
        if count > 0 {
            total / (count as f64)
        } else {
            25.0
        }
    }

    /// Resets position on all motors in the group.
    pub fn reset_position(&self) {
        for m in &self.motors {
            if let Ok(mut lock) = m.lock() {
                lock.reset_position();
            }
        }
    }

    /// Returns count of motors in this group.
    pub fn count(&self) -> usize {
        self.motors.len()
    }

    /// Returns underlying vector of motor references.
    pub fn motors(&self) -> Vec<Arc<Mutex<Box<dyn Motor>>>> {
        self.motors.clone()
    }
}
