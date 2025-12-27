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

## Installation

Download the latest release from the [releases page](https://github.com/stefanodecillis/obs-cutter/releases/latest).

| Platform | GUI (Recommended) | CLI |
|----------|-------------------|-----|
| macOS (Apple Silicon) | `OBS-Cutter-macos-arm64.dmg` | `obs-cutter-cli-macos-aarch64` |
| macOS (Intel) | `OBS-Cutter-macos-x64.dmg` | `obs-cutter-cli-macos-x86_64` |
| Linux (x64) | `obs-cutter-gui-linux-x64.tar.gz` | `obs-cutter-cli-linux-x86_64` |
| Linux (ARM64) | - | `obs-cutter-cli-linux-aarch64` |
| Windows (x64) | `obs-cutter-gui-windows-x64.zip` | `obs-cutter-cli-windows-x86_64.exe` |

- **GUI version**: Includes bundled FFmpeg - no additional dependencies needed
- **CLI version**: Requires FFmpeg to be installed separately

**See [INSTALL.md](INSTALL.md) for detailed installation instructions**, including:
- Step-by-step setup for each platform
- How to handle macOS security warnings
- Building from source
- Troubleshooting common issues

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
