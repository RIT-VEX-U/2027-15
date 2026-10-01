//! Driver control (teleoperation) period routine.
//!
//! Mirrors `src/competition/opcontrol.cpp`.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use crate::config::RobotHardware;
use crate::mantle::motor::{BrakeType as MotorBrakeType, DirectionType, RotationUnits, VoltageUnits};
use crate::mantle::subsystems::tank_drive::BrakeType;

/// Executes driver teleoperation control loop.
pub fn opcontrol(robot: &mut RobotHardware, running_flag: Option<Arc<AtomicBool>>) {
    robot.drive_sys.lock().unwrap().stop();

    let mut reference_rev = 0.0;
    let kp = 8.0; // 8 Volts per 1 revolution error

    robot.lift_left.set_brake(MotorBrakeType::Brake);
    robot.lift_right.set_brake(MotorBrakeType::Brake);

    let is_running = move || {
        if let Some(ref flag) = running_flag {
            flag.load(Ordering::Relaxed)
        } else {
            true
        }
    };

    while is_running() {
        if robot.controller.button_up.is_pressed() {
            reference_rev += 0.5;
            println!("{:.2}", reference_rev);
        }
        if robot.controller.button_down.is_pressed() {
            reference_rev -= 0.5;
            println!("{:.2}", reference_rev);
        }

        let throttle = robot.controller.axis3.value() as f64 / 100.0;
        let turn = robot.controller.axis1.value() as f64 / 100.0;

        let position_rev = robot.lift_left.position(RotationUnits::Rev);
        let vin = kp * (reference_rev - position_rev);

        robot
            .lift_motors
            .spin(DirectionType::Forward, vin, VoltageUnits::Volt);

        robot.drive_sys.lock().unwrap().drive_arcade(
            throttle,
            turn,
            2,
            1,
            BrakeType::None,
            0.05,
            2.0,
            true,
        );

        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
