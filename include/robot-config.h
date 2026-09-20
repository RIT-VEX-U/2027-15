#pragma once

#include "core.h"
#include "vex.h"

// ================ VEX ================

extern vex::brain brain;
extern vex::competition competition;
extern vex::controller controller;

// ================ INPUTS ================
// Digital sensors

// Analog sensors

// ================ OUTPUTS ================
// Motors
extern vex::motor left1;
extern vex::motor left2;
extern vex::motor left3;
extern vex::motor left4;
extern vex::motor_group left_motors;

extern vex::motor right1;
extern vex::motor right2;
extern vex::motor right3;
extern vex::motor right4;
extern vex::motor_group right_motors;

// Pneumatics

// ================ SUBSYSTEMS ================
extern TankDrive drive_sys;

// ================ UTILS ================

extern Initializer initializer;
