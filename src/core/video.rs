//! Video processing and analysis.

use crate::core::config::{Quality, Side};
use crate::core::encoder::{get_codec_args, HardwareEncoder};
use crate::core::error::{ObsCutterError, Result};
use crate::core::ffmpeg;
use crate::core::progress::{EncodingProgress, FfmpegProgressParser};
use serde::Deserialize;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

/// Information about a video stream from FFprobe.
#[derive(Debug, Clone, Deserialize)]
pub struct StreamInfo {
    #[serde(default)]
    pub width: Option<u32>,
    #[serde(default)]
    pub height: Option<u32>,
    pub codec_name: String,
    #[serde(default)]
    pub codec_type: Option<String>,
}

/// FFprobe output structure.
#[derive(Debug, Deserialize)]
struct ProbeOutput {
    streams: Vec<StreamInfo>,
}

/// Video file information.
#[derive(Debug, Clone)]
pub struct VideoInfo {
    /// Path to the video file.
    pub path: PathBuf,
    /// Video width in pixels.
    pub width: u32,
    /// Video height in pixels.
    pub height: u32,
    /// Video codec name (e.g., "h264").
    pub codec: String,
    /// File size in bytes (if available).
    pub file_size: Option<u64>,
}

impl VideoInfo {
    /// Returns true if the video has the expected 32:9 dimensions (3840x1080).
    pub fn is_valid_dimensions(&self) -> bool {
        self.width == 3840 && self.height == 1080
    }

    /// Returns the aspect ratio as a string.
    pub fn aspect_ratio(&self) -> String {
        let gcd = gcd(self.width, self.height);
        format!("{}:{}", self.width / gcd, self.height / gcd)
    }
}

/// Calculates the greatest common divisor.
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Result of processing a single video.
#[derive(Debug, Clone)]
pub struct ProcessingResult {
    /// Input video path.
    pub input: PathBuf,
    /// Left output video path.
    pub left_output: PathBuf,
    /// Right output video path.
    pub right_output: PathBuf,
    /// Left output file size in bytes.
    pub left_size: u64,
    /// Right output file size in bytes.
    pub right_size: u64,
    /// Processing duration.
    pub duration: Duration,
    /// Encoder used for processing.
    pub encoder_used: HardwareEncoder,
}

/// Progress information during video processing.
#[derive(Debug, Clone)]
pub enum ProcessingProgress {
    /// Currently analyzing a video.
    Analyzing {
        video_index: usize,
        total: usize,
        path: PathBuf,
    },
    /// Currently processing a video.
    Processing {
        video_index: usize,
        total: usize,
        side: Side,
        path: PathBuf,
    },
    /// A video has been completed.
    Completed {
        video_index: usize,
        total: usize,
        result: ProcessingResult,
    },
    /// A video processing failed.
    Failed {
        video_index: usize,
        total: usize,
        path: PathBuf,
        error: String,
    },
}

/// Trait for receiving progress updates during processing.
pub trait ProgressCallback: Send + Sync {
    fn on_progress(&self, progress: ProcessingProgress);
}

/// Gets video information using FFprobe.
pub fn get_video_info(video_path: &Path) -> Result<VideoInfo> {
    let ffprobe_path = ffmpeg::get_ffprobe_path();

    let output = Command::new(ffprobe_path)
        .args([
            "-v",
            "error",
            "-select_streams",
            "v:0",
            "-show_entries",
            "stream=width,height,codec_name,codec_type",
            "-of",
            "json",
        ])
        .arg(video_path)
        .output()
        .map_err(|e| ObsCutterError::VideoAnalysisFailed(e.to_string()))?;

    if !output.status.success() {
        return Err(ObsCutterError::VideoAnalysisFailed(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    let probe_output: ProbeOutput = serde_json::from_slice(&output.stdout)?;

    let stream = probe_output
        .streams
        .iter()
        .find(|s| {
            s.codec_type.as_deref() == Some("video") && s.width.is_some() && s.height.is_some()
        })
        .ok_or(ObsCutterError::NoVideoStream)?;

    let width = stream.width.ok_or(ObsCutterError::NoVideoStream)?;
    let height = stream.height.ok_or(ObsCutterError::NoVideoStream)?;

    // Get file size
    let file_size = std::fs::metadata(video_path).ok().map(|m| m.len());

    Ok(VideoInfo {
        path: video_path.to_path_buf(),
        width,
        height,
        codec: stream.codec_name.clone(),
        file_size,
    })
}

/// Processes a video to extract one side (left or right).
pub fn process_video_side(
    input: &Path,
    output: &Path,
    side: Side,
    quality: Quality,
    encoder: &HardwareEncoder,
) -> Result<()> {
    let ffmpeg_path = ffmpeg::get_ffmpeg_path();
    let crop_filter = side.crop_filter();
    let codec_args = get_codec_args(quality.as_str(), encoder);

    let mut args: Vec<String> = vec![
        "-i".to_string(),
        input.to_string_lossy().to_string(),
        "-vf".to_string(),
        crop_filter.to_string(),
    ];
    args.extend(codec_args);
    args.push("-y".to_string());
    args.push(output.to_string_lossy().to_string());

    let output_result = Command::new(ffmpeg_path)
        .args(&args)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| ObsCutterError::FfmpegFailed(e.to_string()))?;

    if !output_result.status.success() {
        let error = String::from_utf8_lossy(&output_result.stderr);
        return Err(ObsCutterError::FfmpegFailed(error.to_string()));
    }

    Ok(())
}

/// Processes a video to extract one side with real-time progress callbacks.
///
/// This version uses `.spawn()` instead of `.output()` to stream FFmpeg's
/// stderr and parse progress information in real-time.
pub fn process_video_side_with_progress<F>(
    input: &Path,
    output: &Path,
    side: Side,
    quality: Quality,
    encoder: &HardwareEncoder,
    total_duration: Option<f64>,
    mut progress_callback: F,
) -> Result<()>
where
    F: FnMut(EncodingProgress),
{
    let ffmpeg_path = ffmpeg::get_ffmpeg_path();
    let crop_filter = side.crop_filter();
    let codec_args = get_codec_args(quality.as_str(), encoder);

    let mut args: Vec<String> = vec![
        "-i".to_string(),
        input.to_string_lossy().to_string(),
        "-vf".to_string(),
        crop_filter.to_string(),
    ];
    args.extend(codec_args);
    args.push("-y".to_string());
    args.push(output.to_string_lossy().to_string());

    // Spawn the process instead of waiting for output
    let mut child = Command::new(ffmpeg_path)
        .args(&args)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| ObsCutterError::FfmpegFailed(e.to_string()))?;

    // Set up the progress parser
    let mut parser = if let Some(duration) = total_duration {
        FfmpegProgressParser::with_duration(duration)
    } else {
        FfmpegProgressParser::new()
    };

    // Read stderr line by line for progress updates
    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);

        // FFmpeg outputs progress on the same line using \r, so we need to
        // handle both newlines and carriage returns
        let mut buffer = String::new();
        let mut buf_reader = BufReader::new(reader.into_inner());

        loop {
            buffer.clear();
            match buf_reader.read_line(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(_) => {
                    // FFmpeg uses \r for progress updates on the same line
                    for line in buffer.split(['\r', '\n']) {
                        if !line.is_empty() {
                            if let Some(progress) = parser.parse_line(line) {
                                progress_callback(progress);
                            }
                        }
                    }
                }
                Err(_) => break,
            }
        }
    }

    // Wait for the process to complete
    let status = child
        .wait()
        .map_err(|e| ObsCutterError::FfmpegFailed(e.to_string()))?;

    if !status.success() {
        return Err(ObsCutterError::FfmpegFailed(
            "FFmpeg process exited with error".to_string(),
        ));
    }

    Ok(())
}

/// Get video duration using FFprobe.
pub fn get_video_duration(video_path: &Path) -> Result<f64> {
    let ffprobe_path = ffmpeg::get_ffprobe_path();

    let output = Command::new(ffprobe_path)
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
        ])
        .arg(video_path)
        .output()
        .map_err(|e| ObsCutterError::VideoAnalysisFailed(e.to_string()))?;

    if !output.status.success() {
        return Err(ObsCutterError::VideoAnalysisFailed(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    let duration_str = String::from_utf8_lossy(&output.stdout);
    duration_str
        .trim()
        .parse::<f64>()
        .map_err(|_| ObsCutterError::VideoAnalysisFailed("Failed to parse duration".to_string()))
}

/// Processes a single video, extracting both left and right sides.
pub fn process_video(
    input: &Path,
    output_dir: &Path,
    output_format: Option<&str>,
    quality: Quality,
    encoder: &HardwareEncoder,
) -> Result<ProcessingResult> {
    let start_time = std::time::Instant::now();

    // Prepare output paths
    let input_name = input
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| ObsCutterError::VideoNotFound(input.to_path_buf()))?;

    let input_ext = input.extension().and_then(|s| s.to_str()).unwrap_or("mp4");

    let ext = output_format.unwrap_or(input_ext);

    let output_left = output_dir.join(format!("{}-left.{}", input_name, ext));
    let output_right = output_dir.join(format!("{}-right.{}", input_name, ext));

    // Process left side
    process_video_side(input, &output_left, Side::Left, quality, encoder)?;

    // Process right side
    process_video_side(input, &output_right, Side::Right, quality, encoder)?;

    // Get output file sizes
    let left_size = std::fs::metadata(&output_left)
        .map(|m| m.len())
        .unwrap_or(0);
    let right_size = std::fs::metadata(&output_right)
        .map(|m| m.len())
        .unwrap_or(0);

    Ok(ProcessingResult {
        input: input.to_path_buf(),
        left_output: output_left,
        right_output: output_right,
        left_size,
        right_size,
        duration: start_time.elapsed(),
        encoder_used: encoder.clone(),
    })
}

/// Formats a byte count as a human-readable string.
pub fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Formats a duration as a human-readable string.
pub fn format_duration(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}
