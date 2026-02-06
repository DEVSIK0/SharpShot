import json
import re
import subprocess
import sys
from pathlib import Path

VALID_TYPES = {"major", "minor", "patch"}

if len(sys.argv) < 2 or sys.argv[1] not in VALID_TYPES:
    print("[!] Invalid release type. Usage: minor, major, patch", file=sys.stderr)
    sys.exit(1)

release_type = sys.argv[1]
package_path = Path.cwd() / "package.json"

try:
    package_data = json.loads(package_path.read_text(encoding="utf-8"))
    current_version = package_data.get("version", "")
    match = re.fullmatch(r"(\d+)\.(\d+)\.(\d+)", current_version)

    if not match:
        print(
            f"[!] Current version {current_version} is not valid SemVer",
            file=sys.stderr,
        )
        sys.exit(1)

    major, minor, patch = map(int, match.groups())

    if release_type == "major":
        major, minor, patch = major + 1, 0, 0
    elif release_type == "minor":
        minor, patch = minor + 1, 0
    else:
        patch += 1

    new_version = f"{major}.{minor}.{patch}"

    print(f"[*] Bumping version: {current_version} -> {new_version}")
    package_data["version"] = new_version
    package_path.write_text(json.dumps(package_data, indent=2) + "\n", encoding="utf-8")

    print("[*] Updated package.json")
    print("[*] Committing, tagging, and pushing...")

    command = (
        f"git add package.json "
        f'&& git commit -m "release: v{new_version}" '
        f'&& git tag -a v{new_version} -m "v{new_version}" '
        f"&& git push --follow-tags"
    )
    subprocess.run(command, check=True, shell=True)

    print(f"[+] Release complete: v{new_version}")
except Exception as exc:
    print(f"[!] Operation failed: {exc}", file=sys.stderr)
    sys.exit(1)
