#include "vex.h"

#include "robot-config.h"

#include "competition/autonomous.h"
#include "competition/opcontrol.h"

vex::competition comp;
extern "C" void vexTasksRun(void);
extern "C" void vexTaskYield(void);

struct SkillIssue {
  __attribute__((noinline)) void bricker() {
    volatile uint8_t *badptr = (volatile uint8_t *)0x00001000;
    uint8_t lol = *badptr;
    // __asm__ volatile("ldr %0, [%1]" : "=r"(lol) : "r"(0u) : "memory");
    printf("%x\n", lol);
  }
};

/**
 * Entry point to the program. No code should be placed here;
 * instead use competition/opcontrol.cpp and
 * competition/autonomous.cpp
 */
int main() {
  // comp.autonomous(autonomous);
  // comp.drivercontrol(opcontrol);

  // robot_init();
  //
  printf("start\n");

  SkillIssue a;

  a.bricker();

  while (true) {
    vexDelay(10);
  }
}
