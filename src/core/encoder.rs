//! Hardware encoder detection and configuration.

use crate::core::ffmpeg;
use std::process::{Command, Stdio};

/// Available hardware encoders for H.264 video encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareEncoder {
    /// VideoToolbox - macOS hardware encoder (Apple Silicon & Intel).
    VideoToolbox,

    /// NVENC - NVIDIA GPU hardware encoder.
    Nvenc,

    /// Quick Sync - Intel integrated GPU hardware encoder.
    QuickSync,

    /// AMF - AMD GPU hardware encoder.
    Amf,

    /// Software encoding fallback (libx264).
    None,
}

impl HardwareEncoder {
    /// Returns the FFmpeg encoder name for H.264.
    pub fn h264_encoder(&self) -> &'static str {
        match self {
            HardwareEncoder::VideoToolbox => "h264_videotoolbox",
            HardwareEncoder::Nvenc => "h264_nvenc",
            HardwareEncoder::QuickSync => "h264_qsv",
            HardwareEncoder::Amf => "h264_amf",
            HardwareEncoder::None => "libx264",
        }
    }

    /// Returns a human-readable name for the encoder.
    pub fn name(&self) -> &'static str {
        match self {
            HardwareEncoder::VideoToolbox => "VideoToolbox (Apple)",
            HardwareEncoder::Nvenc => "NVENC (NVIDIA)",
            HardwareEncoder::QuickSync => "Quick Sync (Intel)",
            HardwareEncoder::Amf => "AMF (AMD)",
            HardwareEncoder::None => "Software (libx264)",
        }
    }

    /// Returns true if this is a hardware encoder.
    pub fn is_hardware(&self) -> bool {
        !matches!(self, HardwareEncoder::None)
    }
}

impl Default for HardwareEncoder {
    fn default() -> Self {
        HardwareEncoder::None
    }
}

impl std::fmt::Display for HardwareEncoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Checks if a specific encoder is available in FFmpeg.
fn check_encoder_available(encoder_name: &str) -> bool {
    let ffmpeg_path = ffmpeg::get_ffmpeg_path();

    let output = Command::new(ffmpeg_path)
        .args(["-hide_banner", "-encoders"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            return stdout.contains(encoder_name);
        }
    }
    false
}

/// Detects the best available hardware encoder for the current system.
///
/// Checks encoders in order of preference:
/// 1. VideoToolbox (macOS only)
/// 2. NVENC (NVIDIA GPUs)
/// 3. Quick Sync (Intel)
/// 4. AMF (AMD)
/// 5. Software fallback (libx264)
pub fn detect_hardware_encoder() -> HardwareEncoder {
    // macOS: VideoToolbox (works on both Apple Silicon and Intel)
    if cfg!(target_os = "macos") && check_encoder_available("h264_videotoolbox") {
        return HardwareEncoder::VideoToolbox;
    }

    // NVIDIA: Check for NVENC
    if check_encoder_available("h264_nvenc") {
        return HardwareEncoder::Nvenc;
    }

    // Intel: Check for Quick Sync
    if check_encoder_available("h264_qsv") {
        return HardwareEncoder::QuickSync;
    }

    // AMD: Check for AMF
    if check_encoder_available("h264_amf") {
        return HardwareEncoder::Amf;
    }

    // Fallback to software encoding
    HardwareEncoder::None
}

/// Returns FFmpeg codec arguments for the given quality and encoder.
pub fn get_codec_args(quality: &str, encoder: &HardwareEncoder) -> Vec<String> {
    let encoder_name = encoder.h264_encoder();

    match encoder {
        HardwareEncoder::VideoToolbox => {
            // VideoToolbox uses bitrate-based encoding
            let bitrate = match quality {
                "high" => "15M",
                "medium" => "10M",
                _ => "25M", // lossless/highest quality
            };
            vec![
                "-c:v".to_string(),
                encoder_name.to_string(),
                "-b:v".to_string(),
                bitrate.to_string(),
                "-allow_sw".to_string(),
                "1".to_string(),
                "-c:a".to_string(),
                "copy".to_string(),
            ]
        }
        HardwareEncoder::Nvenc => {
            // NVENC supports CRF-like quality with -cq parameter
            let cq = match quality {
                "high" => "18",
                "medium" => "23",
                _ => "15", // lossless/highest quality
            };
            let preset = match quality {
                "high" => "p7",   // Slowest, highest quality
                "medium" => "p4", // Medium
                _ => "p7",        // Maximum quality for lossless
            };
            vec![
                "-c:v".to_string(),
                encoder_name.to_string(),
                "-preset".to_string(),
                preset.to_string(),
                "-cq".to_string(),
                cq.to_string(),
                "-c:a".to_string(),
                "copy".to_string(),
            ]
        }
        HardwareEncoder::QuickSync => {
            // Quick Sync uses global_quality parameter
            let quality_param = match quality {
                "high" => "18",
                "medium" => "23",
                _ => "15", // Best quality
            };
            vec![
                "-c:v".to_string(),
                encoder_name.to_string(),
                "-global_quality".to_string(),
                quality_param.to_string(),
                "-look_ahead".to_string(),
                "1".to_string(),
                "-c:a".to_string(),
                "copy".to_string(),
            ]
        }
        HardwareEncoder::Amf => {
            // AMF uses quality parameter
            let quality_param = match quality {
                "high" => "18",
                "medium" => "23",
                _ => "15", // Best quality
            };
            vec![
                "-c:v".to_string(),
                encoder_name.to_string(),
                "-rc".to_string(),
                "cqp".to_string(),
                "-qp_i".to_string(),
                quality_param.to_string(),
                "-qp_p".to_string(),
                quality_param.to_string(),
                "-c:a".to_string(),
                "copy".to_string(),
            ]
        }
        HardwareEncoder::None => {
            // Software encoding (libx264)
            match quality {
                "high" => vec![
                    "-c:v".to_string(),
                    "libx264".to_string(),
                    "-crf".to_string(),
                    "18".to_string(),
                    "-preset".to_string(),
                    "slow".to_string(),
                    "-c:a".to_string(),
                    "copy".to_string(),
                ],
                "medium" => vec![
                    "-c:v".to_string(),
                    "libx264".to_string(),
                    "-crf".to_string(),
                    "23".to_string(),
                    "-preset".to_string(),
                    "medium".to_string(),
                    "-c:a".to_string(),
                    "copy".to_string(),
                ],
                _ => vec![
                    "-c:v".to_string(),
                    "libx264".to_string(),
                    "-crf".to_string(),
                    "0".to_string(),
                    "-preset".to_string(),
                    "veryslow".to_string(),
                    "-c:a".to_string(),
                    "copy".to_string(),
                ], // lossless
            }
        }
    }
}
