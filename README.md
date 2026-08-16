# 📦 cpm — c00l-P4CK4G3-M4N4G3R

> A fast, unified CLI package manager wrapper for **APT**, **Flatpak**, and **Snap**.

---

## ⚡ System Dependencies

`cpm` delegates commands to your system's package managers. Ensure you have the underlying engines enabled:

- **Linux Operating System** (Ubuntu, Debian, Linux Mint, etc.)
- **APT** (`apt`, `apt-cache`) — Default on Debian-based distros.
- **Flatpak** (`flatpak`) — Optional, required for Flatpak engine searches.
- **Snap** (`snapd`) — Optional, required for Snap engine searches. *(Note: Linux Mint users must remove `/etc/apt/preferences.d/nosnap.pref` to install `snapd`)*.
- **Rust Toolchain** — Required **only** if building from source.

---

## 🚀 Installation

### Option 1: Pre-built Release (ZIP)

Download `cpm-release.zip`, extract it, and run the installation script:

```bash
unzip cpm-release.zip
cd cpm-release
chmod +x install.sh
sudo ./install.sh
```

### Option 2: Build from Source

Compile the binary directly using `cargo`:

```bash
git clone https://github.com/your-username/cpm.git
cd cpm
cargo build --release
sudo cp target/release/cpm /usr/local/bin/cpm
```

---

## 🛠️ Command Reference

```bash
cpm <command> [package]
```

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

To remove `cpm` from your binary path:

```bash
sudo rm /usr/local/bin/cpm
```
