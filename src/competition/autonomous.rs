//! Autonomous routines execution.
//!
//! Mirrors `src/competition/autonomous.cpp`.

use crate::config::RobotHardware;

/// Main autonomous routine executed during the 15-second / 45-second autonomous period.
pub fn autonomous(_robot: &mut RobotHardware) {
    // Autonomous motion logic executed here
}

/// Autonomous routine executed during Robot Skills Challenges.
pub fn skills(_robot: &mut RobotHardware) {
    // Skills autonomous motion logic executed here
}
