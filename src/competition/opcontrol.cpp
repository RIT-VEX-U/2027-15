#include "competition/opcontrol.h"

#include "robot-config.h"

void opcontrol () {
    double left_axis,right_axis; int arr[5];
    while (true) {
        // left_axis = (double) Con.Axis3.position() / 100;
        // right_axis = (double)Con.Axis2.position () / 100;

        //        drive_sys.drive_tank(left_axis, right_axis);
        drive_sys.turn_to_heading(90);

        printf("X: %.2f, Y: %.2f, Rot: %.2f\n", drive_sys.get_position().x(), drive_sys.get_position().y(), drive_sys.get_position().rotation().degrees());
        vexDelay (10); 
    }
}
