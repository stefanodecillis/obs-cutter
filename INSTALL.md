# Quick Installation Guide

## Prerequisites

1. **Install FFmpeg** (required for video processing):
   ```bash
   brew install ffmpeg
   ```

2. **Install Rust** (only needed for building):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

## Installation Methods

### Method 1: Homebrew (After Publishing to GitHub)

After setting up a Homebrew tap (see [HOMEBREW-QUICK-START.md](HOMEBREW-QUICK-START.md)):

```bash
brew tap YOURUSERNAME/tools
brew install obs-cutter
```

Homebrew will automatically:
- Install FFmpeg if not already installed
- Put the binary in your PATH
- Handle updates via `brew upgrade`

Note: This requires publishing your project to GitHub first.

### Method 2: Install to System with Cargo

Build and install the binary to your PATH:

```bash
cargo install --path .
```

The binary will be installed to `~/.cargo/bin/obs-cutter`.

### Method 3: Standalone Binary

Build a release binary and copy it manually:

```bash
cargo build --release
sudo cp target/release/obs-cutter /usr/local/bin/
```

### Method 4: Development

For development and testing:

```bash
cargo run -- ./your-video.mov
```

## Usage

After installation, use the tool like this:

```bash
# Basic usage
obs-cutter video.mov

# With options
obs-cutter video.mov --quality high --format mp4

# Get help
obs-cutter --help
```

## Distribution

The compiled binary (`target/release/obs-cutter`) is standalone and can be copied to other machines without needing Rust or Node.js installed. Only FFmpeg is required on the target machine.

### Sharing the Binary

1. Build the release version:
   ```bash
   cargo build --release
   ```

2. The binary is at: `target/release/obs-cutter` (881 KB)

3. Copy this file to another Mac and it will work immediately (as long as FFmpeg is installed)

## Verifying Installation

Check if the installation was successful:

```bash
obs-cutter --version
# Should output: obs-cutter 1.0.0

obs-cutter --help
# Should show the help message
```

## Uninstalling

If you used `cargo install`:
```bash
cargo uninstall obs-cutter
```

If you copied the binary manually:
```bash
sudo rm /usr/local/bin/obs-cutter
```
