//! Advanced feedback, feedforward, state-space, and motion profile controllers.

pub mod bang_bang;
pub mod feedback_base;
pub mod feedforward;
pub mod motion_controller;
pub mod pid;
pub mod pidff;
pub mod state_space;
pub mod trapezoid_profile;

pub use bang_bang::BangBang;
pub use feedback_base::Feedback;
pub use feedforward::{FeedForward, FeedForwardConfig};
pub use motion_controller::{MotionController, MotionProfileConfig};
pub use pid::{ErrorType, PIDConfig, PID};
pub use pidff::PIDFF;
pub use state_space::*;
pub use trapezoid_profile::{MotionState, TrapezoidProfile};
