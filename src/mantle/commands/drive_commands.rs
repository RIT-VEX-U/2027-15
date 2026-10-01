//! Drive commands wrapping `TankDrive` and `Odometry` methods.
//!
//! Mirrors `core/utils/command_structure/drive_commands.h` and `core/utils/command_structure/drive_commands.cpp`.

use super::auto_command::AutoCommand;
use crate::core::controls::feedback_base::Feedback;
use crate::core::geometry::{Pose2d, Translation2d};
use crate::core::pathing::pure_pursuit::Path;
use crate::mantle::motor::DirectionType;
use crate::mantle::subsystems::odometry::Odometry;
use crate::mantle::subsystems::tank_drive::TankDrive;
use std::sync::{Arc, Mutex};

/// AutoCommand wrapping `TankDrive::drive_forward`.
pub struct DriveForwardCommand {
    drive_sys: Arc<Mutex<TankDrive>>,
    feedback: Box<dyn Feedback>,
    inches: f64,
    dir: DirectionType,
    max_speed: f64,
    end_speed: f64,
}

impl DriveForwardCommand {
    /// Creates a new drive forward command.
    pub fn new(
        drive_sys: Arc<Mutex<TankDrive>>,
        feedback: Box<dyn Feedback>,
        inches: f64,
        dir: DirectionType,
        max_speed: f64,
        end_speed: f64,
    ) -> Self {
        Self {
            drive_sys,
            feedback,
            inches,
            dir,
            max_speed,
            end_speed,
        }
    }
}

impl AutoCommand for DriveForwardCommand {
    fn run(&mut self) -> bool {
        let mut drive = self.drive_sys.lock().unwrap();
        drive.drive_forward(
            self.inches,
            self.dir,
            &mut *self.feedback,
            self.max_speed,
            self.end_speed,
        )
    }

    fn to_string(&self) -> String {
        format!(
            "Driving {:?} {:.1} inches at {:.0}% speed",
            self.dir,
            self.inches,
            self.max_speed * 100.0
        )
    }

    fn on_timeout(&mut self) {
        let mut drive = self.drive_sys.lock().unwrap();
        drive.stop();
        drive.reset_auto();
    }
}

/// AutoCommand wrapping `TankDrive::turn_degrees`.
pub struct TurnDegreesCommand {
    drive_sys: Arc<Mutex<TankDrive>>,
    feedback: Box<dyn Feedback>,
    degrees: f64,
    max_speed: f64,
    end_speed: f64,
}

impl TurnDegreesCommand {
    /// Creates a new turn degrees command.
    pub fn new(
        drive_sys: Arc<Mutex<TankDrive>>,
        feedback: Box<dyn Feedback>,
        degrees: f64,
        max_speed: f64,
        end_speed: f64,
    ) -> Self {
        Self {
            drive_sys,
            feedback,
            degrees,
            max_speed,
            end_speed,
        }
    }
}

impl AutoCommand for TurnDegreesCommand {
    fn run(&mut self) -> bool {
        let mut drive = self.drive_sys.lock().unwrap();
        drive.turn_degrees(
            self.degrees,
            &mut *self.feedback,
            self.max_speed,
            self.end_speed,
        )
    }

    fn to_string(&self) -> String {
        format!(
            "Turning {:.1} degrees at {:.0}% speed",
            self.degrees,
            self.max_speed * 100.0
        )
    }

    fn on_timeout(&mut self) {
        let mut drive = self.drive_sys.lock().unwrap();
        drive.stop();
        drive.reset_auto();
    }
}

/// AutoCommand wrapping `TankDrive::drive_to_point`.
pub struct DriveToPointCommand {
    drive_sys: Arc<Mutex<TankDrive>>,
    feedback: Box<dyn Feedback>,
    target: Translation2d,
    dir: DirectionType,
    max_speed: f64,
    end_speed: f64,
}

impl DriveToPointCommand {
    /// Creates a new drive to point command.
    pub fn new(
        drive_sys: Arc<Mutex<TankDrive>>,
        feedback: Box<dyn Feedback>,
        target: Translation2d,
        dir: DirectionType,
        max_speed: f64,
        end_speed: f64,
    ) -> Self {
        Self {
            drive_sys,
            feedback,
            target,
            dir,
            max_speed,
            end_speed,
        }
    }
}

impl AutoCommand for DriveToPointCommand {
    fn run(&mut self) -> bool {
        let mut drive = self.drive_sys.lock().unwrap();
        drive.drive_to_point(
            self.target.x(),
            self.target.y(),
            self.dir,
            &mut *self.feedback,
            self.max_speed,
            self.end_speed,
        )
    }

    fn to_string(&self) -> String {
        format!(
            "Driving {:?} to ({:.1}, {:.1}) at {:.0}% speed",
            self.dir,
            self.target.x(),
            self.target.y(),
            self.max_speed * 100.0
        )
    }

    fn on_timeout(&mut self) {
        let mut drive = self.drive_sys.lock().unwrap();
        drive.stop();
        drive.reset_auto();
    }
}

/// AutoCommand wrapping `TankDrive::turn_to_heading`.
pub struct TurnToHeadingCommand {
    drive_sys: Arc<Mutex<TankDrive>>,
    feedback: Box<dyn Feedback>,
    heading_deg: f64,
    max_speed: f64,
    end_speed: f64,
}

impl TurnToHeadingCommand {
    /// Creates a new turn to heading command.
    pub fn new(
        drive_sys: Arc<Mutex<TankDrive>>,
        feedback: Box<dyn Feedback>,
        heading_deg: f64,
        max_speed: f64,
        end_speed: f64,
    ) -> Self {
        Self {
            drive_sys,
            feedback,
            heading_deg,
            max_speed,
            end_speed,
        }
    }
}

impl AutoCommand for TurnToHeadingCommand {
    fn run(&mut self) -> bool {
        let mut drive = self.drive_sys.lock().unwrap();
        drive.turn_to_heading(
            self.heading_deg,
            &mut *self.feedback,
            self.max_speed,
            self.end_speed,
        )
    }

    fn to_string(&self) -> String {
        format!(
            "Turning to heading {:.1} deg at {:.0}% speed",
            self.heading_deg,
            self.max_speed * 100.0
        )
    }

    fn on_timeout(&mut self) {
        let mut drive = self.drive_sys.lock().unwrap();
        drive.stop();
        drive.reset_auto();
    }
}

/// AutoCommand wrapping `TankDrive::pure_pursuit`.
pub struct PurePursuitCommand {
    drive_sys: Arc<Mutex<TankDrive>>,
    feedback: Box<dyn Feedback>,
    path: Path,
    dir: DirectionType,
    max_speed: f64,
    end_speed: f64,
}

impl PurePursuitCommand {
    /// Creates a new pure pursuit path following command.
    pub fn new(
        drive_sys: Arc<Mutex<TankDrive>>,
        feedback: Box<dyn Feedback>,
        path: Path,
        dir: DirectionType,
        max_speed: f64,
        end_speed: f64,
    ) -> Self {
        Self {
            drive_sys,
            feedback,
            path,
            dir,
            max_speed,
            end_speed,
        }
    }
}

impl AutoCommand for PurePursuitCommand {
    fn run(&mut self) -> bool {
        let mut drive = self.drive_sys.lock().unwrap();
        drive.pure_pursuit(
            &self.path,
            self.dir,
            &mut *self.feedback,
            self.max_speed,
            self.end_speed,
        )
    }

    fn to_string(&self) -> String {
        format!(
            "Pure pursuit path ({:?} waypoints) at {:.0}% speed",
            self.path.get_points().len(),
            self.max_speed * 100.0
        )
    }

    fn on_timeout(&mut self) {
        let mut drive = self.drive_sys.lock().unwrap();
        drive.stop();
        drive.reset_auto();
    }
}

/// AutoCommand stopping the drivetrain.
pub struct DriveStopCommand {
    drive_sys: Arc<Mutex<TankDrive>>,
}

impl DriveStopCommand {
    /// Creates a new drive stop command.
    pub fn new(drive_sys: Arc<Mutex<TankDrive>>) -> Self {
        Self { drive_sys }
    }
}

impl AutoCommand for DriveStopCommand {
    fn run(&mut self) -> bool {
        let mut drive = self.drive_sys.lock().unwrap();
        drive.stop();
        true
    }

    fn to_string(&self) -> String {
        "Stopping the drive".to_string()
    }

    fn on_timeout(&mut self) {
        let mut drive = self.drive_sys.lock().unwrap();
        drive.reset_auto();
    }
}

/// AutoCommand that overrides the current odometry position.
pub struct OdomSetPositionCommand {
    odom: Arc<Mutex<dyn Odometry>>,
    new_pos: Pose2d,
}

impl OdomSetPositionCommand {
    /// Creates a new odometry pose override command.
    pub fn new(odom: Arc<Mutex<dyn Odometry>>, new_pos: Pose2d) -> Self {
        Self { odom, new_pos }
    }
}

impl AutoCommand for OdomSetPositionCommand {
    fn run(&mut self) -> bool {
        let mut o = self.odom.lock().unwrap();
        o.set_position(self.new_pos);
        true
    }

    fn to_string(&self) -> String {
        format!(
            "Setting position to X: {:.2}, Y: {:.2}, ROT: {:.1} deg",
            self.new_pos.x(),
            self.new_pos.y(),
            self.new_pos.rotation().degrees()
        )
    }
}
