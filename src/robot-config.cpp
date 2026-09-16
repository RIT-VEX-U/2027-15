#include "robot-config.h"

#include <vex_controller.h>
#include <vex_global.h>
#include <vex_motorgroup.h>
#include <vex_units.h>

#include <cstddef>

#include "competition/autonomous.h"
#include "competition/opcontrol.h"
#include "core/robot_specs.h"
#include "core/subsystems/screen/legacy.h"
#include "core/subsystems/screen/screen_controller.h"
#include "core/subsystems/tank_drive.h"

vex::brain brain;
vex::competition competition;
vex::controller controller;

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

PID::pid_config_t pid_master{
    .p = 0.019, .i = 0.005, .d = 0.00125, .deadband = 1, .on_target_time = 0.1
};

PID pid_turn(pid_master);

robot_specs_t robot_specs{
    .robot_radius = 10,
    .odom_wheel_diam = 2.75,
    .odom_gear_ratio = 1,
    .dist_between_wheels = 12.0,
    .drive_correction_cutoff = 5,
    .turn_feedback = &pid_turn
};

OdometryTank odom_master(left_motors, right_motors, robot_specs, &inert_master);

TankDrive drive_sys(left_motors, right_motors, robot_specs, &odom_master);

std::vector<Initialization> inits = {};

LegacyScreen::LegacyPage init_page, match_page;
Initializer initializer([]() {
  // Initialization code here
  inert_master.calibrate();
  while (inert_master.isCalibrating()) {
    vexDelay(10);
  }
  printf("random print line here\n");
  ScreenController::set((match_page = LegacyScreen::LegacyPage(
                             brain.Screen,
                             {new LegacyScreen::PIDPage(pid_turn, "turn_pid", nullptr)}
                         ))
                            .handle());
});
