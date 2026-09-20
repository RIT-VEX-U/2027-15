#include "competition/opcontrol.h"

#include "robot-config.h"

void (*opcontrol_ptr)() = opcontrol;

void opcontrol() {
  drive_sys.stop();

  while (true) {
    const double throttle = controller.Axis3.value() / 100.0;
    const double turn = controller.Axis1.value() / 100.0;

    drive_sys.drive_arcade(throttle, turn, 3, TankDrive::BrakeType::None, 0.05, 1.0, true);

    vexDelay(10);
  }
}
