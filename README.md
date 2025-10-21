# OBS-Cutter

[![CI](https://github.com/stefanodecillis/obs-cutter/workflows/CI/badge.svg)](https://github.com/stefanodecillis/obs-cutter/actions)
[![Release](https://github.com/stefanodecillis/obs-cutter/workflows/Release/badge.svg)](https://github.com/stefanodecillis/obs-cutter/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A fast, cross-platform command-line tool to split ultra-wide 32:9 OBS recordings (3840x1080) into two separate 16:9 videos (1920x1080 each).

Perfect for content creators who record with dual monitors in OBS and need to split the output into separate videos for editing or streaming.

## Features

- **Fast & Efficient**: Uses FFmpeg's optimized crop filter for rapid video processing
- **Quality Presets**: Choose from lossless, high, or medium quality settings
- **Cross-Platform**: Works on macOS, Linux, and Windows
- **Simple Interface**: Straightforward CLI with sensible defaults
- **Progress Indicators**: Real-time feedback during processing
- **Flexible Output**: Specify output format, quality, and directory
- **Preserves Audio**: Automatically copies audio tracks without re-encoding

## How It Works

OBS-Cutter uses FFmpeg's crop filter to extract the left and right halves of your ultra-wide recording:

1. **Left video**: Crops from x=0 to x=1920 (left half of the screen)
2. **Right video**: Crops from x=1920 to x=3840 (right half of the screen)

Both output videos maintain the original 1920x1080 resolution and preserve all audio tracks. The tool uses efficient encoding settings based on your quality preference to ensure optimal file sizes without compromising visual quality.

## Prerequisites

**FFmpeg** is required for video processing. Install it using your package manager:

**macOS (Homebrew):**
```bash
brew install ffmpeg
```

**Ubuntu/Debian:**
```bash
sudo apt-get install ffmpeg
```

**Windows (Chocolatey):**
```bash
choco install ffmpeg
```

Or download from the [FFmpeg official site](https://ffmpeg.org/download.html).

## Installation

### Option 1: Download Pre-built Binary (Recommended)

Download the latest release for your platform from the [releases page](https://github.com/stefanodecillis/obs-cutter/releases):

**macOS:**
```bash
# For Apple Silicon (M1/M2/M3)
curl -L https://github.com/stefanodecillis/obs-cutter/releases/latest/download/obs-cutter-macos-aarch64 -o obs-cutter
chmod +x obs-cutter
sudo mv obs-cutter /usr/local/bin/

# For Intel Macs
curl -L https://github.com/stefanodecillis/obs-cutter/releases/latest/download/obs-cutter-macos-x86_64 -o obs-cutter
chmod +x obs-cutter
sudo mv obs-cutter /usr/local/bin/
```

**Linux:**
```bash
# For x86_64
curl -L https://github.com/stefanodecillis/obs-cutter/releases/latest/download/obs-cutter-linux-x86_64 -o obs-cutter
chmod +x obs-cutter
sudo mv obs-cutter /usr/local/bin/

# For ARM64
curl -L https://github.com/stefanodecillis/obs-cutter/releases/latest/download/obs-cutter-linux-aarch64 -o obs-cutter
chmod +x obs-cutter
sudo mv obs-cutter /usr/local/bin/
```

**Windows:**
Download `obs-cutter-windows-x86_64.exe` from the [releases page](https://github.com/stefanodecillis/obs-cutter/releases/latest) and add it to your PATH.

### Option 2: Install from Source

If you have Rust installed, you can build and install from source:

```bash
# Install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone the repository
git clone https://github.com/stefanodecillis/obs-cutter.git
cd obs-cutter

# Install
cargo install --path .
```

This will install the binary to `~/.cargo/bin/obs-cutter`, which should be in your PATH.

### Option 3: Build Release Binary

To build a standalone binary without installing:

```bash
cargo build --release
```

The binary will be available at `target/release/obs-cutter`.

## Usage

### Basic Usage

Split a video into left and right parts:

```bash
obs-cutter video.mov
```

This creates:
- `video-left.mov` - Left half (1920x1080)
- `video-right.mov` - Right half (1920x1080)

### Options

```bash
obs-cutter <VIDEO> [OPTIONS]
```

**Options:**

- `-f, --format <FORMAT>` - Output format (mp4, mov, mkv, etc.). Defaults to input format.
- `-q, --quality <QUALITY>` - Quality preset: `lossless` (default), `high`, or `medium`
- `-o, --output <DIR>` - Output directory. Defaults to input file directory.
- `-h, --help` - Display help information
- `-V, --version` - Display version information

### Examples

**Convert to MP4 format:**
```bash
obs-cutter recording.mov --format mp4
```

**Use high quality preset:**
```bash
obs-cutter recording.mov --quality high
```

**Specify output directory:**
```bash
obs-cutter recording.mov --output ./split-videos/
```

**Combine multiple options:**
```bash
obs-cutter recording.mov --format mp4 --quality high --output ./output/
```

## Quality Presets

### Lossless (Default)
- Uses H.264 lossless encoding (CRF 0)
- Maximum quality preservation
- Larger file sizes
- Best for archival or further editing
- **Use when**: Quality is paramount

### High
- CRF 18 with slow preset
- Visually indistinguishable from lossless for most content
- Moderate file sizes (typically 50-70% smaller than lossless)
- Excellent balance for most use cases
- **Use when**: You need great quality with reasonable file sizes

### Medium
- CRF 23 with medium preset
- Good visual quality
- Smaller file sizes (typically 70-85% smaller than lossless)
- Faster encoding
- **Use when**: File size is a concern or for web sharing

## Platform Support

- **macOS**: Fully tested and supported (Intel and Apple Silicon)
- **Linux**: Fully tested and supported (x86_64 and ARM64)
- **Windows**: Supported but not extensively tested. Please report any issues!

## Troubleshooting

### "FFmpeg is not installed"

Make sure FFmpeg is installed and available in your PATH:

```bash
ffmpeg -version
```

If not installed, see [Prerequisites](#prerequisites).

### Video dimensions warning

The tool expects 3840x1080 videos (32:9 aspect ratio). If your video has different dimensions, it will still attempt to split it at the midpoint, but the output may not be as expected.

### Binary not found after installation

Make sure `~/.cargo/bin` is in your PATH. Add this to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.):

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

### Permission denied (Unix/Linux/macOS)

If you get "permission denied" when running the binary:

```bash
chmod +x obs-cutter
```

## Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Running

```bash
# Run with cargo
cargo run -- video.mov

# Run with debug output
RUST_LOG=debug cargo run -- video.mov
```

### Testing

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Powered by [FFmpeg](https://ffmpeg.org/)
- CLI built with [clap](https://github.com/clap-rs/clap)

## Support

If you encounter any issues or have questions:
- Open an issue on [GitHub](https://github.com/stefanodecillis/obs-cutter/issues)
- Check existing issues for solutions

---

Made with ❤️ for content creators
