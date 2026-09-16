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

// Pneumatics

// ================ SUBSYSTEMS ================

extern vex::controller Con;
extern TankDrive drive_sys;
extern OdometryTank odom_master;

// ================ UTILS ================

extern Initializer initializer;
