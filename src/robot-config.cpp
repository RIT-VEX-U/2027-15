#include "robot-config.h"

#include <vex_controller.h>
#include <vex_global.h>
#include <vex_motorgroup.h>
#include <vex_units.h>

#include "core/robot_specs.h"
#include "core/subsystems/tank_drive.h"

vex::controller Con;


vex::motor left1(vex::PORT7, vex::gearSetting::ratio6_1, false);
vex::motor left2(vex::PORT8, vex::gearSetting::ratio6_1, true);
vex::motor left3(vex::PORT9, vex::gearSetting::ratio6_1, true);
vex::motor left4(vex::PORT17, vex::gearSetting::ratio6_1, false);
vex::motor left5(vex::PORT18, vex::gearSetting::ratio6_1, true);

vex::motor_group left_motors(left1, left2, left3, left4, left5);

vex::motor right1(vex::PORT19, vex::gearSetting::ratio6_1, false);
vex::motor right2(vex::PORT11, vex::gearSetting::ratio6_1, false);
vex::motor right3(vex::PORT13, vex::gearSetting::ratio6_1, true);
vex::motor right4(vex::PORT15, vex::gearSetting::ratio6_1, false);
vex::motor right5(vex::PORT16, vex::gearSetting::ratio6_1, true);

vex::motor_group right_motors(right1, right2, right3, right4, right5);

vex::inertial inert_master(vex::PORT14);

robot_specs_t robot_specs{
    .robot_radius = 10,
    .odom_wheel_diam = 2.75,
    .odom_gear_ratio = 1,
    .dist_between_wheels = 12.0,
    .drive_correction_cutoff = 5,
};

OdometryTank odom_master(left_motors, right_motors, robot_specs, &inert_master);

TankDrive drive_sys(left_motors, right_motors, robot_specs, &odom_master);

void robot_init() {};
