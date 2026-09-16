#include "vex.h"

#include "robot-config.h"

#include "competition/autonomous.h"
#include "competition/opcontrol.h"

vex::competition comp;

/**
 * Entry point to the program. No code should be placed here;
 * instead use competition/opcontrol.cpp and
 * competition/autonomous.cpp
 */
int main() {
  robot_init();
  comp.autonomous(autonomous);
  comp.drivercontrol(opcontrol);

}
