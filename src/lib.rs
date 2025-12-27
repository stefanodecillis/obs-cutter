//! # obs-cutter
//!
//! A library and CLI tool to split 32:9 OBS recordings into two separate 16:9 videos.
//!
//! ## Features
//!
//! - Splits 3840x1080 (32:9) videos into two 1920x1080 (16:9) videos
//! - Hardware acceleration support (VideoToolbox, NVENC, Quick Sync, AMF)
//! - Multiple quality presets (lossless, high, medium)
//! - Batch processing of multiple videos
//!
//! ## Example Usage (Library)
//!
//! ```no_run
//! use obs_cutter::core::{
//!     ProcessingConfig, Quality, detect_hardware_encoder,
//!     get_video_info, process_video, check_ffmpeg,
//! };
//! use std::path::Path;
//!
//! // Check FFmpeg is available
//! check_ffmpeg().expect("FFmpeg not found");
//!
//! // Detect hardware encoder
//! let encoder = detect_hardware_encoder();
//!
//! // Get video info
//! let video_path = Path::new("recording.mov");
//! let info = get_video_info(video_path).expect("Failed to analyze video");
//!
//! // Process video
//! let output_dir = Path::new("./output");
//! let result = process_video(
//!     video_path,
//!     output_dir,
//!     None, // Use input format
//!     Quality::High,
//!     &encoder,
//! ).expect("Failed to process video");
//!
//! println!("Left output: {:?}", result.left_output);
//! println!("Right output: {:?}", result.right_output);
//! ```

pub mod core;

// Re-export core module contents at crate root for convenience
pub use crate::core::*;

// Feature-gated GUI module
#[cfg(feature = "gui")]
pub mod gui;
