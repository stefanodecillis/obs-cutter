//! Error types for obs-cutter operations.

use std::path::PathBuf;
use thiserror::Error;

/// Main error type for obs-cutter operations.
#[derive(Debug, Error)]
pub enum ObsCutterError {
    /// FFmpeg is not installed or not found.
    #[error("FFmpeg is not installed or not found in PATH")]
    FfmpegNotFound,

    /// FFprobe is not installed or not found.
    #[error("FFprobe is not installed or not found in PATH")]
    FfprobeNotFound,

    /// Video file not found.
    #[error("Video file not found: {0}")]
    VideoNotFound(PathBuf),

    /// Failed to analyze video.
    #[error("Failed to analyze video: {0}")]
    VideoAnalysisFailed(String),

    /// No video stream found in file.
    #[error("No video stream found in file")]
    NoVideoStream,

    /// Invalid video dimensions.
    #[error("Invalid video dimensions: {width}x{height}")]
    InvalidDimensions { width: u32, height: u32 },

    /// Invalid quality preset.
    #[error("Invalid quality preset: {0}. Valid options: lossless, high, medium")]
    InvalidQuality(String),

    /// Invalid side parameter.
    #[error("Invalid side: {0}. Valid options: left, right")]
    InvalidSide(String),

    /// FFmpeg processing failed.
    #[error("FFmpeg processing failed: {0}")]
    FfmpegFailed(String),

    /// Failed to create output directory.
    #[error("Failed to create output directory: {0}")]
    OutputDirectoryError(String),

    /// IO error.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON parsing error.
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Processing was cancelled.
    #[error("Processing was cancelled")]
    Cancelled,
}

/// Result type alias for obs-cutter operations.
pub type Result<T> = std::result::Result<T, ObsCutterError>;
