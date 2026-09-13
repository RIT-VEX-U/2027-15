#include "competition/opcontrol.h"

#include "robot-config.h"

void opcontrol() {
  double left_axis;
  double right_axis;

  while (true) {
    left_axis = (double)Con.Axis3.position() / 100;
    right_axis = (double)Con.Axis1.position() / 100;

    drive_sys.drive_arcade(left_axis, right_axis);

    printf(
        "X: %.2f, Y: %.2f, Rot: %.2f\n", drive_sys.get_position().x(), drive_sys.get_position().y(),
        drive_sys.get_position().rotation().degrees()
    );
    vexDelay(10);
  }
}
