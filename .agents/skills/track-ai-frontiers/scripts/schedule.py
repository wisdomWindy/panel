#!/usr/bin/env python3
"""Install or remove a user-level daily refresh task for AI Frontier Radar."""

from __future__ import annotations

import argparse
import json
import os
import plistlib
import platform
import re
import subprocess
import sys
from pathlib import Path
from typing import Any


SKILL_DIR = Path(__file__).resolve().parents[1]
RADAR_SCRIPT = SKILL_DIR / "scripts" / "ai_radar.py"
LABEL = "com.codex.track-ai-frontiers"


def runtime_root() -> Path:
    configured = os.environ.get("AI_FRONTIER_RADAR_HOME")
    if configured:
        return Path(configured).expanduser().resolve()
    parts = SKILL_DIR.parts
    if ".agents" in parts:
        return Path(*parts[: parts.index(".agents")]) / "ai-frontier-radar"
    return Path.cwd() / "ai-frontier-radar"


RUNTIME_ROOT = runtime_root()
DATA_DIR = RUNTIME_ROOT / "data"
SITE_DIR = RUNTIME_ROOT / "site"


def parse_time(value: str) -> tuple[int, int]:
    match = re.fullmatch(r"([01]?\d|2[0-3]):([0-5]\d)", value)
    if not match:
        raise argparse.ArgumentTypeError("time must be HH:MM in 24-hour format")
    return int(match.group(1)), int(match.group(2))


def command() -> list[str]:
    return [
        sys.executable,
        str(RADAR_SCRIPT),
        "update",
        "--data-dir",
        str(DATA_DIR),
        "--site-dir",
        str(SITE_DIR),
    ]


def run(args: list[str], check: bool = True) -> subprocess.CompletedProcess[str]:
    return subprocess.run(args, check=check, capture_output=True, text=True)


def mac_paths() -> tuple[Path, Path, Path]:
    home = Path.home()
    return (
        home / "Library" / "LaunchAgents" / f"{LABEL}.plist",
        DATA_DIR / "logs" / "refresh.log",
        DATA_DIR / "logs" / "refresh-error.log",
    )


def mac_payload(hour: int, minute: int) -> dict[str, Any]:
    plist_path, stdout_path, stderr_path = mac_paths()
    del plist_path
    return {
        "Label": LABEL,
        "ProgramArguments": command(),
        "StartCalendarInterval": {"Hour": hour, "Minute": minute},
        "RunAtLoad": True,
        "StandardOutPath": str(stdout_path),
        "StandardErrorPath": str(stderr_path),
        "ProcessType": "Background",
    }


def install_macos(hour: int, minute: int, dry_run: bool) -> dict[str, Any]:
    plist_path, stdout_path, stderr_path = mac_paths()
    result = {"platform": "macOS", "path": str(plist_path), "time": f"{hour:02d}:{minute:02d}", "command": command()}
    if dry_run:
        result["plist"] = mac_payload(hour, minute)
        return result
    plist_path.parent.mkdir(parents=True, exist_ok=True)
    stdout_path.parent.mkdir(parents=True, exist_ok=True)
    with plist_path.open("wb") as handle:
        plistlib.dump(mac_payload(hour, minute), handle, sort_keys=False)
    domain = f"gui/{os.getuid()}"
    run(["launchctl", "bootout", domain, str(plist_path)], check=False)
    run(["launchctl", "bootstrap", domain, str(plist_path)])
    return result


def uninstall_macos() -> dict[str, Any]:
    plist_path, _, _ = mac_paths()
    domain = f"gui/{os.getuid()}"
    if plist_path.exists():
        run(["launchctl", "bootout", domain, str(plist_path)], check=False)
        plist_path.unlink()
    return {"platform": "macOS", "path": str(plist_path), "installed": False}


def status_macos() -> dict[str, Any]:
    plist_path, _, _ = mac_paths()
    loaded = run(["launchctl", "print", f"gui/{os.getuid()}/{LABEL}"], check=False).returncode == 0
    schedule = None
    if plist_path.exists():
        with plist_path.open("rb") as handle:
            payload = plistlib.load(handle)
        interval = payload.get("StartCalendarInterval", {})
        schedule = f"{int(interval.get('Hour', 0)):02d}:{int(interval.get('Minute', 0)):02d}"
    return {"platform": "macOS", "path": str(plist_path), "installed": plist_path.exists(), "loaded": loaded, "time": schedule}


def linux_paths() -> tuple[Path, Path]:
    base = Path.home() / ".config" / "systemd" / "user"
    return base / "track-ai-frontiers.service", base / "track-ai-frontiers.timer"


def linux_units(hour: int, minute: int) -> tuple[str, str]:
    escaped_command = " ".join(subprocess.list2cmdline([part]) for part in command())
    service = "\n".join([
        "[Unit]", "Description=Refresh AI Frontier Radar", "",
        "[Service]", "Type=oneshot", f"ExecStart={escaped_command}", "",
    ])
    timer = "\n".join([
        "[Unit]", "Description=Daily AI Frontier Radar refresh", "",
        "[Timer]", f"OnCalendar=*-*-* {hour:02d}:{minute:02d}:00", "Persistent=true", "",
        "[Install]", "WantedBy=timers.target", "",
    ])
    return service, timer


def install_linux(hour: int, minute: int, dry_run: bool) -> dict[str, Any]:
    service_path, timer_path = linux_paths()
    service, timer = linux_units(hour, minute)
    result = {"platform": "Linux", "service": str(service_path), "timer": str(timer_path), "time": f"{hour:02d}:{minute:02d}", "command": command()}
    if dry_run:
        result.update({"serviceContent": service, "timerContent": timer})
        return result
    service_path.parent.mkdir(parents=True, exist_ok=True)
    service_path.write_text(service, encoding="utf-8")
    timer_path.write_text(timer, encoding="utf-8")
    run(["systemctl", "--user", "daemon-reload"])
    run(["systemctl", "--user", "enable", "--now", timer_path.name])
    return result


def uninstall_linux() -> dict[str, Any]:
    service_path, timer_path = linux_paths()
    run(["systemctl", "--user", "disable", "--now", timer_path.name], check=False)
    for path in (service_path, timer_path):
        if path.exists():
            path.unlink()
    run(["systemctl", "--user", "daemon-reload"], check=False)
    return {"platform": "Linux", "installed": False, "service": str(service_path), "timer": str(timer_path)}


def status_linux() -> dict[str, Any]:
    service_path, timer_path = linux_paths()
    active = run(["systemctl", "--user", "is-active", timer_path.name], check=False).returncode == 0
    enabled = run(["systemctl", "--user", "is-enabled", timer_path.name], check=False).returncode == 0
    return {"platform": "Linux", "installed": service_path.exists() and timer_path.exists(), "active": active, "enabled": enabled, "service": str(service_path), "timer": str(timer_path)}


def dispatch(action: str, time_value: tuple[int, int], dry_run: bool) -> dict[str, Any]:
    system = platform.system()
    hour, minute = time_value
    if system == "Darwin":
        if action == "install":
            return install_macos(hour, minute, dry_run)
        if action == "uninstall":
            return uninstall_macos()
        return status_macos()
    if system == "Linux":
        if action == "install":
            return install_linux(hour, minute, dry_run)
        if action == "uninstall":
            return uninstall_linux()
        return status_linux()
    raise RuntimeError(f"unsupported platform: {system}; use your OS scheduler to run {' '.join(command())}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=("install", "status", "uninstall"))
    parser.add_argument("--time", type=parse_time, default=(8, 0), metavar="HH:MM")
    parser.add_argument("--dry-run", action="store_true", help="show files without installing them")
    args = parser.parse_args()
    try:
        print(json.dumps(dispatch(args.action, args.time, args.dry_run), ensure_ascii=False, indent=2))
        return 0
    except Exception as exc:
        print(json.dumps({"status": "error", "error": f"{type(exc).__name__}: {exc}"}, ensure_ascii=False), file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
