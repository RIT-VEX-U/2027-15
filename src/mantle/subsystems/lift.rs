//! Articulated lift and arm positioning subsystem.
//!
//! Mirrors `core/subsystems/lift.h`.

use crate::core::controls::pid::{Pid, PidConfig};
use crate::mantle::motor::{DirectionType, MotorGroup, RotationUnits, VoltageUnits};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

/// Physical and algorithmic configuration for lift subsystems.
#[derive(Debug, Clone)]
pub struct LiftConfig {
    /// Maximum voltage speed when moving upward (volts).
    pub up_speed: f64,
    /// Maximum voltage speed when moving downward (volts).
    pub down_speed: f64,
    /// Upper softstop limit in revolutions.
    pub softstop_up: f64,
    /// Lower softstop limit in revolutions.
    pub softstop_down: f64,
    /// PID configuration for position holding.
    pub lift_pid_cfg: PidConfig,
}

impl Default for LiftConfig {
    fn default() -> Self {
        Self {
            up_speed: 12.0,
            down_speed: 12.0,
            softstop_up: 5.0,
            softstop_down: 0.0,
            lift_pid_cfg: PidConfig {
                p: 2.0,
                i: 0.0,
                d: 0.1,
                ..Default::default()
            },
        }
    }
}

/// Lift subsystem with position holding and discrete setpoint targeting.
pub struct Lift<P: Ord + Clone> {
    motors: Arc<Mutex<MotorGroup>>,
    cfg: LiftConfig,
    pid: Pid,
    setpoint_map: BTreeMap<P, f64>,
    setpoint: f64,
    custom_sensor: Option<Box<dyn Fn() -> f64 + Send + Sync>>,
}

impl<P: Ord + Clone> Lift<P> {
    /// Creates a new lift subsystem.
    pub fn new(
        motors: MotorGroup,
        cfg: LiftConfig,
        setpoint_map: BTreeMap<P, f64>,
        custom_sensor: Option<Box<dyn Fn() -> f64 + Send + Sync>>,
    ) -> Self {
        let pid = Pid::new(cfg.lift_pid_cfg);
        Self {
            motors: Arc::new(Mutex::new(motors)),
            cfg,
            pid,
            setpoint_map,
            setpoint: 0.0,
            custom_sensor,
        }
    }

    /// Reads the current lift position in revolutions.
    pub fn get_position(&self) -> f64 {
        if let Some(ref sensor) = self.custom_sensor {
            sensor()
        } else {
            self.motors.lock().unwrap().position(RotationUnits::Rev)
        }
    }

    /// Sets the target setpoint by position enum.
    pub fn set_setpoint(&mut self, pos: &P) {
        if let Some(&height) = self.setpoint_map.get(pos) {
            self.setpoint = height;
            self.pid.set_target(height);
        }
    }

    /// Actively holds the current setpoint using PID.
    pub fn hold(&mut self) {
        let cur = self.get_position();
        let output = self.pid.update(cur);
        let m = self.motors.lock().unwrap();
        m.spin(DirectionType::Forward, output, VoltageUnits::Volt);
    }

    /// Controls the lift using continuous up/down button inputs.
    pub fn control_continuous(&mut self, up_ctrl: bool, down_ctrl: bool) {
        let cur_pos = self.get_position();

        if up_ctrl && cur_pos < self.cfg.softstop_up {
            let m = self.motors.lock().unwrap();
            m.spin(
                DirectionType::Forward,
                self.cfg.up_speed,
                VoltageUnits::Volt,
            );
            self.setpoint = cur_pos;
            self.pid.set_target(cur_pos);
        } else if down_ctrl && cur_pos > self.cfg.softstop_down {
            let m = self.motors.lock().unwrap();
            m.spin(
                DirectionType::Reverse,
                self.cfg.down_speed,
                VoltageUnits::Volt,
            );
            self.setpoint = cur_pos;
            self.pid.set_target(cur_pos);
        } else {
            self.hold();
        }
    }
}
