import json
import os
import platform
import shutil
import urllib.request
from dataclasses import dataclass
from pathlib import Path

VEX_DIR = Path.home() / ".vex" / "vexcode"
OS_MAP = {"Windows": "Windows-x86_64", "Darwin": "Darwin-universal", "Linux": "Linux-x86_64"}
SDK_MANIFEST = "https://content.vexrobotics.com/vexos/public/V5/vscode/sdk/cpp/manifest.json"
LSCRIPT_URL = "https://gist.githubusercontent.com/PascalSkylake/bf9e3d4b1dbdbbae2b40add09e99ecdb/raw/lscript_llvm.ld"
LIBVEXPATCHER_URL = "https://github.com/RIT-VEX-U/ForkTemplate/releases/download/22.1/libvexpatcher.a"
ATFE_VERSION = "22.1.0"
ATFE_RELEASE = "22.1"
CLANG_VERSION = ATFE_VERSION.split(".", maxsplit=1)[0]
HOST = platform.system()

if HOST not in OS_MAP:
    raise SystemExit(f"Unsupported host platform: {HOST}")

ATFE_NAME = f"ATfE-{ATFE_VERSION}-{OS_MAP[HOST]}"
ATFE_URL = f"https://github.com/RIT-VEX-U/ForkTemplate/releases/download/{ATFE_RELEASE}/{ATFE_NAME}.tar.xz"


@dataclass
class Toolchain:
    sdk_path: Path
    toolchain_path: Path
    clang: Path
    linker: Path
    objcopy: Path
    objdump: Path
    size: Path
    make: Path
    vflash: Path
    resource_dir: Path
    cxx_include_dir: Path
    newlib_include_dir: Path
    newlib_lib_dir: Path
    linker_script: Path

# makes a request of a url
def request(url: str) -> urllib.request.Request:
    return urllib.request.Request(url, headers={"User-Agent": "build"})

# downloads a file from a url
def download(url: str, dest: Path) -> None:
    print(f"Downloading {url}...", flush=True)
    with urllib.request.urlopen(request(url)) as response, dest.open("wb") as output:
        shutil.copyfileobj(response, output)

# downloads latest VEX sdk from their website
def get_sdk(version: str = "latest") -> Path:
    if version != "latest" and (VEX_DIR / version / "vexv5").is_dir():
        sdk_path = VEX_DIR / version / "vexv5"
    elif version == "latest" and (sdks := sorted(VEX_DIR.glob("V5_*/vexv5"), reverse=True)):
        sdk_path = sdks[0]
    else:
        VEX_DIR.mkdir(parents=True, exist_ok=True)
        if version == "latest":
            with urllib.request.urlopen(request(SDK_MANIFEST)) as response:
                version = json.load(response)["latest"]

        sdk_path = VEX_DIR / version / "vexv5"
        zip_path = VEX_DIR / f"{version}.zip"

        download(f"https://content.vexrobotics.com/vexos/public/V5/vscode/sdk/cpp/{version}.zip", zip_path)
        shutil.unpack_archive(zip_path, VEX_DIR)
        zip_path.unlink()

    linker_script = sdk_path / "lscript_llvm.ld"
    if not linker_script.is_file():
        download(LSCRIPT_URL, linker_script)

    vexpatcher_library = sdk_path / "libvexpatcher.a"
    if not vexpatcher_library.is_file():
        download(LIBVEXPATCHER_URL, vexpatcher_library)

    return sdk_path

# gets a toolchain object with all the tool paths, downloads if not already installed
def discover_toolchain(sdk_version: str = "latest") -> Toolchain:
    sdk_path = get_sdk(sdk_version)
    toolchain_path = VEX_DIR / ATFE_NAME
    exe = ".exe" if os.name == "nt" else ""
    vflash = toolchain_path / "tools" / "vflash" / f"vflash{exe}"

    if toolchain_path.exists() and not vflash.is_file():
        shutil.rmtree(toolchain_path)

    if not toolchain_path.exists():
        VEX_DIR.mkdir(parents=True, exist_ok=True)
        tar_path = VEX_DIR / f"{ATFE_NAME}.tar.xz"
        download(ATFE_URL, tar_path)
        shutil.unpack_archive(tar_path, VEX_DIR)
        tar_path.unlink()

    bin_dir = toolchain_path / "bin"

    return Toolchain(
        sdk_path=sdk_path,
        toolchain_path=toolchain_path,
        clang=bin_dir / f"clang{exe}",
        linker=bin_dir / f"ld.lld{exe}",
        objcopy=bin_dir / f"llvm-objcopy{exe}",
        objdump=bin_dir / f"llvm-objdump{exe}",
        size=bin_dir / f"llvm-size{exe}",
        make=toolchain_path / "tools" / f"make{exe}",
        vflash=vflash,
        resource_dir=toolchain_path / f"lib/clang/{CLANG_VERSION}",
        cxx_include_dir=toolchain_path / "lib/clang-runtimes/newlib/arm-none-eabi/include/c++/v1",
        newlib_include_dir=toolchain_path / "lib/clang-runtimes/newlib/arm-none-eabi/include",
        newlib_lib_dir=toolchain_path / "lib/clang-runtimes/newlib/arm-none-eabi/armv7a_soft_vfpv3_d16_unaligned/lib",
        linker_script=sdk_path / "lscript_llvm.ld",
    )
