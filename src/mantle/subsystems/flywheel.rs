//! Flywheel velocity control subsystem.
//!
//! Mirrors `core/subsystems/flywheel.h` and `core/subsystems/flywheel.cpp`.

use crate::core::controls::feedback_base::Feedback;
use crate::core::controls::feedforward::FeedForward;
use crate::core::filter::Filter;
use crate::mantle::motor::{DirectionType, MotorGroup, VelocityUnits, VoltageUnits};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// Subsystem controlling a high-inertia spinning flywheel using feedforward and feedback.
pub struct Flywheel {
    motors: Arc<Mutex<MotorGroup>>,
    feedback: Arc<Mutex<Box<dyn Feedback>>>,
    feedforward: Arc<Mutex<FeedForward>>,
    ratio: f64,
    filter: Arc<Mutex<Box<dyn Filter>>>,
    target_rpm: Arc<Mutex<f64>>,
    running: Arc<AtomicBool>,
    worker_handle: Option<JoinHandle<()>>,
}

impl Flywheel {
    /// Creates a new flywheel subsystem instance.
    pub fn new(
        motors: MotorGroup,
        feedback: Box<dyn Feedback>,
        feedforward: FeedForward,
        ratio: f64,
        filter: Box<dyn Filter>,
    ) -> Self {
        Self {
            motors: Arc::new(Mutex::new(motors)),
            feedback: Arc::new(Mutex::new(feedback)),
            feedforward: Arc::new(Mutex::new(feedforward)),
            ratio,
            filter: Arc::new(Mutex::new(filter)),
            target_rpm: Arc::new(Mutex::new(0.0)),
            running: Arc::new(AtomicBool::new(false)),
            worker_handle: None,
        }
    }

    /// Measures filtered velocity in RPM.
    pub fn measure_rpm(&self) -> f64 {
        let raw_rpm = {
            let m = self.motors.lock().unwrap();
            self.ratio * m.velocity_units(VelocityUnits::Rpm)
        };
        let mut filt = self.filter.lock().unwrap();
        filt.add_entry(raw_rpm);
        filt.get_value()
    }

    /// Returns the most recently computed filtered RPM.
    pub fn get_rpm(&self) -> f64 {
        self.filter.lock().unwrap().get_value()
    }

    /// Returns the active target RPM.
    pub fn get_target(&self) -> f64 {
        *self.target_rpm.lock().unwrap()
    }

    /// Sets the motor directly with normalized voltage percent when background task is not running.
    pub fn spin_manual(&mut self, speed: f64, dir: DirectionType) {
        if !self.running.load(Ordering::Relaxed) {
            let m = self.motors.lock().unwrap();
            m.spin(dir, speed * 12.0, VoltageUnits::Volt);
        }
    }

    /// Commands a target velocity and starts the background closed-loop regulation task if idle.
    pub fn spin_rpm(&mut self, input_rpm: f64) {
        if input_rpm.abs() < 1e-6 {
            self.stop();
            return;
        }

        {
            let mut target = self.target_rpm.lock().unwrap();
            *target = input_rpm;
            let current_rpm = self.get_rpm();
            self.feedback.lock().unwrap().init(current_rpm, input_rpm);
        }

        if !self.running.load(Ordering::Relaxed) {
            self.running.store(true, Ordering::Relaxed);
            let motors = Arc::clone(&self.motors);
            let feedback = Arc::clone(&self.feedback);
            let feedforward = Arc::clone(&self.feedforward);
            let target_rpm = Arc::clone(&self.target_rpm);
            let running = Arc::clone(&self.running);
            let filter = Arc::clone(&self.filter);
            let ratio = self.ratio;

            let handle = thread::spawn(move || {
                while running.load(Ordering::Relaxed) {
                    let cur_target = *target_rpm.lock().unwrap();
                    let raw_rpm = {
                        let m = motors.lock().unwrap();
                        ratio * m.velocity_units(VelocityUnits::Rpm)
                    };
                    let rpm = {
                        let mut filt = filter.lock().unwrap();
                        filt.add_entry(raw_rpm);
                        filt.get_value()
                    };

                    if cur_target.abs() > 1e-6 {
                        let ff_out = feedforward.lock().unwrap().calculate(cur_target, 0.0, 0.0);
                        let fb_out = {
                            let mut fb = feedback.lock().unwrap();
                            fb.update(rpm);
                            fb.get()
                        };
                        let total_out = ff_out + fb_out;
                        let m = motors.lock().unwrap();
                        m.spin(DirectionType::Forward, total_out * 12.0, VoltageUnits::Volt);
                    }

                    thread::sleep(Duration::from_millis(5));
                }
                let m = motors.lock().unwrap();
                m.stop();
            });

            self.worker_handle = Some(handle);
        }
    }

    /// Returns true if the feedback controller reports the flywheel is on velocity target.
    pub fn is_on_target(&self) -> bool {
        self.feedback.lock().unwrap().is_on_target()
    }

    /// Stops the flywheel and its background control thread.
    pub fn stop(&mut self) {
        if self.running.swap(false, Ordering::Relaxed) {
            *self.target_rpm.lock().unwrap() = 0.0;
            if let Some(handle) = self.worker_handle.take() {
                let _ = handle.join();
            }
            self.motors.lock().unwrap().stop();
        }
    }
}

impl Drop for Flywheel {
    fn drop(&mut self) {
        self.stop();
    }
}
