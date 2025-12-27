//! Core functionality for obs-cutter.
//!
//! This module contains the shared logic for video processing,
//! encoder detection, and configuration that is used by both
//! the CLI and GUI interfaces.

pub mod config;
pub mod encoder;
pub mod error;
pub mod ffmpeg;
pub mod progress;
pub mod video;

// Re-export commonly used types
pub use config::{ProcessingConfig, Quality, Side};
pub use encoder::{detect_hardware_encoder, get_codec_args, HardwareEncoder};
pub use error::{ObsCutterError, Result};
pub use ffmpeg::{check_ffmpeg, check_ffprobe, get_ffmpeg_path, get_ffprobe_path};
pub use progress::{EncodingProgress, FfmpegProgressParser};
pub use video::{
    format_duration, format_file_size, get_video_duration, get_video_info, process_video,
    process_video_side, process_video_side_with_progress, ProcessingProgress, ProcessingResult,
    VideoInfo,
};
