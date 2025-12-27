//! FFmpeg and FFprobe path resolution.
//!
//! This module handles finding FFmpeg and FFprobe binaries, with support for:
//! 1. Bundled binaries (relative to executable) - for distributed applications
//! 2. System PATH - for development and CLI usage

use crate::core::error::{ObsCutterError, Result};
use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Returns the path to the FFmpeg binary.
///
/// Resolution order:
/// 1. Bundled binary relative to the executable
/// 2. System PATH
pub fn get_ffmpeg_path() -> PathBuf {
    // Try bundled binary first
    if let Some(bundled) = get_bundled_path("ffmpeg") {
        if bundled.exists() {
            return bundled;
        }
    }

    // Fall back to system PATH
    PathBuf::from("ffmpeg")
}

/// Returns the path to the FFprobe binary.
///
/// Resolution order:
/// 1. Bundled binary relative to the executable
/// 2. System PATH
pub fn get_ffprobe_path() -> PathBuf {
    // Try bundled binary first
    if let Some(bundled) = get_bundled_path("ffprobe") {
        if bundled.exists() {
            return bundled;
        }
    }

    // Fall back to system PATH
    PathBuf::from("ffprobe")
}

/// Gets the path to a bundled binary relative to the executable.
fn get_bundled_path(binary_name: &str) -> Option<PathBuf> {
    let exe_path = env::current_exe().ok()?;
    let exe_dir = exe_path.parent()?;

    // Platform-specific binary names and locations
    #[cfg(target_os = "windows")]
    let binary_name = format!("{}.exe", binary_name);

    #[cfg(not(target_os = "windows"))]
    let binary_name = binary_name.to_string();

    // Check multiple possible locations:

    // 1. Same directory as executable (Windows, Linux portable)
    let same_dir = exe_dir.join(&binary_name);
    if same_dir.exists() {
        return Some(same_dir);
    }

    // 2. macOS .app bundle: Contents/Resources/
    #[cfg(target_os = "macos")]
    {
        if let Some(contents_dir) = exe_dir.parent() {
            let resources_path = contents_dir.join("Resources").join(&binary_name);
            if resources_path.exists() {
                return Some(resources_path);
            }
        }
    }

    // 3. bin/ subdirectory
    let bin_dir = exe_dir.join("bin").join(&binary_name);
    if bin_dir.exists() {
        return Some(bin_dir);
    }

    // 4. lib/ subdirectory (Linux)
    #[cfg(target_os = "linux")]
    {
        let lib_dir = exe_dir.join("lib").join(&binary_name);
        if lib_dir.exists() {
            return Some(lib_dir);
        }
    }

    None
}

/// Checks if FFmpeg is available and returns Ok if found.
pub fn check_ffmpeg() -> Result<()> {
    let ffmpeg_path = get_ffmpeg_path();

    // Try to run ffmpeg -version
    let output = Command::new(&ffmpeg_path)
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match output {
        Ok(status) if status.success() => Ok(()),
        _ => Err(ObsCutterError::FfmpegNotFound),
    }
}

/// Checks if FFprobe is available and returns Ok if found.
pub fn check_ffprobe() -> Result<()> {
    let ffprobe_path = get_ffprobe_path();

    // Try to run ffprobe -version
    let output = Command::new(&ffprobe_path)
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match output {
        Ok(status) if status.success() => Ok(()),
        _ => Err(ObsCutterError::FfprobeNotFound),
    }
}

/// Returns the FFmpeg version string, if available.
pub fn get_ffmpeg_version() -> Option<String> {
    let ffmpeg_path = get_ffmpeg_path();

    let output = Command::new(&ffmpeg_path)
        .arg("-version")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .ok()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Extract first line which contains version
        stdout.lines().next().map(|s| s.to_string())
    } else {
        None
    }
}

/// Returns true if using bundled FFmpeg, false if using system FFmpeg.
pub fn is_bundled() -> bool {
    let ffmpeg_path = get_ffmpeg_path();
    // If path is just "ffmpeg", it's using system PATH
    ffmpeg_path.components().count() > 1
}
