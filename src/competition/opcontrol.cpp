#include "competition/opcontrol.h"

#include "robot-config.h"

using namespace units::literals;

void (*opcontrol_ptr)() = opcontrol;

void opcontrol() {
  drive_sys.stop();
  static units::Voltage vin = 0_V;
  static units::Angle position = 0_rev;
  static units::Angle reference = 0_rev;

  constexpr double min = 0;
  constexpr double max = 4.4;
  constexpr units::AngularProportionalGain kp = 8_V / 1_rev;

  controller.ButtonUp.pressed([]() { reference += 0.5_rev; printf("%0.01f\n", reference.to(units::rev)); });
  controller.ButtonDown.pressed([]() { reference -= 0.5_rev; printf("%0.01f\n", reference.to(units::rev)); });

  while (true) {
    const double throttle = controller.Axis3.value() / 100.0;
    const double turn = controller.Axis1.value() / 100.0;
    const double axis2 = controller.Axis2.value() / 100.0;

    bool moveup = controller.ButtonL1.pressing();
    bool movedown = controller.ButtonL2.pressing();

    units::Angle position = units::Angle(lift_left.position(vex::rev), units::rev);

    vin = kp * (reference - position);

    lift_left.setBrake(vex::brake);
    lift_right.setBrake(vex::brake);
    // if (moveup) {
    //   lift_motors.spin(vex::forward, vin, vex::volt);
    // } else if (movedown) {
    //   lift_motors.spin(vex::forward, -vin, vex::volt);
    // } else {
    //   lift_motors.stop();
    // }
    lift_motors.spin(vex::forward, vin.to(units::volts), vex::volt);

    drive_sys.drive_arcade(throttle, turn, 2, 1, TankDrive::BrakeType::None, 0.05, 2.0, true);
    // drive_sys.drive_tank(throttle, axis2);

    vexDelay(10);
  }
}
