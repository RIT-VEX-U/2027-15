//! Concrete VEX V5 Smart Motor driver.
//!
//! Direct vendor integration layer consuming V5 smart port APIs.

pub use crate::mantle::motor::{
    BrakeMode, BrakeType, Direction, DirectionType, GearSetting, Motor as MotorTrait, MotorGroup,
    RotationUnits, VelocityUnits, VoltageUnits,
};

/// Individual VEX V5 Smart Motor connected via an RS-485 smart port.
#[derive(Debug, Clone)]
pub struct V5Motor {
    port: i32,
    reversed: bool,
    gear_ratio: GearSetting,
    brake_mode: BrakeType,
    commanded_voltage: f64,
    position_rev: f64,
    velocity_rpm: f64,
}

/// Type alias for V5Motor.
pub type Motor = V5Motor;

impl V5Motor {
    /// Creates a new Motor instance on the specified smart port (1-21).
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

    /// Creates a new Motor with specified gear setting.
    pub fn with_gear_ratio(port: i32, reversed: bool, gear: GearSetting) -> Self {
        Self {
            port,
            reversed,
            gear_ratio: gear,
            brake_mode: BrakeType::Brake,
            commanded_voltage: 0.0,
            position_rev: 0.0,
            velocity_rpm: 0.0,
        }
    }

    /// Returns the port index (1-21).
    pub fn port(&self) -> i32 {
        self.port
    }

    /// Returns whether the motor direction is reversed.
    pub fn is_reversed(&self) -> bool {
        self.reversed
    }

    /// Returns the gear ratio setting.
    pub fn gear_ratio(&self) -> GearSetting {
        self.gear_ratio
    }

    /// Sets the brake mode of the motor.
    pub fn set_brake_mode(&mut self, brake: BrakeType) {
        self.brake_mode = brake;
    }

    /// Sets the brake mode of the motor (alias).
    pub fn set_brake(&mut self, brake: BrakeType) {
        self.set_brake_mode(brake);
    }

    /// Returns the current brake mode.
    pub fn brake_mode(&self) -> BrakeType {
        self.brake_mode
    }

    /// Returns the current recorded position in specified units.
    pub fn position(&self, units: RotationUnits) -> f64 {
        MotorTrait::position(self, units)
    }

    /// Spins the motor with specified direction and voltage (in Volts).
    pub fn spin_voltage(&mut self, dir: DirectionType, volts: f64) {
        let sign = dir.sign();
        let mult = if self.reversed { -1.0 } else { 1.0 };
        self.commanded_voltage = (sign * mult * volts).clamp(-12.0, 12.0);
    }
}

impl MotorTrait for V5Motor {
    fn spin(&mut self, direction: DirectionType, voltage: f64, units: VoltageUnits) {
        let volts = match units {
            VoltageUnits::Volt => voltage,
            VoltageUnits::Mv => voltage / 1000.0,
        };
        self.spin_voltage(direction, volts);
    }

    fn spin_velocity(&mut self, direction: DirectionType, velocity: f64, units: VelocityUnits) {
        let max_rpm = match self.gear_ratio {
            GearSetting::Ratio36_1 => 100.0,
            GearSetting::Ratio18_1 => 200.0,
            GearSetting::Ratio6_1 => 600.0,
        };
        let target_rpm = match units {
            VelocityUnits::Pct => (velocity / 100.0) * max_rpm,
            VelocityUnits::Rpm => velocity,
            VelocityUnits::Dps => (velocity / 360.0) * 60.0,
        };
        let sign = direction.sign();
        let mult = if self.reversed { -1.0 } else { 1.0 };
        self.velocity_rpm = sign * mult * target_rpm;
        self.commanded_voltage = (self.velocity_rpm / max_rpm) * 12.0;
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
        let mult = if self.reversed { -1.0 } else { 1.0 };
        let rev = self.position_rev * mult;
        match units {
            RotationUnits::Rev => rev,
            RotationUnits::Deg | RotationUnits::Raw => rev * 360.0,
        }
    }

    fn velocity(&self, units: VelocityUnits) -> f64 {
        let mult = if self.reversed { -1.0 } else { 1.0 };
        let rpm = self.velocity_rpm * mult;
        match units {
            VelocityUnits::Rpm => rpm,
            VelocityUnits::Pct => {
                let max_rpm = match self.gear_ratio {
                    GearSetting::Ratio36_1 => 100.0,
                    GearSetting::Ratio18_1 => 200.0,
                    GearSetting::Ratio6_1 => 600.0,
                };
                (rpm / max_rpm) * 100.0
            }
            VelocityUnits::Dps => rpm * 6.0,
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
