#include "vex.h"

#include "robot-config.h"
#include "competition/autonomous.h"
#include "competition/opcontrol.h"

/**
 * Entry point to the program. No code should be placed here;
 * instead use competition/opcontrol.cpp and
 * competition/autonomous.cpp
 */
int main() {
  initializer.initialize();

  competition.autonomous(autonomous_ptr);
  competition.drivercontrol(opcontrol_ptr);
}