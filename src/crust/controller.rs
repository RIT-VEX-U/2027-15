//! Concrete VEX V5 Handheld Controller driver (Primary & Partner).

pub use crate::mantle::controller::{Controller, ControllerAxis, ControllerButton};

/// Controller identity: Primary or Partner controller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ControllerId {
    #[default]
    Primary,
    Partner,
}

/// VEX V5 Handheld Controller connected over VEXnet or Bluetooth.
pub type V5Controller = Controller;
