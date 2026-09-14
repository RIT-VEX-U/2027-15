#include "vex.h"

#include "robot-config.h"

#include "competition/autonomous.h"
#include "competition/opcontrol.h"

#include "v5_api.h"
#include "v5_display_documented.h"

#include "inttypes.h"

vex::competition comp;

extern "C"
__attribute__((section(".boot_data"), used, aligned(4)))
const std::uint32_t vexCodeSig[8] = {
    0x35585658, // +0x00: "XVX5"
    0x00000000, // +0x04: V5_SIG_TYPE_USER
    0x00000001, // +0x08: owner -- see note below
    0x00000000, // +0x0C: normal startup options

    0x00000000, // +0x10
    0x00000008, // +0x14: fullscreen
    0x00000000, // +0x18
    0x00000000  // +0x1C
};

#define VEXOS_TABLE_BASE 0x037fc000
#define SLOT_FCNTL_USER 0x0838
#define SLOT_ENABLE_REMOTE_CONSOLE 0x08a8

#define VEX_JUMPTABLE_START ((uintptr_t)0x037fc000u)

void __vex_function_prolog(void);

#define VEX_JUMPTABLE_CALL(ret, params, call_args, offset)                                      \
    return ((ret (*__attribute__((pcs("aapcs")))) params)(                                      \
                *(volatile uintptr_t *)(VEX_JUMPTABLE_START + (uintptr_t)(offset))))call_args;  \

#define VEX_JUMPTABLE_PROLOG(name, ret, params, call_args, offset)  \
    ret name params {                                               \
        __vex_function_prolog();                                    \
        VEX_JUMPTABLE_CALL(ret, params, call_args, offset)          \
    }

#define VEX_JUMPTABLE(name, ret, params, call_args, offset) \
    ret name params {                                       \
        VEX_JUMPTABLE_CALL(ret, params, call_args, offset)  \
    }

typedef int (*fcntl_user_fn)(int fd, int command, uintptr_t argument);
typedef void (*enable_console_fn)(void);
static void *vexos_slot(uint32_t offset) {
    return (void *)(uintptr_t)(*(volatile uint32_t*)(VEXOS_TABLE_BASE + offset));
}

typedef void (*mmu_section_map_fn)(uint32_t virtual_section, uint32_t descriptor);
static mmu_section_map_fn mmu_section_map = (mmu_section_map_fn)0x03444ff0;

VEX_JUMPTABLE(vexAssetsFind, uint32_t, (const char *name, const uint8_t **out_data), (name, out_data), 0x988)

/**
 * Entry point to the program. No code should be placed here;
 * instead use competition/opcontrol.cpp and
 * competition/autonomous.cpp
 */
int main() {
  //comp.autonomous(autonomous);
  //comp.drivercontrol(opcontrol);

  mmu_section_map(0x00000000, 0x00015de6);

  //uint32_t size = 480*272;
  //uint32_t *framebuffer = (uint32_t*)malloc(size * sizeof(uint32_t));


  for (int i = 0; i < 480*272; i++) {
      //framebuffer[i] = 0x00ffffff;
  }

  vex::brain Brain;

  const uint8_t *data = NULL;
  uint32_t size = vexAssetsFind("USER000D.bmp", &data);
  printf("size %d addr 0x%08" PRIx32 "\n", size, data);
  for (int i = 0; i < size; i++) {
  }



  vexDisplayOffsetSet(0, 0);
  vexDisplayClipRegionSet(0, 0, 479, 271);
  uint32_t start_addr = 0x00100000;
  Brain.Screen.drawImageFromBuffer((uint8_t*)data, 0, 0, size);
  while (true) {

    //vexDisplayCopyRect(0, 0, 479, 271, (uint32_t*)data, 480);
    //vexDisplayRender(true, true);
    //printf("0x%08" PRIx32 "\n", start_addr);
    //start_addr += 480*16;
    vexDelay(10);
  }

  //vexDisplayCopyRect(0, 0, 479, 271, framebuffer, 480);

  /*
  fcntl_user_fn hidden_fcntl = (fcntl_user_fn)vexos_slot(SLOT_FCNTL_USER);
  enable_console_fn enable_console = (enable_console_fn)vexos_slot(SLOT_ENABLE_REMOTE_CONSOLE);

  int result = hidden_fcntl(70, 32, 0x769);
  if (result >= 0) {
      printf("yep\n");
      enable_console();
  } else {
      printf("nope\n");
  }
  */
  while (true) {
      vexDelay(1000);
  }

}
