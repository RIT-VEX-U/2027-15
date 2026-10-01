//! Physical units and dimensional analysis system for robotics.
//!
//! Provides type-safe quantities for Length, Angle, Velocity, Angular Velocity,
//! Voltage, Current, Time, Temperature, and PID/Feedforward control gains.

use std::f64::consts::PI;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

macro_rules! define_unit {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
        pub struct $name(pub f64);

        impl $name {
            /// Constructs a new quantity with the base SI value.
            pub const fn from_base(val: f64) -> Self {
                Self(val)
            }

            /// Returns the raw value in base SI units.
            pub const fn base(&self) -> f64 {
                self.0
            }

            /// Returns the raw value.
            pub const fn internal(&self) -> f64 {
                self.0
            }
        }

        impl Add for $name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Self(self.0 + rhs.0)
            }
        }

        impl Sub for $name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                Self(self.0 - rhs.0)
            }
        }

        impl Neg for $name {
            type Output = Self;
            fn neg(self) -> Self {
                Self(-self.0)
            }
        }

        impl Mul<f64> for $name {
            type Output = Self;
            fn mul(self, rhs: f64) -> Self {
                Self(self.0 * rhs)
            }
        }

        impl Mul<$name> for f64 {
            type Output = $name;
            fn mul(self, rhs: $name) -> $name {
                $name(self * rhs.0)
            }
        }

        impl Div<f64> for $name {
            type Output = Self;
            fn div(self, rhs: f64) -> Self {
                Self(self.0 / rhs)
            }
        }

        impl Div<$name> for $name {
            type Output = f64;
            fn div(self, rhs: $name) -> f64 {
                self.0 / rhs.0
            }
        }
    };
}

define_unit!(Length, "Length measurement (Base: meters)");
define_unit!(Time, "Time measurement (Base: seconds)");
define_unit!(Angle, "Angle measurement (Base: radians)");
define_unit!(Velocity, "Linear velocity (Base: meters/second)");
define_unit!(Acceleration, "Linear acceleration (Base: meters/second^2)");
define_unit!(AngularVelocity, "Angular velocity (Base: radians/second)");
define_unit!(AngularAcceleration, "Angular acceleration (Base: radians/second^2)");
define_unit!(Voltage, "Electrical potential (Base: Volts)");
define_unit!(Current, "Electric current (Base: Amperes)");
define_unit!(Torque, "Torque (Base: Newton-meters)");

define_unit!(AngularProportionalGain, "Angular Proportional Gain (Base: Volts / radian)");
define_unit!(LinearProportionalGain, "Linear Proportional Gain (Base: Volts / meter)");
define_unit!(LinearVelocityFeedforward, "Linear Velocity Feedforward (Base: Volts / (meter/second))");
define_unit!(AngularVelocityFeedforward, "Angular Velocity Feedforward (Base: Volts / (radian/second))");

impl Length {
    pub fn meters(m: f64) -> Self { Self(m) }
    pub fn inches(inch: f64) -> Self { Self(inch * 0.0254) }
    pub fn feet(ft: f64) -> Self { Self(ft * 0.3048) }
    pub fn cm(cm: f64) -> Self { Self(cm * 0.01) }
    pub fn mm(mm: f64) -> Self { Self(mm * 0.001) }
    pub fn tiles(tile: f64) -> Self { Self::inches(tile * 23.75) }

    pub fn to_meters(&self) -> f64 { self.0 }
    pub fn to_inches(&self) -> f64 { self.0 / 0.0254 }
    pub fn to_feet(&self) -> f64 { self.0 / 0.3048 }
    pub fn to_cm(&self) -> f64 { self.0 * 100.0 }
    pub fn to_mm(&self) -> f64 { self.0 * 1000.0 }
    pub fn to_tiles(&self) -> f64 { self.to_inches() / 23.75 }
}

impl Time {
    pub fn seconds(s: f64) -> Self { Self(s) }
    pub fn milliseconds(ms: f64) -> Self { Self(ms / 1000.0) }
    pub fn minutes(mins: f64) -> Self { Self(mins * 60.0) }
    pub fn hours(hr: f64) -> Self { Self(hr * 3600.0) }

    pub fn to_seconds(&self) -> f64 { self.0 }
    pub fn to_milliseconds(&self) -> f64 { self.0 * 1000.0 }
    pub fn to_minutes(&self) -> f64 { self.0 / 60.0 }
}

impl Angle {
    pub fn radians(rad: f64) -> Self { Self(rad) }
    pub fn degrees(deg: f64) -> Self { Self(deg * (PI / 180.0)) }
    pub fn revolutions(rev: f64) -> Self { Self(rev * 2.0 * PI) }

    pub fn to_radians(&self) -> f64 { self.0 }
    pub fn to_degrees(&self) -> f64 { self.0 * (180.0 / PI) }
    pub fn to_revolutions(&self) -> f64 { self.0 / (2.0 * PI) }
}

impl Velocity {
    pub fn mps(mps: f64) -> Self { Self(mps) }
    pub fn inps(inps: f64) -> Self { Self(inps * 0.0254) }
    pub fn mph(mph: f64) -> Self { Self(mph * 0.44704) }

    pub fn to_mps(&self) -> f64 { self.0 }
    pub fn to_inps(&self) -> f64 { self.0 / 0.0254 }
    pub fn to_mph(&self) -> f64 { self.0 / 0.44704 }
}

impl AngularVelocity {
    pub fn radps(radps: f64) -> Self { Self(radps) }
    pub fn dps(dps: f64) -> Self { Self(dps * (PI / 180.0)) }
    pub fn rpm(rpm: f64) -> Self { Self((rpm / 60.0) * 2.0 * PI) }

    pub fn to_radps(&self) -> f64 { self.0 }
    pub fn to_dps(&self) -> f64 { self.0 * (180.0 / PI) }
    pub fn to_rpm(&self) -> f64 { (self.0 / (2.0 * PI)) * 60.0 }
}

impl Voltage {
    pub fn volts(v: f64) -> Self { Self(v) }
    pub fn millivolts(mv: f64) -> Self { Self(mv / 1000.0) }

    pub fn to_volts(&self) -> f64 { self.0 }
    pub fn to_millivolts(&self) -> f64 { self.0 * 1000.0 }
}

impl Current {
    pub fn amps(a: f64) -> Self { Self(a) }
    pub fn milliamps(ma: f64) -> Self { Self(ma / 1000.0) }

    pub fn to_amps(&self) -> f64 { self.0 }
    pub fn to_milliamps(&self) -> f64 { self.0 * 1000.0 }
}

// Cross-unit dimensional operators
impl Div<Time> for Length {
    type Output = Velocity;
    fn div(self, rhs: Time) -> Velocity {
        Velocity(self.0 / rhs.0)
    }
}

impl Div<Time> for Angle {
    type Output = AngularVelocity;
    fn div(self, rhs: Time) -> AngularVelocity {
        AngularVelocity(self.0 / rhs.0)
    }
}

impl Div<Time> for Velocity {
    type Output = Acceleration;
    fn div(self, rhs: Time) -> Acceleration {
        Acceleration(self.0 / rhs.0)
    }
}

impl Div<Time> for AngularVelocity {
    type Output = AngularAcceleration;
    fn div(self, rhs: Time) -> AngularAcceleration {
        AngularAcceleration(self.0 / rhs.0)
    }
}

impl Mul<Time> for Velocity {
    type Output = Length;
    fn mul(self, rhs: Time) -> Length {
        Length(self.0 * rhs.0)
    }
}

impl Mul<Time> for AngularVelocity {
    type Output = Angle;
    fn mul(self, rhs: Time) -> Angle {
        Angle(self.0 * rhs.0)
    }
}

impl Mul<Angle> for AngularProportionalGain {
    type Output = Voltage;
    fn mul(self, rhs: Angle) -> Voltage {
        Voltage(self.0 * rhs.0)
    }
}

impl Mul<Length> for LinearProportionalGain {
    type Output = Voltage;
    fn mul(self, rhs: Length) -> Voltage {
        Voltage(self.0 * rhs.0)
    }
}

impl Mul<Velocity> for LinearVelocityFeedforward {
    type Output = Voltage;
    fn mul(self, rhs: Velocity) -> Voltage {
        Voltage(self.0 * rhs.0)
    }
}

impl Mul<AngularVelocity> for AngularVelocityFeedforward {
    type Output = Voltage;
    fn mul(self, rhs: AngularVelocity) -> Voltage {
        Voltage(self.0 * rhs.0)
    }
}

/// Temperature measurement with affine conversions.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Temperature(f64); // Kelvin

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemperatureUnit {
    Kelvin,
    Celsius,
    Fahrenheit,
}

impl Temperature {
    pub fn from(val: f64, unit: TemperatureUnit) -> Self {
        match unit {
            TemperatureUnit::Kelvin => Self(val),
            TemperatureUnit::Celsius => Self(val + 273.15),
            TemperatureUnit::Fahrenheit => Self((val - 32.0) * (5.0 / 9.0) + 273.15),
        }
    }

    pub fn to(&self, unit: TemperatureUnit) -> f64 {
        match unit {
            TemperatureUnit::Kelvin => self.0,
            TemperatureUnit::Celsius => self.0 - 273.15,
            TemperatureUnit::Fahrenheit => (self.0 - 273.15) * (9.0 / 5.0) + 32.0,
        }
    }
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.3} m", self.0)
    }
}

impl fmt::Display for Voltage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} V", self.0)
    }
}

impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} deg", self.to_degrees())
    }
}

/// Direction of motion or rotation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Direction {
    #[default]
    Forward,
    Reverse,
}

impl Direction {
    /// Returns 1.0 for Forward and -1.0 for Reverse.
    pub const fn sign(&self) -> f64 {
        match self {
            Self::Forward => 1.0,
            Self::Reverse => -1.0,
        }
    }
}

/// Type alias for compatibility.
pub type DirectionType = Direction;

/// Motor braking mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BrakeMode {
    #[default]
    Coast,
    Brake,
    Hold,
}

/// Type alias for compatibility.
pub type BrakeType = BrakeMode;

/// Angular displacement unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RotationUnits {
    #[default]
    Deg,
    Rev,
    Raw,
}

/// Rotational velocity unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VelocityUnits {
    #[default]
    Pct,
    Rpm,
    Dps,
}

/// Electrical potential unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VoltageUnits {
    #[default]
    Volt,
    Mv,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_units_conversions() {
        let l = Length::inches(10.0);
        assert!((l.to_meters() - 0.254).abs() < 1e-6);

        let ang = Angle::revolutions(1.0);
        assert!((ang.to_degrees() - 360.0).abs() < 1e-6);

        let v = Voltage::volts(12.0);
        assert_eq!(v.to_millivolts(), 12000.0);

        let kp = AngularProportionalGain::from_base(8.0 / (2.0 * PI)); // 8 V / rev
        let error = Angle::revolutions(0.5);
        let out_v = kp * error;
        assert!((out_v.to_volts() - 4.0).abs() < 1e-6);
    }
}
