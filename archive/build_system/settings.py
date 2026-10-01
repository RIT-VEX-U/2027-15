from __future__ import annotations

import json
import re
import tomllib
from dataclasses import dataclass

from .util import PROJECT_FILE, SETTINGS_FILE, write_if_changed

VALID_ICONS = {
    "alien",
    "alien-in-ufo",
    "clawbot",
    "code-file",
    "cool-x",
    "cup-and-ball",
    "cup-in-field",
    "matlab",
    "pizza",
    "planets",
    "power-button",
    "pros",
    "question-mark",
    "robot",
    "robot-mesh",
    "robot-mesh-blockly",
    "robot-mesh-cpp",
    "robot-mesh-flowol",
    "robot-mesh-js",
    "robot-mesh-py",
    "vex-coding-studio",
    "vexcode-blocks",
    "vexcode-brackets",
    "vexcode-cpp",
    "vexcode-python",
}
VALID_STRATEGIES = {"full", "differential", "ram"}
VALID_TERMINAL_MODES = {"interactive", "none", "watch"}


@dataclass(frozen=True)
class ProjectSettings:
    name: str
    description: str
    platform: str
    icon: str
    sdk: str
    slot: int
    strategy: str
    terminal: str


def _table(data: dict, name: str) -> dict:
    value = data.get(name, {})
    if not isinstance(value, dict):
        raise SystemExit(f"project.toml: [{name}] must be a table")
    return value


def validate_project_name(name: object) -> str:
    if not isinstance(name, str) or not name or any(
        char not in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-"
        for char in name
    ):
        raise SystemExit(
            "project.name must contain only letters, numbers, underscores, and dashes"
        )
    return name

# read in project.toml
def load_settings() -> ProjectSettings:
    if not PROJECT_FILE.is_file():
        raise SystemExit(f"Project settings not found: {PROJECT_FILE}")
    try:
        data = tomllib.loads(PROJECT_FILE.read_text(encoding="utf-8"))
    except tomllib.TOMLDecodeError as error:
        raise SystemExit(f"Invalid project.toml: {error}") from error

    schema = data.get("schema", 1)
    if schema != 1:
        raise SystemExit(f"project.toml: unsupported schema {schema!r}; expected 1")
    project = _table(data, "project")
    build = _table(data, "build")
    upload = _table(data, "upload")
    run = _table(data, "run")

    name = project.get("name", "VexProject")
    description = project.get("description", "")
    platform = project.get("platform", "V5")
    icon = project.get("icon", "question-mark")
    sdk = build.get("sdk", "latest")
    slot = upload.get("slot", 1)
    strategy = upload.get("strategy", "full")
    terminal = run.get("terminal", "interactive")

    try:
        name = validate_project_name(name)
    except SystemExit as error:
        raise SystemExit(f"project.toml: {error}") from error
    if not isinstance(description, str) or "\n" in description or "\r" in description:
        raise SystemExit("project.toml: project.description must be a single-line string")
    if platform != "V5":
        raise SystemExit("project.toml: only project.platform = \"V5\" is currently supported")
    if not isinstance(icon, str) or icon not in VALID_ICONS:
        raise SystemExit(f"project.toml: unsupported project.icon {icon!r}")
    if not isinstance(sdk, str) or (
        sdk != "latest"
        and re.fullmatch(r"V5_\d{8}_\d{2}_\d{2}_\d{2}", sdk) is None
    ):
        raise SystemExit("project.toml: build.sdk must be \"latest\" or a V5 SDK directory name")
    if not isinstance(slot, int) or isinstance(slot, bool) or not 1 <= slot <= 8:
        raise SystemExit("project.toml: upload.slot must be an integer from 1 through 8")
    if not isinstance(strategy, str) or strategy not in VALID_STRATEGIES:
        raise SystemExit('project.toml: upload.strategy must be "full", "differential", or "ram"')
    if not isinstance(terminal, str) or terminal not in VALID_TERMINAL_MODES:
        raise SystemExit(
            'project.toml: run.terminal must be "interactive", "watch", or "none"'
        )

    return ProjectSettings(name, description, platform, icon, sdk, slot, strategy, terminal)

# update one setting keeping comments etc.
def update_setting(section: str, key: str, value: str | int) -> None:
    content = PROJECT_FILE.read_text(encoding="utf-8")
    rendered = json.dumps(value) if isinstance(value, str) else str(value)
    section_match = re.search(rf"(?m)^\[{re.escape(section)}\]\s*(?:#.*)?$", content)
    if section_match is None:
        raise SystemExit(f"project.toml: missing [{section}] table")
    remaining = content[section_match.end():]
    next_section = re.search(r"(?m)^\[", remaining)
    section_end = section_match.end() + (
        next_section.start() if next_section else len(remaining)
    )
    body = content[section_match.end() : section_end]
    key_match = re.search(rf"(?m)^(\s*{re.escape(key)}\s*=\s*)([^#\n]*?)(\s*(?:#.*)?)$", body)
    if key_match is None:
        raise SystemExit(f"project.toml: missing {section}.{key}")
    start = section_match.end() + key_match.start(2)
    end = section_match.end() + key_match.end(2)
    write_if_changed(PROJECT_FILE, content[:start] + rendered + content[end:])

# modify vex_project_settings.json to match toml
def sync_vscode(settings: ProjectSettings, sdk_version: str) -> None:
    data = json.loads(SETTINGS_FILE.read_text(encoding="utf-8")) if SETTINGS_FILE.is_file() else {}
    project = data.setdefault("project", {})
    project.update({
        "name": settings.name,
        "description": settings.description,
        "platform": settings.platform,
        "slot": settings.slot,
        "sdkVersion": sdk_version,
    })
    write_if_changed(SETTINGS_FILE, json.dumps(data, indent="\t") + "\n")
