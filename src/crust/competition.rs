//! VEX V5 Competition Control abstraction.

use std::sync::{Arc, Mutex};

type CompetitionCallback = Box<dyn Fn() + Send + Sync + 'static>;

/// Competition controller managing match phases (autonomous, driver control, disabled).
#[derive(Default)]
pub struct Competition {
    autonomous_cb: Arc<Mutex<Option<CompetitionCallback>>>,
    drivercontrol_cb: Arc<Mutex<Option<CompetitionCallback>>>,
    is_autonomous_mode: Arc<Mutex<bool>>,
    is_driver_mode: Arc<Mutex<bool>>,
    is_enabled_mode: Arc<Mutex<bool>>,
}

impl Competition {
    /// Creates a new Competition instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers the autonomous callback function.
    pub fn autonomous<F>(&self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        *self.autonomous_cb.lock().unwrap() = Some(Box::new(callback));
    }

    /// Registers the driver control callback function.
    pub fn drivercontrol<F>(&self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        *self.drivercontrol_cb.lock().unwrap() = Some(Box::new(callback));
    }

    /// Runs the registered autonomous callback.
    pub fn trigger_autonomous(&self) {
        *self.is_autonomous_mode.lock().unwrap() = true;
        *self.is_driver_mode.lock().unwrap() = false;
        *self.is_enabled_mode.lock().unwrap() = true;

        if let Some(ref cb) = *self.autonomous_cb.lock().unwrap() {
            cb();
        }
    }

    /// Runs the registered driver control callback.
    pub fn trigger_drivercontrol(&self) {
        *self.is_autonomous_mode.lock().unwrap() = false;
        *self.is_driver_mode.lock().unwrap() = true;
        *self.is_enabled_mode.lock().unwrap() = true;

        if let Some(ref cb) = *self.drivercontrol_cb.lock().unwrap() {
            cb();
        }
    }

    /// Returns true if currently in autonomous phase.
    pub fn is_autonomous(&self) -> bool {
        *self.is_autonomous_mode.lock().unwrap()
    }

    /// Returns true if currently in driver control phase.
    pub fn is_driver_control(&self) -> bool {
        *self.is_driver_mode.lock().unwrap()
    }

    /// Returns true if the robot is currently enabled by field control.
    pub fn is_enabled(&self) -> bool {
        *self.is_enabled_mode.lock().unwrap()
    }
}
