#include "robot-config.h"

#include "competition/autonomous.h"
#include "competition/opcontrol.h"
#include "core/subsystems/screen/legacy.h"

vex::brain brain;
vex::competition competition;
vex::controller controller;

vex::motor left1(vex::PORT1, true);
vex::motor left2(vex::PORT2, true);
vex::motor left3(vex::PORT3, true);
vex::motor left4(vex::PORT4, true);
vex::motor_group left_motors(left1, left2, left3, left4);

vex::motor right1(vex::PORT5, false);
vex::motor right2(vex::PORT6, false);
vex::motor right3(vex::PORT7, false);
vex::motor right4(vex::PORT9, false);
vex::motor_group right_motors(right1, right2, right3, right4);

std::vector<Initialization> inits = {};

robot_specs_t robot_config{

};

TankDrive drive_sys(left_motors, right_motors, robot_config);

LegacyScreen::LegacyPage init_page, match_page;
Initializer initializer([]() {
  ScreenController::set((match_page = LegacyScreen::LegacyPage(
                             brain.Screen, {new LegacyScreen::StatsPage({
                                               {"left1", left1},
                                               {"left2", left2},
                                               {"left3", left3},
                                               {"left4", left4},

                                               {"right1", right1},
                                               {"right2", right2},
                                               {"right3", right3},
                                               {"right4", right4},
                                           })}
                         ))
                            .handle());
});
