#include "robot-config.h"

#include <vex_controller.h>
#include <vex_global.h>
#include <vex_motorgroup.h>
#include <vex_units.h>

#include "core/robot_specs.h"
#include "core/subsystems/tank_drive.h"

vex::controller Con;

vex::motor left1(vex::PORT1, vex::gearSetting::ratio6_1, true);
vex::motor left2(vex::PORT2, vex::gearSetting::ratio6_1, true);
vex::motor left3(vex::PORT3, vex::gearSetting::ratio6_1, true);
vex::motor left4(vex::PORT1, vex::gearSetting::ratio6_1, true);

vex::motor_group left_motors(left1, left2, left3, left4);

vex::motor right1(vex::PORT5, vex::gearSetting::ratio6_1, false);
vex::motor right2(vex::PORT6, vex::gearSetting::ratio6_1, false);
vex::motor right3(vex::PORT7, vex::gearSetting::ratio6_1, false);
vex::motor right4(vex::PORT9, vex::gearSetting::ratio6_1, false);

vex::motor_group right_motors(right1, right2, right3, right4);

PID drive_pid(0.05, 0, 0.005, 0.5, 0.1);

robot_specs_t robot_specs{
    .robot_radius = 10,
    .odom_wheel_diam = 2.75,
    .odom_gear_ratio = 0.82758620689,
    .dist_between_wheels = 11.75,
    .drive_feedback = &drive_pid,
};

OdometryTank odom_tank(left_motors, right_motors, robot_specs);

TankDrive drive_sys(left_motors, right_motors, robot_specs, &odom_tank);

void robot_init() { odom_tank.set_position({0, 0, 0}); }
