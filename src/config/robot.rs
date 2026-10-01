//! Robot hardware configuration, port mapping, and global subsystem wiring.

use super::ports::*;
use super::specs::RobotSpecs;
use crate::crust::brain::Brain;
use crate::crust::competition::Competition;
use crate::crust::controller::Controller;
use crate::crust::motor::{Motor, V5Motor};
use crate::mantle::display::{LegacyPage, ScreenController, StatsPage};
use crate::mantle::initializer::Initializer;
use crate::mantle::motor::{Motor as MotorTrait, MotorGroup};
use crate::mantle::subsystems::tank_drive::TankDrive;
use std::sync::{Arc, Mutex};

/// Hardware container aggregating all sensors, motors, and subsystems on the robot.
pub struct RobotHardware {
    pub brain: Brain,
    pub competition: Competition,
    pub controller: Controller,

    // Drive motors
    pub left1: Motor,
    pub left2: Motor,
    pub left3: Motor,
    pub left4: Motor,
    pub left_motors: MotorGroup,

    pub right1: Motor,
    pub right2: Motor,
    pub right3: Motor,
    pub right4: Motor,
    pub right_motors: MotorGroup,

    // Lift motors
    pub lift_left: Motor,
    pub lift_right: Motor,
    pub lift_motors: MotorGroup,

    // Drivetrain subsystem
    pub drive_sys: Arc<Mutex<TankDrive>>,

    // Screen and initialization
    pub screen_controller: Arc<Mutex<ScreenController>>,
    pub initializer: Initializer,
}

impl Default for RobotHardware {
    fn default() -> Self {
        Self::new()
    }
}

impl RobotHardware {
    /// Initializes all physical hardware devices matching the 2027-15 robot specification.
    pub fn new() -> Self {
        let brain = Brain::new();
        let competition = Competition::new();
        let controller = Controller::new();

        // Left drive motors (reversed = true)
        let left1 = V5Motor::new(PORT_LEFT_DRIVE_1, true);
        let left2 = V5Motor::new(PORT_LEFT_DRIVE_2, true);
        let left3 = V5Motor::new(PORT_LEFT_DRIVE_3, true);
        let left4 = V5Motor::new(PORT_LEFT_DRIVE_4, true);

        let left1_m: Arc<Mutex<Box<dyn MotorTrait>>> =
            Arc::new(Mutex::new(Box::new(left1.clone())));
        let left2_m: Arc<Mutex<Box<dyn MotorTrait>>> =
            Arc::new(Mutex::new(Box::new(left2.clone())));
        let left3_m: Arc<Mutex<Box<dyn MotorTrait>>> =
            Arc::new(Mutex::new(Box::new(left3.clone())));
        let left4_m: Arc<Mutex<Box<dyn MotorTrait>>> =
            Arc::new(Mutex::new(Box::new(left4.clone())));

        let left_motors = MotorGroup::new_with_motors(vec![
            left1_m.clone(),
            left2_m.clone(),
            left3_m.clone(),
            left4_m.clone(),
        ]);

        // Right drive motors (reversed = false)
        let right1 = V5Motor::new(PORT_RIGHT_DRIVE_1, false);
        let right2 = V5Motor::new(PORT_RIGHT_DRIVE_2, false);
        let right3 = V5Motor::new(PORT_RIGHT_DRIVE_3, false);
        let right4 = V5Motor::new(PORT_RIGHT_DRIVE_4, false);

        let right1_m: Arc<Mutex<Box<dyn MotorTrait>>> =
            Arc::new(Mutex::new(Box::new(right1.clone())));
        let right2_m: Arc<Mutex<Box<dyn MotorTrait>>> =
            Arc::new(Mutex::new(Box::new(right2.clone())));
        let right3_m: Arc<Mutex<Box<dyn MotorTrait>>> =
            Arc::new(Mutex::new(Box::new(right3.clone())));
        let right4_m: Arc<Mutex<Box<dyn MotorTrait>>> =
            Arc::new(Mutex::new(Box::new(right4.clone())));

        let right_motors = MotorGroup::new_with_motors(vec![
            right1_m.clone(),
            right2_m.clone(),
            right3_m.clone(),
            right4_m.clone(),
        ]);

        // Lift motors
        let lift_left = V5Motor::new(PORT_LIFT_LEFT, false);
        let lift_right = V5Motor::new(PORT_LIFT_RIGHT, true);

        let lift_l_m: Arc<Mutex<Box<dyn MotorTrait>>> =
            Arc::new(Mutex::new(Box::new(lift_left.clone())));
        let lift_r_m: Arc<Mutex<Box<dyn MotorTrait>>> =
            Arc::new(Mutex::new(Box::new(lift_right.clone())));

        let lift_motors = MotorGroup::new_with_motors(vec![lift_l_m.clone(), lift_r_m.clone()]);

        let robot_specs = RobotSpecs::default();
        let drive_sys = Arc::new(Mutex::new(TankDrive::new(
            left_motors.clone(),
            right_motors.clone(),
            robot_specs,
            None,
        )));

        let screen_controller = Arc::new(Mutex::new(ScreenController::new()));

        let sc_clone = Arc::clone(&screen_controller);
        let stats_motors = vec![
            ("left1", left1_m),
            ("left2", left2_m),
            ("left3", left3_m),
            ("left4", left4_m),
            ("right1", right1_m),
            ("right2", right2_m),
            ("right3", right3_m),
            ("right4", right4_m),
            ("lift_left", lift_l_m),
            ("lift_right", lift_r_m),
        ];

        let initializer = Initializer::new_simple(move || {
            let stats_page = Box::new(StatsPage::new(stats_motors.clone()));
            let legacy_page = Arc::new(Mutex::new(LegacyPage::new(vec![stats_page])));
            let brain_screen = Arc::new(Mutex::new(Brain::new().screen));

            let lp = Arc::clone(&legacy_page);
            let bs = Arc::clone(&brain_screen);
            sc_clone.lock().unwrap().set(
                move || {
                    let mut page = lp.lock().unwrap();
                    let mut screen = bs.lock().unwrap();
                    page.update(false, 0, 0);
                    page.draw(&mut *screen, false, 0);
                },
                None,
                true,
            );
        });

        Self {
            brain,
            competition,
            controller,
            left1,
            left2,
            left3,
            left4,
            left_motors,
            right1,
            right2,
            right3,
            right4,
            right_motors,
            lift_left,
            lift_right,
            lift_motors,
            drive_sys,
            screen_controller,
            initializer,
        }
    }
}
