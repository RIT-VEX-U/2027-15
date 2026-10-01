from __future__ import annotations

import argparse

from .build import build, clean, rebuild, run_program, stop, upload


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command", required=True)

    def add_build_options(subparser: argparse.ArgumentParser) -> None:
        subparser.add_argument("-p", "--parallel", type=int, default=None, help="number of parallel compile jobs")
        subparser.add_argument("-q", "--quiet", action="store_true", help="suppress compiler warnings")
        subparser.add_argument("-n", "--name", dest="project_name", help="change project name")

    add_build_options(subparsers.add_parser("build", help="build the project"))
    add_build_options(subparsers.add_parser("rebuild", help="clean and build the project"))
    upload_parser = subparsers.add_parser("upload", help="build and upload the project")
    add_build_options(upload_parser)
    upload_parser.add_argument("-s", "--slot", type=int, choices=range(1, 9), help="VEX brain program slot")
    run_parser = subparsers.add_parser("run", help="run a program on the VEX brain")
    run_parser.add_argument("slot", nargs="?", type=int, choices=range(1, 9), help="VEX brain program slot")
    subparsers.add_parser("stop", help="stop the running program")
    subparsers.add_parser("clean", help="nuke build directory")

    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.command == "clean":
        return clean()
    if args.command == "rebuild":
        return rebuild(args)
    if args.command == "upload":
        return upload(args)
    if args.command == "run":
        return run_program(args)
    if args.command == "stop":
        return stop()
    return build(args)
