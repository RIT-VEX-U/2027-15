#pragma once

#include "core.h"
#include "vex.h"

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

void robot_init();
