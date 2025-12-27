//! GUI message types for obs-cutter.

use std::path::PathBuf;

use crate::core::{HardwareEncoder, ProcessingResult, Quality, Side};

/// All possible messages in the GUI application.
#[derive(Debug, Clone)]
pub enum Message {
    // Navigation
    /// Navigate to the file selection screen.
    GoToFileSelection,
    /// Navigate to the settings screen.
    GoToSettings,

    // File Selection
    /// Open the file picker dialog.
    OpenFilePicker,
    /// Files have been selected from the picker.
    FilesSelected(Vec<PathBuf>),
    /// Remove a specific file from the list.
    RemoveFile(usize),
    /// Clear all selected files.
    ClearFiles,

    // Settings
    /// Change the quality preset.
    SetQuality(Quality),
    /// Change the output format.
    SetOutputFormat(Option<String>),
    /// Open the output directory picker.
    SelectOutputDir,
    /// Output directory has been selected.
    OutputDirSelected(Option<PathBuf>),
    /// Toggle hardware acceleration.
    ToggleHardwareAccel(bool),

    // Processing
    /// Start processing the selected videos.
    StartProcessing,
    /// Cancel the current processing.
    CancelProcessing,
    /// A video has been processed (one side complete).
    VideoSideProcessed {
        video_index: usize,
        side: Side,
        result: Result<(), String>,
    },
    /// A full video has been processed (both sides complete).
    VideoProcessed(Result<ProcessingResult, String>),
    /// All processing is complete.
    ProcessingComplete,
    /// Real-time encoding progress update from FFmpeg.
    EncodingProgress {
        video_index: usize,
        side: Side,
        percentage: f32,
        fps: f64,
        speed: f64,
        eta_secs: Option<f64>,
    },

    // Results
    /// Open the output directory in the file manager.
    OpenOutputDir,
    /// Process more videos (go back to file selection).
    ProcessMore,
    /// Exit the application.
    Exit,

    // System
    /// Hardware encoder has been detected.
    EncoderDetected(HardwareEncoder),
    /// FFmpeg check result.
    FfmpegChecked(bool),

    // Error handling
    /// An error occurred.
    Error(String),
}
