//! Configuration types for video processing.

use std::path::PathBuf;
use std::str::FromStr;

use crate::core::error::{ObsCutterError, Result};

/// Quality preset for video encoding.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Quality {
    /// Lossless quality (CRF 0 for software, highest bitrate for hardware).
    /// Largest file sizes, best quality.
    #[default]
    Lossless,

    /// High quality (CRF 18 equivalent).
    /// Good balance between quality and file size.
    High,

    /// Medium quality (CRF 23 equivalent).
    /// Smaller files, acceptable quality.
    Medium,
}

impl Quality {
    /// Returns the quality preset as a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Quality::Lossless => "lossless",
            Quality::High => "high",
            Quality::Medium => "medium",
        }
    }

    /// Returns all available quality presets.
    pub fn all() -> &'static [Quality] {
        &[Quality::Lossless, Quality::High, Quality::Medium]
    }
}

impl FromStr for Quality {
    type Err = ObsCutterError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "lossless" => Ok(Quality::Lossless),
            "high" => Ok(Quality::High),
            "medium" => Ok(Quality::Medium),
            _ => Err(ObsCutterError::InvalidQuality(s.to_string())),
        }
    }
}

impl std::fmt::Display for Quality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Which side of the video to extract.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Side {
    /// Left half of the video (x=0).
    #[default]
    Left,
    /// Right half of the video (x=1920).
    Right,
}

impl Side {
    /// Returns the side as a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Side::Left => "left",
            Side::Right => "right",
        }
    }

    /// Returns the FFmpeg crop filter for this side.
    pub fn crop_filter(&self) -> &'static str {
        match self {
            Side::Left => "crop=1920:1080:0:0",
            Side::Right => "crop=1920:1080:1920:0",
        }
    }
}

impl FromStr for Side {
    type Err = ObsCutterError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "left" => Ok(Side::Left),
            "right" => Ok(Side::Right),
            _ => Err(ObsCutterError::InvalidSide(s.to_string())),
        }
    }
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Configuration for video processing.
#[derive(Debug, Clone)]
pub struct ProcessingConfig {
    /// Quality preset for encoding.
    pub quality: Quality,

    /// Output format (file extension). If None, uses input format.
    pub output_format: Option<String>,

    /// Output directory. If None, uses input file's directory.
    pub output_dir: Option<PathBuf>,

    /// Whether to use hardware acceleration.
    pub use_hardware_accel: bool,
}

impl Default for ProcessingConfig {
    fn default() -> Self {
        Self {
            quality: Quality::default(),
            output_format: None,
            output_dir: None,
            use_hardware_accel: true,
        }
    }
}

impl ProcessingConfig {
    /// Creates a new ProcessingConfig with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the quality preset.
    pub fn with_quality(mut self, quality: Quality) -> Self {
        self.quality = quality;
        self
    }

    /// Sets the output format.
    pub fn with_output_format(mut self, format: Option<String>) -> Self {
        self.output_format = format;
        self
    }

    /// Sets the output directory.
    pub fn with_output_dir(mut self, dir: Option<PathBuf>) -> Self {
        self.output_dir = dir;
        self
    }

    /// Sets whether to use hardware acceleration.
    pub fn with_hardware_accel(mut self, enabled: bool) -> Self {
        self.use_hardware_accel = enabled;
        self
    }
}
