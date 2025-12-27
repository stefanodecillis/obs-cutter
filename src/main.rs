//! obs-cutter CLI - Split 32:9 OBS recordings into two 16:9 videos.

use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::{Path, PathBuf};

use obs_cutter::core::{
    check_ffmpeg, detect_hardware_encoder, format_file_size, get_video_info, process_video,
    HardwareEncoder, Quality,
};

#[derive(Parser)]
#[command(name = "obs-cutter")]
#[command(version = "2.0.0")]
#[command(about = "Split 32:9 OBS recordings into two separate 16:9 videos", long_about = None)]
struct Cli {
    /// Path(s) to video file(s) to split
    #[arg(value_name = "VIDEO", required = true, num_args = 1..)]
    videos: Vec<PathBuf>,

    /// Output format (defaults to input format)
    #[arg(short, long, value_name = "FORMAT")]
    format: Option<String>,

    /// Quality preset (lossless/high/medium)
    #[arg(short, long, value_name = "QUALITY", default_value = "lossless")]
    quality: String,

    /// Output directory (defaults to input directory)
    #[arg(short, long, value_name = "DIR")]
    output: Option<PathBuf>,

    /// Disable hardware acceleration (force software encoding)
    #[arg(long)]
    no_hw_accel: bool,

    /// Continue processing remaining videos on error
    #[arg(long)]
    continue_on_error: bool,
}

/// Result of processing a single video in the batch.
struct BatchResult {
    path: PathBuf,
    success: bool,
    left_size: Option<u64>,
    right_size: Option<u64>,
    error: Option<String>,
}

fn print_header() {
    println!("\n{}", "OBS-Cutter - Video Splitter".cyan());
    println!("{}\n", "===========================".cyan());
}

fn print_ffmpeg_install_help() {
    println!("\n{}", "To install FFmpeg on macOS:".yellow());
    println!("  {}", "brew install ffmpeg".white());
    println!("\n{}", "On Ubuntu/Debian:".yellow());
    println!("  {}", "sudo apt-get install ffmpeg".white());
    println!("\n{}", "On Windows:".yellow());
    println!(
        "  {}",
        "Download from https://ffmpeg.org/download.html".white()
    );
}

fn setup_encoder(no_hw_accel: bool) -> HardwareEncoder {
    if no_hw_accel {
        println!("{} Hardware acceleration disabled by user\n", "ℹ".blue());
        HardwareEncoder::None
    } else {
        let detected = detect_hardware_encoder();
        if detected == HardwareEncoder::None {
            println!(
                "{} No hardware encoder detected, using software encoding\n",
                "ℹ".blue()
            );
        } else {
            println!(
                "{} Using hardware encoder: {}\n",
                "✓".green(),
                detected.name()
            );
        }
        detected
    }
}

fn process_single_video(
    video_path: &Path,
    output_dir: &Path,
    format: Option<&str>,
    quality: &Quality,
    encoder: &HardwareEncoder,
    video_index: usize,
    total_videos: usize,
) -> BatchResult {
    let prefix = if total_videos > 1 {
        format!("[{}/{}] ", video_index + 1, total_videos)
    } else {
        String::new()
    };

    // Check if video file exists
    if !video_path.exists() {
        eprintln!(
            "{}{} {}",
            prefix,
            "Error: Video file not found:".red(),
            video_path.display()
        );
        return BatchResult {
            path: video_path.to_path_buf(),
            success: false,
            left_size: None,
            right_size: None,
            error: Some("File not found".to_string()),
        };
    }

    // Get video information
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    spinner.set_message(format!("{}Analyzing video...", prefix));
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let video_info = match get_video_info(video_path) {
        Ok(info) => info,
        Err(e) => {
            spinner.finish_with_message(format!(
                "{}{} Failed to analyze: {}",
                prefix,
                "✗".red(),
                e
            ));
            return BatchResult {
                path: video_path.to_path_buf(),
                success: false,
                left_size: None,
                right_size: None,
                error: Some(e.to_string()),
            };
        }
    };

    spinner.finish_with_message(format!(
        "{}{} Video analyzed: {}x{}",
        prefix,
        "✓".green(),
        video_info.width,
        video_info.height
    ));

    // Validate video dimensions
    if !video_info.is_valid_dimensions() {
        println!(
            "\n{}{} Video dimensions are {}x{}",
            prefix,
            "Warning:".yellow(),
            video_info.width,
            video_info.height
        );
        println!(
            "{}{} Expected: 3840x1080 (32:9 aspect ratio)",
            prefix,
            "Warning:".yellow()
        );
        println!(
            "{}{} The output might not be as expected.\n",
            prefix,
            "Warning:".yellow()
        );
    }

    // Prepare output directory
    let input_dir = video_path.parent().unwrap_or(Path::new("."));
    let actual_output_dir = output_dir.parent().map(|_| output_dir).unwrap_or(input_dir);

    // Create output directory if it doesn't exist
    if !actual_output_dir.exists() {
        if let Err(e) = fs::create_dir_all(actual_output_dir) {
            return BatchResult {
                path: video_path.to_path_buf(),
                success: false,
                left_size: None,
                right_size: None,
                error: Some(format!("Failed to create output directory: {}", e)),
            };
        }
    }

    // Print configuration for this video
    let input_name = video_path.file_name().unwrap().to_string_lossy();
    println!("{}Processing: {}", prefix, input_name.white());

    // Process left video
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    spinner.set_message(format!("{}Extracting left video...", prefix));
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let result = process_video(video_path, actual_output_dir, format, *quality, encoder);

    match result {
        Ok(processing_result) => {
            spinner.finish_with_message(format!(
                "{}{} Split complete: {} | {}",
                prefix,
                "✓".green(),
                format_file_size(processing_result.left_size),
                format_file_size(processing_result.right_size)
            ));

            BatchResult {
                path: video_path.to_path_buf(),
                success: true,
                left_size: Some(processing_result.left_size),
                right_size: Some(processing_result.right_size),
                error: None,
            }
        }
        Err(e) => {
            spinner.finish_with_message(format!("{}{} Failed: {}", prefix, "✗".red(), e));
            BatchResult {
                path: video_path.to_path_buf(),
                success: false,
                left_size: None,
                right_size: None,
                error: Some(e.to_string()),
            }
        }
    }
}

fn print_summary(results: &[BatchResult]) {
    let successful = results.iter().filter(|r| r.success).count();
    let failed = results.iter().filter(|r| !r.success).count();
    let total = results.len();

    println!("\n{}", "═".repeat(50).cyan());
    println!("{}", "Summary".cyan().bold());
    println!("{}\n", "═".repeat(50).cyan());

    if total == 1 {
        if successful == 1 {
            println!("{}", "✓ Video split successfully!".green());
        } else {
            println!("{}", "✗ Video processing failed!".red());
        }
    } else {
        println!(
            "Total: {} | {} {} | {} {}",
            total.to_string().white().bold(),
            "✓".green(),
            successful.to_string().green(),
            "✗".red(),
            failed.to_string().red()
        );
    }

    // Show file sizes for successful videos
    let successful_results: Vec<_> = results.iter().filter(|r| r.success).collect();
    if !successful_results.is_empty() && total > 1 {
        println!("\n{}", "Processed files:".bright_black());
        for result in successful_results {
            let name = result.path.file_name().unwrap().to_string_lossy();
            let left = format_file_size(result.left_size.unwrap_or(0));
            let right = format_file_size(result.right_size.unwrap_or(0));
            println!(
                "  {} → Left: {}, Right: {}",
                name.white(),
                left.bright_black(),
                right.bright_black()
            );
        }
    } else if successful == 1 {
        let result = results.iter().find(|r| r.success).unwrap();
        println!("\n{}", "File sizes:".bright_black());
        println!(
            "  Left:  {}",
            format_file_size(result.left_size.unwrap_or(0)).bright_black()
        );
        println!(
            "  Right: {}",
            format_file_size(result.right_size.unwrap_or(0)).bright_black()
        );
    }

    // Show errors for failed videos
    let failed_results: Vec<_> = results.iter().filter(|r| !r.success).collect();
    if !failed_results.is_empty() {
        println!("\n{}", "Failed files:".red());
        for result in failed_results {
            let name = result.path.file_name().unwrap().to_string_lossy();
            let error = result.error.as_deref().unwrap_or("Unknown error");
            println!("  {} - {}", name.red(), error.bright_black());
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    print_header();

    // Check if FFmpeg is installed
    if check_ffmpeg().is_err() {
        eprintln!("{}", "Error: FFmpeg is not installed!".red());
        print_ffmpeg_install_help();
        std::process::exit(1);
    }

    // Parse quality
    let quality: Quality = cli.quality.parse().context("Invalid quality preset")?;

    // Detect hardware encoder
    let encoder = setup_encoder(cli.no_hw_accel);

    // Prepare output directory
    let output_dir = cli.output.clone().unwrap_or_else(|| PathBuf::from("."));

    // Print batch info
    if cli.videos.len() > 1 {
        println!(
            "{} Processing {} videos\n",
            "ℹ".blue(),
            cli.videos.len().to_string().white().bold()
        );
    }

    // Print configuration
    println!("{} {}", "Quality:".white(), quality.as_str());
    if let Some(ref format) = cli.format {
        println!("{} {}", "Output format:".white(), format);
    }
    if output_dir != PathBuf::from(".") {
        println!("{} {}", "Output directory:".white(), output_dir.display());
    }
    println!();

    // Process each video
    let mut results = Vec::new();

    for (index, video_path) in cli.videos.iter().enumerate() {
        let result = process_single_video(
            video_path,
            &output_dir,
            cli.format.as_deref(),
            &quality,
            &encoder,
            index,
            cli.videos.len(),
        );

        let failed = !result.success;
        results.push(result);

        // Stop on first error unless continue_on_error is set
        if failed && !cli.continue_on_error && index < cli.videos.len() - 1 {
            eprintln!(
                "\n{} Use {} to continue processing remaining videos",
                "Hint:".yellow(),
                "--continue-on-error".white()
            );
            break;
        }

        // Add spacing between videos
        if cli.videos.len() > 1 && index < cli.videos.len() - 1 {
            println!();
        }
    }

    // Print summary
    print_summary(&results);

    // Exit with error code if any failed
    let any_failed = results.iter().any(|r| !r.success);
    if any_failed {
        std::process::exit(1);
    }

    Ok(())
}
