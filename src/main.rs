use anyhow::{bail, Context, Result};
use clap::Parser;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Parser)]
#[command(name = "obs-cutter")]
#[command(version = "1.0.0")]
#[command(about = "Split 32:9 OBS recordings into two separate 16:9 videos", long_about = None)]
struct Cli {
    /// Path to the video file to split
    #[arg(value_name = "VIDEO")]
    video: PathBuf,

    /// Output format (defaults to input format)
    #[arg(short, long, value_name = "FORMAT")]
    format: Option<String>,

    /// Quality preset (lossless/high/medium)
    #[arg(short, long, value_name = "QUALITY", default_value = "lossless")]
    quality: String,

    /// Output directory (defaults to input directory)
    #[arg(short, long, value_name = "DIR")]
    output: Option<PathBuf>,
}

#[derive(Debug, Clone, Deserialize)]
struct StreamInfo {
    #[serde(default)]
    width: Option<u32>,
    #[serde(default)]
    height: Option<u32>,
    codec_name: String,
    #[serde(default)]
    codec_type: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ProbeOutput {
    streams: Vec<StreamInfo>,
}

fn check_ffmpeg() -> Result<()> {
    let output = Command::new("which")
        .arg("ffmpeg")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match output {
        Ok(status) if status.success() => Ok(()),
        _ => bail!("FFmpeg is not installed"),
    }
}

fn get_video_info(video_path: &Path) -> Result<(u32, u32, String)> {
    let output = Command::new("ffprobe")
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
        .context("Failed to run ffprobe")?;

    if !output.status.success() {
        bail!("Failed to get video information");
    }

    let probe_output: ProbeOutput =
        serde_json::from_slice(&output.stdout).context("Failed to parse ffprobe output")?;

    let stream = probe_output
        .streams
        .iter()
        .find(|s| {
            s.codec_type.as_deref() == Some("video") && s.width.is_some() && s.height.is_some()
        })
        .context("No video stream found")?;

    let width = stream.width.context("Video stream missing width")?;
    let height = stream.height.context("Video stream missing height")?;
    let codec_name = stream.codec_name.clone();

    Ok((width, height, codec_name))
}

fn get_codec_args(quality: &str) -> Vec<&str> {
    match quality {
        "high" => vec![
            "-c:v", "libx264", "-crf", "18", "-preset", "slow", "-c:a", "copy",
        ],
        "medium" => vec![
            "-c:v", "libx264", "-crf", "23", "-preset", "medium", "-c:a", "copy",
        ],
        _ => vec![
            "-c:v", "libx264", "-crf", "0", "-preset", "veryslow", "-c:a", "copy",
        ], // lossless
    }
}

fn process_video(
    input: &Path,
    output: &Path,
    side: &str,
    quality: &str,
    _spinner: &ProgressBar,
) -> Result<()> {
    let crop_filter = match side {
        "left" => "crop=1920:1080:0:0",
        "right" => "crop=1920:1080:1920:0",
        _ => bail!("Invalid side: {}", side),
    };

    let codec_args = get_codec_args(quality);

    let mut args = vec!["-i", input.to_str().unwrap(), "-vf", crop_filter];
    args.extend_from_slice(&codec_args);
    args.extend_from_slice(&["-y", output.to_str().unwrap()]);

    let output = Command::new("ffmpeg")
        .args(&args)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to run ffmpeg")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        bail!("FFmpeg failed: {}", error);
    }

    Ok(())
}

fn format_file_size(bytes: u64) -> String {
    let mb = bytes as f64 / (1024.0 * 1024.0);
    format!("{:.2} MB", mb)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Print header
    println!("\n{}", "OBS-Cutter - Video Splitter".cyan());
    println!("{}\n", "===========================".cyan());

    // Check if FFmpeg is installed
    if check_ffmpeg().is_err() {
        eprintln!("{}", "Error: FFmpeg is not installed!".red());
        println!("\n{}", "To install FFmpeg on macOS:".yellow());
        println!("  {}", "brew install ffmpeg".white());
        println!("\n{}", "On Ubuntu/Debian:".yellow());
        println!("  {}", "sudo apt-get install ffmpeg".white());
        println!("\n{}", "On Windows:".yellow());
        println!(
            "  {}",
            "Download from https://ffmpeg.org/download.html".white()
        );
        std::process::exit(1);
    }

    // Check if video file exists
    if !cli.video.exists() {
        eprintln!(
            "{} {}",
            "Error: Video file not found:".red(),
            cli.video.display()
        );
        std::process::exit(1);
    }

    // Get video information
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    spinner.set_message("Analyzing video...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let (width, height, _codec_name) =
        get_video_info(&cli.video).context("Failed to analyze video")?;

    spinner.finish_with_message(format!(
        "{} Video analyzed: {}x{}",
        "✓".green(),
        width,
        height
    ));

    // Validate video dimensions
    if width != 3840 || height != 1080 {
        println!(
            "\n{} Video dimensions are {}x{}",
            "Warning:".yellow(),
            width,
            height
        );
        println!(
            "{} Expected: 3840x1080 (32:9 aspect ratio)",
            "Warning:".yellow()
        );
        println!(
            "{} The output might not be as expected.\n",
            "Warning:".yellow()
        );
    }

    // Prepare output paths
    let input_dir = cli.video.parent().unwrap_or(Path::new("."));
    let input_name = cli.video.file_stem().unwrap().to_str().unwrap();
    let input_ext = cli
        .format
        .as_deref()
        .unwrap_or_else(|| cli.video.extension().unwrap().to_str().unwrap());

    let default_output_dir = input_dir.to_path_buf();
    let output_dir = cli.output.as_ref().unwrap_or(&default_output_dir);

    // Create output directory if it doesn't exist
    if !output_dir.exists() {
        fs::create_dir_all(output_dir).context("Failed to create output directory")?;
    }

    let output_left = output_dir.join(format!("{}-left.{}", input_name, input_ext));
    let output_right = output_dir.join(format!("{}-right.{}", input_name, input_ext));

    // Print configuration
    println!("{} {}", "Input:  ".white(), cli.video.display());
    println!("{} {}", "Quality:".white(), cli.quality);
    println!("{}", "Output: ".white());
    println!("  - Left:  {}", output_left.display());
    println!("  - Right: {}\n", output_right.display());

    // Process left video
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    spinner.set_message("Extracting left video...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    process_video(&cli.video, &output_left, "left", &cli.quality, &spinner)?;

    spinner.finish_with_message(format!(
        "{} Left video saved: {}",
        "✓".green(),
        output_left.display()
    ));

    // Process right video
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    spinner.set_message("Extracting right video...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    process_video(&cli.video, &output_right, "right", &cli.quality, &spinner)?;

    spinner.finish_with_message(format!(
        "{} Right video saved: {}",
        "✓".green(),
        output_right.display()
    ));

    // Success message
    println!("\n{}\n", "✓ Video split successfully!".green());

    // Show file sizes
    let left_size = fs::metadata(&output_left)?.len();
    let right_size = fs::metadata(&output_right)?.len();

    println!("{}", "File sizes:".bright_black());
    println!("  Left:  {}", format_file_size(left_size).bright_black());
    println!("  Right: {}", format_file_size(right_size).bright_black());

    Ok(())
}
