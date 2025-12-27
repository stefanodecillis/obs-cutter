//! FFmpeg progress parsing for real-time encoding updates.

use regex::Regex;
use std::sync::LazyLock;

/// Real-time encoding progress from FFmpeg.
#[derive(Debug, Clone, Default)]
pub struct EncodingProgress {
    /// Current position in the video (seconds).
    pub current_time_secs: f64,
    /// Total video duration (seconds).
    pub total_duration_secs: f64,
    /// Current frame number.
    pub current_frame: u64,
    /// Encoding speed in frames per second.
    pub fps: f64,
    /// Speed multiplier (e.g., 1.5 means 1.5x realtime).
    pub speed: f64,
    /// Encoding progress as percentage (0.0 - 100.0).
    pub percentage: f32,
}

impl EncodingProgress {
    /// Calculate estimated time remaining in seconds.
    pub fn eta_secs(&self) -> Option<f64> {
        if self.speed <= 0.0 || self.total_duration_secs <= 0.0 {
            return None;
        }
        let remaining_secs = self.total_duration_secs - self.current_time_secs;
        if remaining_secs <= 0.0 {
            return Some(0.0);
        }
        Some(remaining_secs / self.speed)
    }

    /// Format ETA as human-readable string.
    pub fn eta_string(&self) -> String {
        match self.eta_secs() {
            Some(secs) if secs < 60.0 => format!("~{}s", secs as u64),
            Some(secs) if secs < 3600.0 => {
                let mins = (secs / 60.0) as u64;
                let remaining_secs = (secs % 60.0) as u64;
                format!("~{}:{:02}", mins, remaining_secs)
            }
            Some(secs) => {
                let hours = (secs / 3600.0) as u64;
                let mins = ((secs % 3600.0) / 60.0) as u64;
                format!("~{}h {:02}m", hours, mins)
            }
            None => "calculating...".to_string(),
        }
    }
}

// Regex patterns for parsing FFmpeg output
static DURATION_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Duration:\s*(\d{2}):(\d{2}):(\d{2})\.(\d+)").unwrap());

static TIME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"time=\s*(\d{2}):(\d{2}):(\d{2})\.(\d+)").unwrap());

static FRAME_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"frame=\s*(\d+)").unwrap());

static FPS_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"fps=\s*([\d.]+)").unwrap());

static SPEED_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"speed=\s*([\d.]+)x").unwrap());

/// Parse a time string in HH:MM:SS.ms format to seconds.
fn parse_time_to_secs(hours: &str, mins: &str, secs: &str, ms: &str) -> f64 {
    let h: f64 = hours.parse().unwrap_or(0.0);
    let m: f64 = mins.parse().unwrap_or(0.0);
    let s: f64 = secs.parse().unwrap_or(0.0);

    // Handle variable-length milliseconds (could be 2 or 3 digits)
    let ms_str = if ms.len() > 2 { &ms[..2] } else { ms };
    let centisecs: f64 = ms_str.parse().unwrap_or(0.0);

    h * 3600.0 + m * 60.0 + s + centisecs / 100.0
}

/// Parse duration from FFmpeg output line.
/// Example: "Duration: 00:10:45.20, start: 0.000000, bitrate: 5000 kb/s"
pub fn parse_duration(line: &str) -> Option<f64> {
    DURATION_REGEX.captures(line).map(|caps| {
        parse_time_to_secs(
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str(),
            caps.get(3).unwrap().as_str(),
            caps.get(4).unwrap().as_str(),
        )
    })
}

/// Parse a progress line from FFmpeg stderr.
/// Example: "frame= 1234 fps= 45 q=28.0 size=  18234kB time=00:00:45.67 bitrate=3265.5kbits/s speed=1.23x"
pub fn parse_progress_line(line: &str, total_duration: f64) -> Option<EncodingProgress> {
    // Must have time= to be a progress line
    let current_time = TIME_REGEX.captures(line).map(|caps| {
        parse_time_to_secs(
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str(),
            caps.get(3).unwrap().as_str(),
            caps.get(4).unwrap().as_str(),
        )
    })?;

    let frame = FRAME_REGEX
        .captures(line)
        .and_then(|caps| caps.get(1)?.as_str().parse().ok())
        .unwrap_or(0);

    let fps = FPS_REGEX
        .captures(line)
        .and_then(|caps| caps.get(1)?.as_str().parse().ok())
        .unwrap_or(0.0);

    let speed = SPEED_REGEX
        .captures(line)
        .and_then(|caps| caps.get(1)?.as_str().parse().ok())
        .unwrap_or(0.0);

    let percentage = if total_duration > 0.0 {
        ((current_time / total_duration) * 100.0).min(100.0) as f32
    } else {
        0.0
    };

    Some(EncodingProgress {
        current_time_secs: current_time,
        total_duration_secs: total_duration,
        current_frame: frame,
        fps,
        speed,
        percentage,
    })
}

/// FFmpeg progress parser that maintains state across lines.
#[derive(Debug, Default)]
pub struct FfmpegProgressParser {
    /// Total duration of the input video.
    pub total_duration: f64,
    /// Whether we've found the duration line.
    pub duration_found: bool,
}

impl FfmpegProgressParser {
    /// Create a new parser.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a parser with a known duration (e.g., from ffprobe).
    pub fn with_duration(duration_secs: f64) -> Self {
        Self {
            total_duration: duration_secs,
            duration_found: true,
        }
    }

    /// Parse a line of FFmpeg stderr output.
    /// Returns Some(EncodingProgress) if this line contains progress information.
    pub fn parse_line(&mut self, line: &str) -> Option<EncodingProgress> {
        // Try to extract duration if we haven't found it yet
        if !self.duration_found {
            if let Some(duration) = parse_duration(line) {
                self.total_duration = duration;
                self.duration_found = true;
            }
        }

        // Try to parse as a progress line
        parse_progress_line(line, self.total_duration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() {
        let line = "  Duration: 00:10:45.20, start: 0.000000, bitrate: 5000 kb/s";
        let duration = parse_duration(line).unwrap();
        assert!((duration - 645.20).abs() < 0.01);
    }

    #[test]
    fn test_parse_progress_line() {
        let line = "frame= 1234 fps= 45.0 q=28.0 size=  18234kB time=00:00:45.67 bitrate=3265.5kbits/s speed=1.23x";
        let progress = parse_progress_line(line, 645.20).unwrap();

        assert_eq!(progress.current_frame, 1234);
        assert!((progress.fps - 45.0).abs() < 0.1);
        assert!((progress.speed - 1.23).abs() < 0.01);
        assert!((progress.current_time_secs - 45.67).abs() < 0.01);
        assert!((progress.percentage - 7.08).abs() < 0.1); // 45.67 / 645.20 * 100
    }

    #[test]
    fn test_parser_state() {
        let mut parser = FfmpegProgressParser::new();

        // First, feed the duration line
        let duration_line = "  Duration: 00:01:30.00, start: 0.000000, bitrate: 5000 kb/s";
        parser.parse_line(duration_line);
        assert!(parser.duration_found);
        assert!((parser.total_duration - 90.0).abs() < 0.01);

        // Then feed a progress line
        let progress_line = "frame=  500 fps= 30.0 size=   1024kB time=00:00:30.00 speed=2.00x";
        let progress = parser.parse_line(progress_line).unwrap();

        assert_eq!(progress.current_frame, 500);
        assert!((progress.percentage - 33.33).abs() < 0.1); // 30 / 90 * 100
    }

    #[test]
    fn test_eta_calculation() {
        let progress = EncodingProgress {
            current_time_secs: 30.0,
            total_duration_secs: 90.0,
            speed: 2.0,
            ..Default::default()
        };

        // Remaining: 60 secs, Speed: 2x, ETA: 30 secs
        let eta = progress.eta_secs().unwrap();
        assert!((eta - 30.0).abs() < 0.1);
    }
}
