#include "competition/opcontrol.h"

#include "robot-config.h"

vex::controller Con;
vex::motor m(vex::PORT20);

static double mult = 0.6;


void opcontrol() {
    while (true) {
        if (Con.ButtonUp.pressing()) {
            m.spin(vex::fwd, mult*12, vex::volt);
        } else if (Con.ButtonDown.pressing()) {
            m.spin(vex::reverse, mult*12, vex::volt);
        } else {
            m.stop();
        }

        double axis = (double)(Con.Axis3.position())/100;
        if (std::abs(axis) > 0.05) {
            m.spin(vex::fwd, axis * 12, vex::volt);
            Con.Screen.clearScreen();
            Con.Screen.setCursor(1, 1);
            Con.Screen.print("v=%0.02f\n", axis*12);
        }

        vexDelay(10);
    }
}
