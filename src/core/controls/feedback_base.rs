//! Abstract base trait for feedback controllers (PID, BangBang, MotionController, etc.).

/// Interface so that subsystems can easily switch between feedback loops.
pub trait Feedback: Send + Sync {
    /// Initialize the feedback controller for a movement.
    fn init(&mut self, start_pt: f64, set_pt: f64);

    /// Iterate the feedback loop once with an updated sensor value.
    fn update(&mut self, val: f64) -> f64;

    /// Returns the last saved output result from the feedback controller.
    fn get(&self) -> f64;

    /// Clamp the lower and upper limits of the output.
    /// If both are 0.0, no limits are applied.
    fn set_limits(&mut self, lower: f64, upper: f64);

    /// Returns true if the feedback controller has reached its setpoint.
    fn is_on_target(&self) -> bool;
}
