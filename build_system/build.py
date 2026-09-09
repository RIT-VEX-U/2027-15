from __future__ import annotations

import argparse
import json
import os
import shlex
import shutil
import subprocess
import time
from datetime import datetime
from pathlib import Path

from .settings import (
    ProjectSettings,
    load_settings,
    sync_vscode,
    update_setting,
    validate_project_name,
)
from .toolchain import Toolchain, discover_toolchain
from .util import BUILD_DIR, PROJECT_FILE, ROOT, blue, bold, green, print_step, write_if_changed, yellow

SOURCE_GLOBS = (
    "src/**/*.c",
    "src/**/*.cpp",
    "core/src/**/*.c",
    "core/src/**/*.cpp",
)
PROJECT_INCLUDES = (
    ROOT / "include",
    ROOT / "core" / "include",
    ROOT / "vendor" / "eigen",
    ROOT / "vendor" / "gcem" / "include",
    ROOT / "vendor" / "cevalm" / "include",
)

COMMON_FLAGS = [
    "-DVexV5",
    "-target",
    "armv7a-none-eabi",
    "-fshort-enums",
    "-mfpu=vfpv3",
    "-mfloat-abi=softfp",
    "-Os",
    "-g3",
    "-fcolor-diagnostics",
    "-U__INT32_TYPE__",
    "-U__UINT32_TYPE__",
    "-D__INT32_TYPE__=long",
    "-D__UINT32_TYPE__=unsigned long",
]
C_FLAGS = ["-std=gnu99"]
CXX_FLAGS = [
    "-fno-rtti",
    "-fno-threadsafe-statics",
    "-fno-exceptions",
    "-std=gnu++26",
    "-ffunction-sections",
    "-fdata-sections",
]
WARNING_FLAGS = ["-Wall", "-Werror=return-type"]
LINK_LIBS = [
    "-lvexpatcher",
    "-lv5rt",
    "-lc++",
    "-lc++abi",
    "-lunwind",
    "-lm",
    "-lc",
    "-lnosys",
    "-lclang_rt.builtins",
]
OBJDUMP_FLAGS = ["--source", "--line-numbers", "--demangle", "--disassemble"]

# generates the makefile and compile_commands.json for clangd
def generate_build_files(name: str, sources: list[Path], toolchain: Toolchain, quiet: bool) -> None:
    includes = [f"-I{path}" for path in PROJECT_INCLUDES]
    system_includes = [
        "-isystem",
        str(toolchain.cxx_include_dir),
        "-isystem",
        str(toolchain.resource_dir / "include"),
        "-isystem",
        str(toolchain.newlib_include_dir),
        "-isystem",
        str(toolchain.sdk_path / "include"),
    ]
    base_flags = [f"-resource-dir={toolchain.resource_dir}", *COMMON_FLAGS, *(["-w"] if quiet else WARNING_FLAGS)]

    # write compile_commands.json
    commands = []
    for source in sources:
        obj = BUILD_DIR / "objects" / f"{source.relative_to(ROOT)}.obj"
        lang_flags = CXX_FLAGS if source.suffix == ".cpp" else C_FLAGS
        cmd_args = [*base_flags, *lang_flags, *includes, *system_includes, "-MMD", "-MP", "-MF", str(obj.with_suffix(".obj.d")), "-o", str(obj), "-c", str(source)]
        driver = "clang++" if source.suffix == ".cpp" else "clang"
        if os.name == "nt":
            driver += ".exe"
        commands.append({"directory": str(ROOT), "arguments": [driver, *cmd_args], "file": str(source), "output": str(obj)})

    write_if_changed(BUILD_DIR / "compile_commands.json", json.dumps(commands, indent=2) + "\n")

    # write Makefile
    obj_list = " \\\n  ".join(f"$(OBJROOT)/{source.relative_to(ROOT).as_posix()}.obj" for source in sources)
    ld_flags = [
        "-z",
        "norelro",
        "-T",
        toolchain.linker_script.as_posix(),
        "-e",
        "patcher_startup",
        "--gc-sections",
        f"-L{toolchain.sdk_path.as_posix()}",
        f"-L{toolchain.newlib_lib_dir.as_posix()}",
    ]
    makefile_path = BUILD_DIR / "Makefile"

    makefile = f"""\
.PHONY: all

PROJECT := {name}
BUILD_MAKEFILE := {makefile_path.relative_to(ROOT).as_posix()}
OBJROOT := build/objects

CLANG := "{toolchain.clang.as_posix()}"
LD := "{toolchain.linker.as_posix()}"
OBJCOPY := "{toolchain.objcopy.as_posix()}"
OBJDUMP := "{toolchain.objdump.as_posix()}"
SIZE := "{toolchain.size.as_posix()}"

ELF := build/$(PROJECT).elf
BIN := build/$(PROJECT).bin
ASM := build/$(PROJECT).S

COMMON_FLAGS := {shlex.join(base_flags)}
INCLUDES := {shlex.join(includes)}
SYSTEM_INCLUDES := {shlex.join(system_includes)}
CFLAGS := {shlex.join(C_FLAGS)}
CXXFLAGS := {shlex.join(CXX_FLAGS)}
LDFLAGS := {shlex.join(ld_flags)}
LDLIBS := {shlex.join(LINK_LIBS)}

OBJECTS := \\
  {obj_list}
DEPFILES := $(OBJECTS:.obj=.obj.d)

all: $(BIN) $(ASM)

$(OBJROOT)/%.cpp.obj: %.cpp $(BUILD_MAKEFILE)
\t@echo CXX      $<
\t@$(CLANG) $(COMMON_FLAGS) $(CXXFLAGS) $(INCLUDES) $(SYSTEM_INCLUDES) -MMD -MP -MF "$@.d" -o "$@" -c "$<"

$(OBJROOT)/%.c.obj: %.c $(BUILD_MAKEFILE)
\t@echo CC       $<
\t@$(CLANG) $(COMMON_FLAGS) $(CFLAGS) $(INCLUDES) $(SYSTEM_INCLUDES) -MMD -MP -MF "$@.d" -o "$@" -c "$<"

$(ELF): $(OBJECTS) $(BUILD_MAKEFILE)
\t@echo {'LINK':<8} $(PROJECT).elf
\t@$(LD) $(LDFLAGS) $(OBJECTS) -o "$@" --start-group $(LDLIBS) --end-group
\t@$(SIZE) "$@"

$(BIN): $(ELF)
\t@$(OBJCOPY) -O binary "$<" "$@"

$(ASM): $(ELF)
\t@$(OBJDUMP) {shlex.join(OBJDUMP_FLAGS)} "$<" > "$@"

-include $(DEPFILES)
"""
    write_if_changed(makefile_path, makefile)

# remove the build directory
def clean() -> int:
    if BUILD_DIR.exists():
        shutil.rmtree(BUILD_DIR)
        print_step(green("Cleaned build directory"))
    else:
        print_step(yellow("Build directory does not exist"))
    return 0

# compile changed files, link, strip
def build(args: argparse.Namespace, settings: ProjectSettings | None = None) -> int:
    start_time = time.time()
    if args.project_name is not None:
        update_setting("project", "name", validate_project_name(args.project_name))
        settings = None
    settings = settings or load_settings()
    name = settings.name
    toolchain = discover_toolchain(settings.sdk)
    # vex extension compatability, updates .vscode/vex_project_settings.json
    sync_vscode(settings, toolchain.sdk_path.parent.name)
    sources = sorted(source for glob in SOURCE_GLOBS for source in ROOT.glob(glob))
    jobs = args.parallel or os.cpu_count() or 1

    print_step(bold("VEX V5 Build"))
    print_step(f"{blue('Project:')} {name}")
    print_step(f"{blue('SDK:')} {toolchain.sdk_path}")
    print_step(f"{blue('Toolchain:')} {toolchain.toolchain_path}")
    print_step(f"{blue('Parallel jobs:')} {jobs}")

    for source in sources:
        (BUILD_DIR / "objects" / f"{source.relative_to(ROOT)}.obj").parent.mkdir(parents=True, exist_ok=True)

    generate_build_files(name, sources, toolchain, args.quiet)

    # run make
    result = subprocess.run(
        [str(toolchain.make), "--silent", "--no-print-directory", "-f", str(BUILD_DIR / "Makefile"), f"-j{jobs}"],
        cwd=ROOT,
        check=False,
    )
    if result.returncode != 0:
        return result.returncode

    for suffix in ("elf", "bin", "S"):
        artifact = BUILD_DIR / f"{name}.{suffix}"
        if artifact.exists():
            print_step(green("Output: ") + f"{artifact.name} ({artifact.stat().st_size:,} bytes)")

    print_step(green(f"Build successful ({time.time() - start_time:.1f}s)"))
    print(datetime.now().strftime(green("Time of Build: ") + ("%I:%M:%S %p")))
    return 0

# clean build
def rebuild(args: argparse.Namespace) -> int:
    clean()
    return build(args)

# check for vflash in the toolchain then try PATH
def find_vflash(settings: ProjectSettings) -> str | None:
    bundled = discover_toolchain(settings.sdk).vflash
    return str(bundled) if bundled.is_file() else shutil.which(bundled.name)

# switch to download channel async while building so we don't have to wait
def prepare_download_channel(vflash: str) -> subprocess.Popen[bytes] | None:
    try:
        return subprocess.Popen(
            [vflash, "channel", "download"],
            cwd=ROOT,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
    except OSError as error:
        print_step(yellow(f"Could not prepare download channel: {error}"))
        return None


# build and upload using project.toml settings
def upload(args: argparse.Namespace) -> int:
    if args.slot is not None:
        update_setting("upload", "slot", args.slot)
    settings = load_settings()
    vflash = find_vflash(settings)

    if vflash is None:
        print_step("Error: vflash not found in the toolchain or PATH")
        return 1

    # preemptively switch to download channel
    channel_process = prepare_download_channel(vflash) if vflash is not None else None

    try:
        result = build(args, settings)
    finally:
        if channel_process is not None:
            channel_process.wait()

    if result != 0:
        return result

    binary = BUILD_DIR / f"{settings.name}.bin"
    if not binary.exists():
        print_step(f"Error: binary not found: {binary}")
        return 1


    print_step(yellow(f"Uploading {binary.name} to slot {settings.slot} ({settings.strategy})..."))
    result = subprocess.run(
        [vflash, "upload", "--config", str(PROJECT_FILE), str(binary)],
        cwd=ROOT,
        check=False,
    )
    if result.returncode == 0:
        print_step(green("Upload successful"))
    return result.returncode

# runs program in slot
def run_program(args: argparse.Namespace) -> int:
    settings = load_settings()
    slot = args.slot or settings.slot

    vflash = find_vflash(settings)
    if vflash is None:
        print_step("Error: vflash not found in the toolchain or PATH")
        return 1

    command = [vflash, "run", str(slot)]
    if settings.terminal == "interactive":
        command.extend(("--terminal", "--interactive"))
    elif settings.terminal == "watch":
        command.extend(("--terminal", "--read-only"))

    print_step(yellow(f"Running program in slot {slot} ({settings.terminal})..."))
    return subprocess.run(command, cwd=ROOT, check=False).returncode

# stops currently running program
def stop() -> int:
    settings = load_settings()
    vflash = find_vflash(settings)
    if vflash is None:
        print_step("Error: vflash not found in the toolchain or PATH")
        return 1

    print_step(yellow("Stopping program..."))
    return subprocess.run([vflash, "stop"], cwd=ROOT, check=False).returncode
