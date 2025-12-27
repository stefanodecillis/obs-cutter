# Installation Guide

This guide covers all installation methods for OBS-Cutter.

## Table of Contents

- [GUI Application (Recommended)](#gui-application-recommended)
  - [macOS](#macos-gui)
  - [Windows](#windows-gui)
  - [Linux](#linux-gui)
- [CLI Application](#cli-application)
  - [macOS](#macos-cli)
  - [Windows](#windows-cli)
  - [Linux](#linux-cli)
- [Building from Source](#building-from-source)
- [Troubleshooting](#troubleshooting)

---

## GUI Application (Recommended)

The GUI version includes bundled FFmpeg - no additional dependencies required.

### macOS GUI

#### Download

Download the DMG for your Mac from the [releases page](https://github.com/stefanodecillis/obs-cutter/releases/latest):

- **Apple Silicon (M1/M2/M3/M4)**: `OBS-Cutter-macos-arm64.dmg`
- **Intel Macs**: `OBS-Cutter-macos-x64.dmg`

#### Install

1. Open the downloaded `.dmg` file
2. Drag `OBS-Cutter.app` to the `Applications` folder
3. Eject the disk image

#### First Launch (Important!)

Since OBS-Cutter is not notarized with Apple, macOS will block it on first launch. To open it:

**Method 1: Right-click to Open**
1. Open Finder and go to Applications
2. **Right-click** (or Control-click) on `OBS-Cutter.app`
3. Select **"Open"** from the context menu
4. Click **"Open"** in the dialog that appears
5. The app will now open and be remembered as safe

**Method 2: System Settings**
1. Try to open the app normally (it will be blocked)
2. Open **System Settings** > **Privacy & Security**
3. Scroll down to find the message about OBS-Cutter being blocked
4. Click **"Open Anyway"**
5. Enter your password if prompted

**Method 3: Terminal (Advanced)**
```bash
xattr -cr /Applications/OBS-Cutter.app
```

After the first successful launch, the app will open normally in the future.

---

### Windows GUI

#### Download

Download `obs-cutter-gui-windows-x64.zip` from the [releases page](https://github.com/stefanodecillis/obs-cutter/releases/latest).

#### Install

1. Extract the ZIP file to a folder (e.g., `C:\Program Files\OBS-Cutter`)
2. Run `obs-cutter-gui.exe`

#### First Launch

Windows SmartScreen may block the app:

1. Click **"More info"**
2. Click **"Run anyway"**

Optionally, add the folder to your PATH for command-line access.

---

### Linux GUI

#### Download

Download `obs-cutter-gui-linux-x64.tar.gz` from the [releases page](https://github.com/stefanodecillis/obs-cutter/releases/latest).

#### Install

```bash
# Extract
tar -xzf obs-cutter-gui-linux-x64.tar.gz

# Move to a permanent location
sudo mv obs-cutter /opt/obs-cutter

# Create a symlink (optional)
sudo ln -s /opt/obs-cutter/obs-cutter-gui /usr/local/bin/obs-cutter-gui

# Run
obs-cutter-gui
```

---

## CLI Application

The CLI version requires FFmpeg to be installed separately.

### Prerequisites

Install FFmpeg using your package manager:

**macOS (Homebrew):**
```bash
brew install ffmpeg
```

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install ffmpeg
```

**Fedora:**
```bash
sudo dnf install ffmpeg
```

**Windows (Chocolatey):**
```bash
choco install ffmpeg
```

**Windows (winget):**
```bash
winget install ffmpeg
```

Or download from [ffmpeg.org](https://ffmpeg.org/download.html).

---

### macOS CLI

```bash
# For Apple Silicon (M1/M2/M3/M4)
curl -L https://github.com/stefanodecillis/obs-cutter/releases/latest/download/obs-cutter-cli-macos-aarch64 -o obs-cutter
chmod +x obs-cutter
sudo mv obs-cutter /usr/local/bin/

# For Intel Macs
curl -L https://github.com/stefanodecillis/obs-cutter/releases/latest/download/obs-cutter-cli-macos-x86_64 -o obs-cutter
chmod +x obs-cutter
sudo mv obs-cutter /usr/local/bin/
```

---

### Windows CLI

1. Download `obs-cutter-cli-windows-x86_64.exe` from the [releases page](https://github.com/stefanodecillis/obs-cutter/releases/latest)
2. Rename to `obs-cutter.exe`
3. Move to a folder in your PATH, or add the folder to PATH

---

### Linux CLI

```bash
# For x86_64
curl -L https://github.com/stefanodecillis/obs-cutter/releases/latest/download/obs-cutter-cli-linux-x86_64 -o obs-cutter
chmod +x obs-cutter
sudo mv obs-cutter /usr/local/bin/

# For ARM64
curl -L https://github.com/stefanodecillis/obs-cutter/releases/latest/download/obs-cutter-cli-linux-aarch64 -o obs-cutter
chmod +x obs-cutter
sudo mv obs-cutter /usr/local/bin/
```

---

## Building from Source

### Prerequisites

1. **Rust** (1.70 or later)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **FFmpeg** (for CLI) - see [Prerequisites](#prerequisites) above

3. **Platform-specific dependencies:**

   **Linux (for GUI):**
   ```bash
   sudo apt-get install -y \
       libgtk-3-dev \
       libwebkit2gtk-4.1-dev \
       libayatana-appindicator3-dev \
       librsvg2-dev \
       libxdo-dev
   ```

### Build CLI

```bash
git clone https://github.com/stefanodecillis/obs-cutter.git
cd obs-cutter

# Build release binary
cargo build --release

# Binary is at: target/release/obs-cutter
```

### Build GUI

```bash
git clone https://github.com/stefanodecillis/obs-cutter.git
cd obs-cutter

# Build with GUI feature
cargo build --release --features gui

# Binary is at: target/release/obs-cutter-gui
```

### Install from Source

```bash
# Install CLI
cargo install --path .

# The binary will be at ~/.cargo/bin/obs-cutter
```

---

## Troubleshooting

### macOS: "App is damaged and can't be opened"

This happens when the app's quarantine attribute is set. Fix it with:

```bash
xattr -cr /Applications/OBS-Cutter.app
```

Then try opening the app again.

### macOS: "App can't be opened because it is from an unidentified developer"

Right-click the app and select "Open", then click "Open" in the dialog.

### Windows: "Windows protected your PC"

Click "More info" then "Run anyway".

### Linux: "Permission denied"

Make the binary executable:

```bash
chmod +x obs-cutter
```

### "FFmpeg is not installed" (CLI only)

Verify FFmpeg is installed and in your PATH:

```bash
ffmpeg -version
```

If not found, install it using the instructions in [Prerequisites](#prerequisites).

### Binary not found after cargo install

Add `~/.cargo/bin` to your PATH. Add this to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.):

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

---

## Uninstall

### macOS GUI
1. Drag `OBS-Cutter.app` from Applications to Trash
2. Empty Trash

### Windows GUI
Delete the extracted folder.

### Linux GUI
```bash
sudo rm -rf /opt/obs-cutter
sudo rm /usr/local/bin/obs-cutter-gui
```

### CLI (installed via cargo)
```bash
cargo uninstall obs-cutter
```

### CLI (manual install)
```bash
sudo rm /usr/local/bin/obs-cutter
```
