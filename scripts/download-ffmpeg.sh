#!/bin/bash
# Download FFmpeg binaries for bundling with obs-cutter
# Usage: ./download-ffmpeg.sh <platform> <output_dir>
# Platforms: macos-arm64, macos-x64, linux-x64, linux-arm64, windows-x64

set -e

PLATFORM="${1:-}"
OUTPUT_DIR="${2:-./ffmpeg-bin}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

usage() {
    echo "Usage: $0 <platform> [output_dir]"
    echo ""
    echo "Platforms:"
    echo "  macos-arm64    macOS Apple Silicon"
    echo "  macos-x64      macOS Intel"
    echo "  linux-x64      Linux x86_64"
    echo "  linux-arm64    Linux ARM64"
    echo "  windows-x64    Windows x86_64"
    echo ""
    echo "Output directory defaults to ./ffmpeg-bin"
    exit 1
}

if [ -z "$PLATFORM" ]; then
    usage
fi

mkdir -p "$OUTPUT_DIR"

case "$PLATFORM" in
    macos-arm64)
        log_info "Downloading FFmpeg for macOS ARM64..."

        # Use ffmpeg-static builds from GitHub (osxcross builds)
        FFMPEG_URL="https://github.com/eugeneware/ffmpeg-static/releases/download/b6.0/ffmpeg-darwin-arm64"
        FFPROBE_URL="https://github.com/eugeneware/ffmpeg-static/releases/download/b6.0/ffprobe-darwin-arm64"

        curl -L "$FFMPEG_URL" -o "$OUTPUT_DIR/ffmpeg"
        curl -L "$FFPROBE_URL" -o "$OUTPUT_DIR/ffprobe"

        chmod +x "$OUTPUT_DIR/ffmpeg" "$OUTPUT_DIR/ffprobe"
        ;;

    macos-x64)
        log_info "Downloading FFmpeg for macOS x64..."

        FFMPEG_URL="https://github.com/eugeneware/ffmpeg-static/releases/download/b6.0/ffmpeg-darwin-x64"
        FFPROBE_URL="https://github.com/eugeneware/ffmpeg-static/releases/download/b6.0/ffprobe-darwin-x64"

        curl -L "$FFMPEG_URL" -o "$OUTPUT_DIR/ffmpeg"
        curl -L "$FFPROBE_URL" -o "$OUTPUT_DIR/ffprobe"

        chmod +x "$OUTPUT_DIR/ffmpeg" "$OUTPUT_DIR/ffprobe"
        ;;

    linux-x64)
        log_info "Downloading FFmpeg for Linux x64..."

        # Use BtbN's reliable GitHub releases
        FFMPEG_URL="https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-linux64-gpl.tar.xz"

        curl -L "$FFMPEG_URL" -o "$OUTPUT_DIR/ffmpeg.tar.xz"
        tar -xf "$OUTPUT_DIR/ffmpeg.tar.xz" -C "$OUTPUT_DIR" --strip-components=1
        rm "$OUTPUT_DIR/ffmpeg.tar.xz"

        # Move binaries from bin folder and clean up
        if [ -d "$OUTPUT_DIR/bin" ]; then
            mv "$OUTPUT_DIR/bin/ffmpeg" "$OUTPUT_DIR/"
            mv "$OUTPUT_DIR/bin/ffprobe" "$OUTPUT_DIR/"
            rm -rf "$OUTPUT_DIR/bin" "$OUTPUT_DIR/doc" "$OUTPUT_DIR/lib" "$OUTPUT_DIR/include" 2>/dev/null || true
        fi

        # Keep only ffmpeg and ffprobe
        find "$OUTPUT_DIR" -type f ! \( -name "ffmpeg" -o -name "ffprobe" \) -delete 2>/dev/null || true
        find "$OUTPUT_DIR" -type d -empty -delete 2>/dev/null || true

        chmod +x "$OUTPUT_DIR/ffmpeg" "$OUTPUT_DIR/ffprobe"
        ;;

    linux-arm64)
        log_info "Downloading FFmpeg for Linux ARM64..."

        # Use BtbN's reliable GitHub releases
        FFMPEG_URL="https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-linuxarm64-gpl.tar.xz"

        curl -L "$FFMPEG_URL" -o "$OUTPUT_DIR/ffmpeg.tar.xz"
        tar -xf "$OUTPUT_DIR/ffmpeg.tar.xz" -C "$OUTPUT_DIR" --strip-components=1
        rm "$OUTPUT_DIR/ffmpeg.tar.xz"

        # Move binaries from bin folder and clean up
        if [ -d "$OUTPUT_DIR/bin" ]; then
            mv "$OUTPUT_DIR/bin/ffmpeg" "$OUTPUT_DIR/"
            mv "$OUTPUT_DIR/bin/ffprobe" "$OUTPUT_DIR/"
            rm -rf "$OUTPUT_DIR/bin" "$OUTPUT_DIR/doc" "$OUTPUT_DIR/lib" "$OUTPUT_DIR/include" 2>/dev/null || true
        fi

        # Keep only ffmpeg and ffprobe
        find "$OUTPUT_DIR" -type f ! \( -name "ffmpeg" -o -name "ffprobe" \) -delete 2>/dev/null || true
        find "$OUTPUT_DIR" -type d -empty -delete 2>/dev/null || true

        chmod +x "$OUTPUT_DIR/ffmpeg" "$OUTPUT_DIR/ffprobe"
        ;;

    windows-x64)
        log_info "Downloading FFmpeg for Windows x64..."

        # Use BtbN's reliable GitHub releases
        FFMPEG_URL="https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip"

        curl -L "$FFMPEG_URL" -o "$OUTPUT_DIR/ffmpeg.zip"
        unzip -o "$OUTPUT_DIR/ffmpeg.zip" -d "$OUTPUT_DIR"

        # Find and move the binaries
        FFMPEG_DIR=$(find "$OUTPUT_DIR" -type d -name "ffmpeg-*" | head -1)
        if [ -n "$FFMPEG_DIR" ]; then
            mv "$FFMPEG_DIR/bin/ffmpeg.exe" "$OUTPUT_DIR/"
            mv "$FFMPEG_DIR/bin/ffprobe.exe" "$OUTPUT_DIR/"
            rm -rf "$FFMPEG_DIR"
        fi

        rm "$OUTPUT_DIR/ffmpeg.zip"
        ;;

    *)
        log_error "Unknown platform: $PLATFORM"
        usage
        ;;
esac

log_info "FFmpeg binaries downloaded to $OUTPUT_DIR"
ls -la "$OUTPUT_DIR"
