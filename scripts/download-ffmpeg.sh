#!/bin/bash
# Download FFmpeg binaries for bundling with obs-cutter
# Usage: ./download-ffmpeg.sh <platform> <output_dir>
# Platforms: macos-arm64, macos-x64, linux-x64, linux-arm64, windows-x64

set -e

PLATFORM="${1:-}"
OUTPUT_DIR="${2:-./ffmpeg-bin}"

# FFmpeg version to download
FFMPEG_VERSION="7.0"

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
    macos-arm64|macos-x64)
        # Use evermeet.cx builds for macOS (static builds)
        log_info "Downloading FFmpeg for macOS..."

        FFMPEG_URL="https://evermeet.cx/ffmpeg/ffmpeg-${FFMPEG_VERSION}.zip"
        FFPROBE_URL="https://evermeet.cx/ffmpeg/ffprobe-${FFMPEG_VERSION}.zip"

        # Download and extract ffmpeg
        curl -L "$FFMPEG_URL" -o "$OUTPUT_DIR/ffmpeg.zip" || {
            log_warn "evermeet.cx download failed, trying alternative..."
            # Alternative: use homebrew's formula info to get the bottle
            FFMPEG_URL="https://github.com/eugeneware/ffmpeg-static/releases/download/b${FFMPEG_VERSION}/ffmpeg-darwin-arm64"
            if [ "$PLATFORM" = "macos-x64" ]; then
                FFMPEG_URL="https://github.com/eugeneware/ffmpeg-static/releases/download/b${FFMPEG_VERSION}/ffmpeg-darwin-x64"
            fi
            curl -L "$FFMPEG_URL" -o "$OUTPUT_DIR/ffmpeg"
            chmod +x "$OUTPUT_DIR/ffmpeg"
        }

        if [ -f "$OUTPUT_DIR/ffmpeg.zip" ]; then
            unzip -o "$OUTPUT_DIR/ffmpeg.zip" -d "$OUTPUT_DIR"
            rm "$OUTPUT_DIR/ffmpeg.zip"
        fi

        # Download and extract ffprobe
        curl -L "$FFPROBE_URL" -o "$OUTPUT_DIR/ffprobe.zip" 2>/dev/null || {
            FFPROBE_URL="https://github.com/eugeneware/ffmpeg-static/releases/download/b${FFMPEG_VERSION}/ffprobe-darwin-arm64"
            if [ "$PLATFORM" = "macos-x64" ]; then
                FFPROBE_URL="https://github.com/eugeneware/ffmpeg-static/releases/download/b${FFMPEG_VERSION}/ffprobe-darwin-x64"
            fi
            curl -L "$FFPROBE_URL" -o "$OUTPUT_DIR/ffprobe"
            chmod +x "$OUTPUT_DIR/ffprobe"
        }

        if [ -f "$OUTPUT_DIR/ffprobe.zip" ]; then
            unzip -o "$OUTPUT_DIR/ffprobe.zip" -d "$OUTPUT_DIR"
            rm "$OUTPUT_DIR/ffprobe.zip"
        fi

        chmod +x "$OUTPUT_DIR/ffmpeg" "$OUTPUT_DIR/ffprobe"
        ;;

    linux-x64)
        log_info "Downloading FFmpeg for Linux x64..."

        # Use John Van Sickle's static builds
        FFMPEG_URL="https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz"

        curl -L "$FFMPEG_URL" -o "$OUTPUT_DIR/ffmpeg.tar.xz"
        tar -xf "$OUTPUT_DIR/ffmpeg.tar.xz" -C "$OUTPUT_DIR" --strip-components=1
        rm "$OUTPUT_DIR/ffmpeg.tar.xz"

        # Keep only ffmpeg and ffprobe
        find "$OUTPUT_DIR" -type f ! \( -name "ffmpeg" -o -name "ffprobe" \) -delete 2>/dev/null || true
        find "$OUTPUT_DIR" -type d -empty -delete 2>/dev/null || true

        chmod +x "$OUTPUT_DIR/ffmpeg" "$OUTPUT_DIR/ffprobe"
        ;;

    linux-arm64)
        log_info "Downloading FFmpeg for Linux ARM64..."

        FFMPEG_URL="https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-arm64-static.tar.xz"

        curl -L "$FFMPEG_URL" -o "$OUTPUT_DIR/ffmpeg.tar.xz"
        tar -xf "$OUTPUT_DIR/ffmpeg.tar.xz" -C "$OUTPUT_DIR" --strip-components=1
        rm "$OUTPUT_DIR/ffmpeg.tar.xz"

        # Keep only ffmpeg and ffprobe
        find "$OUTPUT_DIR" -type f ! \( -name "ffmpeg" -o -name "ffprobe" \) -delete 2>/dev/null || true
        find "$OUTPUT_DIR" -type d -empty -delete 2>/dev/null || true

        chmod +x "$OUTPUT_DIR/ffmpeg" "$OUTPUT_DIR/ffprobe"
        ;;

    windows-x64)
        log_info "Downloading FFmpeg for Windows x64..."

        # Use gyan.dev builds (BtbN builds are also an option)
        FFMPEG_URL="https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip"

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
