# Changelog

All notable changes to this project will be documented in this file.

## [1.0.0] - 2025-10-10

### Added
- Initial Rust implementation
- CLI tool to split 32:9 (3840x1080) videos into two 16:9 (1920x1080) videos
- Multiple quality presets (lossless, high, medium)
- Progress indicators with spinners
- Comprehensive error handling
- FFmpeg validation
- Video dimension validation with warnings
- Colored terminal output
- File size reporting
- Homebrew formula for easy installation

### Fixed
- **ffprobe parsing error**: Fixed "missing field `width`" error by properly handling mixed stream types (video + audio)
  - Made `width` and `height` fields optional in `StreamInfo` struct
  - Added `codec_type` field to identify stream types
  - Added `-select_streams v:0` flag to ffprobe to target video streams
  - Filter streams to only process video streams with dimensions

### Technical Details

#### The Bug
When running obs-cutter on video files with both video and audio streams, ffprobe would return JSON with multiple streams. The audio stream doesn't have `width` and `height` fields, causing the parser to fail with:
```
Error: Failed to analyze video
Caused by:
    0: Failed to parse ffprobe output
    1: missing field `width` at line 16 column 9
```

#### The Fix
1. Changed `StreamInfo` struct to use `Option<u32>` for width/height
2. Added `codec_type: Option<String>` to identify stream type
3. Updated ffprobe command to include `-select_streams v:0` to target first video stream
4. Added filtering logic to find the first video stream with dimensions
5. Changed return type from `StreamInfo` to `(u32, u32, String)` tuple

### Dependencies
- clap 4.5 - CLI argument parsing
- colored 2.2 - Terminal colors
- indicatif 0.17 - Progress bars and spinners
- serde 1.0 - Serialization
- serde_json 1.0 - JSON parsing
- anyhow 1.0 - Error handling

### System Requirements
- Rust 1.70+ (for building)
- FFmpeg (runtime dependency)
- macOS, Linux, or Windows

## [Unreleased]

### Planned
- Add support for custom video dimensions
- Add batch processing for multiple files
- Add option to specify custom crop coordinates
- Add progress percentage during encoding
- Support for more video formats
- GPU acceleration support
