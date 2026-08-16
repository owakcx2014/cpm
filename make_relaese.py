#!/usr/bin/env python3
import shutil
import subprocess
import zipfile
from pathlib import Path

# chr(96) generates backticks dynamically so markdown fences never break rendering
BT = chr(96) * 3
B = chr(96)

README_MD = f"""# 📦 cpm — c00l-P4CK4G3-M4N4G3R

> A fast, unified CLI package manager wrapper for **APT**, **Flatpak**, and **Snap**.

---

## ⚡ System Dependencies

{B}cpm{B} delegates commands to your system's package managers. Ensure you have the underlying engines enabled:

- **Linux Operating System** (Ubuntu, Debian, Linux Mint, etc.)
- **APT** ({B}apt{B}, {B}apt-cache{B}) — Default on Debian-based distros.
- **Flatpak** ({B}flatpak{B}) — Optional, required for Flatpak engine searches.
- **Snap** ({B}snapd{B}) — Optional, required for Snap engine searches. *(Note: Linux Mint users must remove {B}/etc/apt/preferences.d/nosnap.pref{B} to install {B}snapd{B})*.
- **Rust Toolchain** — Required **only** if building from source.

---

## 🚀 Installation

### Option 1: Pre-built Release (ZIP)

Download {B}cpm-release.zip{B}, extract it, and run the installation script:

{BT}bash
unzip cpm-release.zip
cd cpm-release
chmod +x install.sh
sudo ./install.sh
{BT}

### Option 2: Build from Source

Compile the binary directly using {B}cargo{B}:

{BT}bash
git clone https://github.com/your-username/cpm.git
cd cpm
cargo build --release
sudo cp target/release/cpm /usr/local/bin/cpm
{BT}

---

## 🛠️ Command Reference

{BT}bash
cpm <command> [package]
{BT}

| Command | Action | Example |
| :--- | :--- | :--- |
| `install` | Interactively search and select engine (APT/Flatpak/Snap) to install | `cpm install vlc` |
| `remove` | Uninstall a package using APT | `cpm remove gimp` |
| `update` | Sequentially update system packages across APT, Flatpak, and Snap | `cpm update` |
| `clean` | Autoremove unused packages across all three engines | `cpm clean` |
| `cache` | Clean APT archive files and repair Flatpak | `cpm cache` |
| `search` | Query package availability concurrently across APT, Flatpak, and Snap | `cpm search firefox` |

---

## 🗑️ Uninstallation

To remove {B}cpm{B} from your binary path:

{BT}bash
sudo rm /usr/local/bin/cpm
{BT}
"""

INSTALL_SH = """#!/bin/bash
set -e

echo "--- Installing cpm (c00l-P4CK4G3-M4N4G3R) ---"

if [ ! -f "cpm" ]; then
    echo "Error: 'cpm' binary not found in current directory."
    exit 1
fi

chmod +x cpm
sudo cp cpm /usr/local/bin/cpm

echo "Successfully installed! Run 'cpm' in your terminal."
"""

def main():
    print("🚀 Building Rust binary...")
    subprocess.run(["cargo", "build", "--release"], check=True)

    release_dir = Path("cpm-release")
    if release_dir.exists():
        shutil.rmtree(release_dir)
    release_dir.mkdir()

    print("📄 Creating README.md and install.sh...")
    (release_dir / "README.md").write_text(README_MD)

    install_script = release_dir / "install.sh"
    install_script.write_text(INSTALL_SH)
    install_script.chmod(0o755)

    print("📦 Copying compiled binary...")
    shutil.copy("target/release/cpm", release_dir / "cpm")

    print("🤐 Creating cpm-release.zip...")
    zip_path = Path("cpm-release.zip")
    with zipfile.ZipFile(zip_path, "w", zipfile.ZIP_DEFLATED) as zipf:
        for file in release_dir.glob("*"):
            zipf.write(file, arcname=Path("cpm-release") / file.name)

    print("✅ Done! Created 'cpm-release.zip' successfully.")

if __name__ == "__main__":
    main()
