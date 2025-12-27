//! Main GUI application state and logic.

use std::path::PathBuf;
use std::sync::mpsc;

use iced::widget::{
    button, center, column, container, horizontal_space, pick_list, progress_bar, radio, row,
    scrollable, text, toggler, Space,
};
use iced::{Alignment, Element, Fill, Length, Task, Theme};

use crate::core::{
    check_ffmpeg, detect_hardware_encoder, format_file_size, get_video_duration,
    process_video_side_with_progress, HardwareEncoder, ProcessingResult, Quality, Side,
};
use crate::gui::message::Message;
use crate::gui::theme::{self, colors};

/// Current screen in the application.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Screen {
    /// Initial screen for selecting video files.
    #[default]
    FileSelection,
    /// Settings configuration screen.
    Settings,
    /// Processing screen with progress indicators.
    Processing,
    /// Results screen showing completed operations.
    Results,
}

/// Application settings.
#[derive(Debug, Clone)]
pub struct Settings {
    pub quality: Quality,
    pub output_format: Option<String>,
    pub output_dir: Option<PathBuf>,
    pub use_hardware_accel: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            quality: Quality::Lossless,
            output_format: None,
            output_dir: None,
            use_hardware_accel: true,
        }
    }
}

/// Processing state information.
#[derive(Debug, Clone, Default)]
pub struct ProcessingState {
    pub current_video: usize,
    pub total_videos: usize,
    pub current_side: Side,
    pub current_status: String,
    pub is_cancelled: bool,
    // Encoding progress details
    pub encoding_percentage: f32,
    pub encoding_fps: f64,
    pub encoding_speed: f64,
    pub eta_secs: Option<f64>,
}

/// Main application state.
#[derive(Default)]
pub struct App {
    pub screen: Screen,
    pub videos: Vec<PathBuf>,
    pub settings: Settings,
    pub processing_state: ProcessingState,
    pub results: Vec<ProcessingResult>,
    pub errors: Vec<(PathBuf, String)>,
    pub encoder: HardwareEncoder,
    pub ffmpeg_available: bool,
    pub ffmpeg_checked: bool,
}

impl App {
    /// Create a new App instance.
    pub fn new() -> (Self, Task<Message>) {
        let app = Self::default();

        // Check FFmpeg availability and detect hardware encoder on startup
        let ffmpeg_task = Task::perform(async { check_ffmpeg().is_ok() }, Message::FfmpegChecked);

        let encoder_task = Task::perform(
            async { detect_hardware_encoder() },
            Message::EncoderDetected,
        );

        (app, Task::batch([ffmpeg_task, encoder_task]))
    }

    /// Get the window title.
    pub fn title(&self) -> String {
        match self.screen {
            Screen::FileSelection => String::from("OBS-Cutter"),
            Screen::Settings => String::from("OBS-Cutter - Settings"),
            Screen::Processing => String::from("OBS-Cutter - Processing"),
            Screen::Results => String::from("OBS-Cutter - Complete"),
        }
    }

    /// Get the theme.
    pub fn theme(&self) -> Theme {
        Theme::Dark
    }

    /// Handle messages and update state.
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // Navigation
            Message::GoToFileSelection => {
                self.screen = Screen::FileSelection;
                Task::none()
            }
            Message::GoToSettings => {
                self.screen = Screen::Settings;
                Task::none()
            }

            // File Selection
            Message::OpenFilePicker => Task::perform(
                async {
                    let files = rfd::AsyncFileDialog::new()
                        .add_filter("Video Files", &["mp4", "mov", "mkv", "avi", "webm"])
                        .set_title("Select Videos to Split")
                        .pick_files()
                        .await;

                    files
                        .map(|handles| {
                            handles
                                .into_iter()
                                .map(|h| h.path().to_path_buf())
                                .collect()
                        })
                        .unwrap_or_default()
                },
                Message::FilesSelected,
            ),
            Message::FilesSelected(paths) => {
                for path in paths {
                    if !self.videos.contains(&path) {
                        self.videos.push(path);
                    }
                }
                Task::none()
            }
            Message::RemoveFile(index) => {
                if index < self.videos.len() {
                    self.videos.remove(index);
                }
                Task::none()
            }
            Message::ClearFiles => {
                self.videos.clear();
                Task::none()
            }

            // Settings
            Message::SetQuality(quality) => {
                self.settings.quality = quality;
                Task::none()
            }
            Message::SetOutputFormat(format) => {
                self.settings.output_format = format;
                Task::none()
            }
            Message::SelectOutputDir => Task::perform(
                async {
                    rfd::AsyncFileDialog::new()
                        .set_title("Select Output Directory")
                        .pick_folder()
                        .await
                        .map(|h| h.path().to_path_buf())
                },
                Message::OutputDirSelected,
            ),
            Message::OutputDirSelected(path) => {
                self.settings.output_dir = path;
                Task::none()
            }
            Message::ToggleHardwareAccel(enabled) => {
                self.settings.use_hardware_accel = enabled;
                Task::none()
            }

            // Processing
            Message::StartProcessing => {
                self.screen = Screen::Processing;
                self.results.clear();
                self.errors.clear();
                self.processing_state = ProcessingState {
                    current_video: 0,
                    total_videos: self.videos.len(),
                    current_side: Side::Left,
                    current_status: "Starting...".to_string(),
                    is_cancelled: false,
                    encoding_percentage: 0.0,
                    encoding_fps: 0.0,
                    encoding_speed: 0.0,
                    eta_secs: None,
                };

                // Start processing the first video (left side first)
                self.process_next_video()
            }
            Message::CancelProcessing => {
                self.processing_state.is_cancelled = true;
                self.screen = Screen::FileSelection;
                Task::none()
            }
            Message::VideoProcessed(result) => {
                match result {
                    Ok(processing_result) => {
                        self.results.push(processing_result);
                    }
                    Err(error) => {
                        if let Some(video) = self.videos.get(self.processing_state.current_video) {
                            self.errors.push((video.clone(), error));
                        }
                    }
                }

                self.processing_state.current_video += 1;

                // Continue with next video or finish
                if self.processing_state.current_video < self.processing_state.total_videos
                    && !self.processing_state.is_cancelled
                {
                    self.process_next_video()
                } else {
                    Task::done(Message::ProcessingComplete)
                }
            }
            Message::ProcessingComplete => {
                self.screen = Screen::Results;
                Task::none()
            }
            Message::EncodingProgress {
                video_index: _,
                side,
                percentage,
                fps,
                speed,
                eta_secs,
            } => {
                // Update encoding progress in real-time
                self.processing_state.current_side = side;
                self.processing_state.encoding_percentage = percentage;
                self.processing_state.encoding_fps = fps;
                self.processing_state.encoding_speed = speed;
                self.processing_state.eta_secs = eta_secs;
                Task::none()
            }
            Message::VideoSideProcessed {
                video_index: _,
                side,
                result,
            } => {
                match result {
                    Ok(()) => {
                        // Side completed successfully
                        if side == Side::Left {
                            // Left done, continue with right
                            self.processing_state.current_side = Side::Right;
                            self.processing_state.encoding_percentage = 0.0;
                            self.processing_state.current_status = format!(
                                "Encoding right side of: {}",
                                self.videos
                                    .get(self.processing_state.current_video)
                                    .and_then(|p| p.file_name())
                                    .unwrap_or_default()
                                    .to_string_lossy()
                            );
                            self.process_video_side(Side::Right)
                        } else {
                            // Right done, video complete - collect result
                            self.collect_video_result()
                        }
                    }
                    Err(error) => {
                        // Side failed, record error and move on
                        if let Some(video) = self.videos.get(self.processing_state.current_video) {
                            self.errors.push((video.clone(), error));
                        }
                        self.processing_state.current_video += 1;

                        if self.processing_state.current_video < self.processing_state.total_videos
                            && !self.processing_state.is_cancelled
                        {
                            self.process_next_video()
                        } else {
                            Task::done(Message::ProcessingComplete)
                        }
                    }
                }
            }

            // Results
            Message::OpenOutputDir => {
                if let Some(ref output_dir) = self.settings.output_dir {
                    let _ = open::that(output_dir);
                } else if let Some(first_result) = self.results.first() {
                    if let Some(parent) = first_result.left_output.parent() {
                        let _ = open::that(parent);
                    }
                }
                Task::none()
            }
            Message::ProcessMore => {
                self.videos.clear();
                self.results.clear();
                self.errors.clear();
                self.screen = Screen::FileSelection;
                Task::none()
            }
            Message::Exit => {
                std::process::exit(0);
            }

            // System
            Message::EncoderDetected(encoder) => {
                self.encoder = encoder;
                Task::none()
            }
            Message::FfmpegChecked(available) => {
                self.ffmpeg_available = available;
                self.ffmpeg_checked = true;
                Task::none()
            }

            // Error handling
            Message::Error(error) => {
                eprintln!("Error: {}", error);
                Task::none()
            }
        }
    }

    /// Process the next video in the queue (starts with left side).
    fn process_next_video(&mut self) -> Task<Message> {
        if self.processing_state.current_video >= self.videos.len() {
            return Task::done(Message::ProcessingComplete);
        }

        let video = &self.videos[self.processing_state.current_video];
        self.processing_state.current_side = Side::Left;
        self.processing_state.encoding_percentage = 0.0;
        self.processing_state.current_status = format!(
            "Encoding left side of: {}",
            video.file_name().unwrap_or_default().to_string_lossy()
        );

        self.process_video_side(Side::Left)
    }

    /// Process a specific side of the current video with real-time progress.
    fn process_video_side(&self, side: Side) -> Task<Message> {
        let video_index = self.processing_state.current_video;
        let video = self.videos[video_index].clone();
        let quality = self.settings.quality;
        let output_format = self.settings.output_format.clone();
        let output_dir = self.settings.output_dir.clone();
        let encoder = if self.settings.use_hardware_accel {
            self.encoder
        } else {
            HardwareEncoder::None
        };

        // Create a channel for progress updates
        let (tx, rx) = mpsc::channel::<Message>();

        // Spawn the processing task
        let process_task = Task::perform(
            async move {
                // Determine output directory
                let output_path = output_dir.unwrap_or_else(|| {
                    video
                        .parent()
                        .unwrap_or(std::path::Path::new("."))
                        .to_path_buf()
                });

                // Create output directory if needed
                if !output_path.exists() {
                    std::fs::create_dir_all(&output_path)
                        .map_err(|e| format!("Failed to create output directory: {}", e))?;
                }

                // Prepare output path
                let input_name = video
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("video");
                let input_ext = video.extension().and_then(|s| s.to_str()).unwrap_or("mp4");
                let ext = output_format.as_deref().unwrap_or(input_ext);
                let side_suffix = if side == Side::Left { "left" } else { "right" };
                let output_file =
                    output_path.join(format!("{}-{}.{}", input_name, side_suffix, ext));

                // Get video duration for progress calculation
                let duration = get_video_duration(&video).ok();

                // Process with progress callback
                std::thread::spawn(move || {
                    process_video_side_with_progress(
                        &video,
                        &output_file,
                        side,
                        quality,
                        &encoder,
                        duration,
                        |progress| {
                            let _ = tx.send(Message::EncodingProgress {
                                video_index,
                                side,
                                percentage: progress.percentage,
                                fps: progress.fps,
                                speed: progress.speed,
                                eta_secs: if progress.speed > 0.0
                                    && progress.total_duration_secs > 0.0
                                {
                                    let remaining =
                                        progress.total_duration_secs - progress.current_time_secs;
                                    Some(remaining / progress.speed)
                                } else {
                                    None
                                },
                            });
                        },
                    )
                })
                .join()
                .map_err(|_| "Thread panicked".to_string())?
                .map_err(|e| e.to_string())
            },
            move |result| Message::VideoSideProcessed {
                video_index,
                side,
                result,
            },
        );

        // Create a stream of progress messages from the channel
        let progress_stream = Task::run(
            async_stream::stream! {
                loop {
                    match rx.try_recv() {
                        Ok(msg) => yield msg,
                        Err(mpsc::TryRecvError::Empty) => {
                            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                        }
                        Err(mpsc::TryRecvError::Disconnected) => break,
                    }
                }
            },
            |msg| msg,
        );

        Task::batch([process_task, progress_stream])
    }

    /// Collect the result after both sides are processed.
    fn collect_video_result(&mut self) -> Task<Message> {
        let video_index = self.processing_state.current_video;
        let video = self.videos[video_index].clone();
        let output_format = self.settings.output_format.clone();
        let output_dir = self.settings.output_dir.clone();
        let encoder = if self.settings.use_hardware_accel {
            self.encoder
        } else {
            HardwareEncoder::None
        };
        let start_time = std::time::Instant::now();

        // Advance to next video
        self.processing_state.current_video += 1;

        Task::perform(
            async move {
                let output_path = output_dir.unwrap_or_else(|| {
                    video
                        .parent()
                        .unwrap_or(std::path::Path::new("."))
                        .to_path_buf()
                });

                let input_name = video
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("video");
                let input_ext = video.extension().and_then(|s| s.to_str()).unwrap_or("mp4");
                let ext = output_format.as_deref().unwrap_or(input_ext);

                let left_output = output_path.join(format!("{}-left.{}", input_name, ext));
                let right_output = output_path.join(format!("{}-right.{}", input_name, ext));

                let left_size = std::fs::metadata(&left_output)
                    .map(|m| m.len())
                    .unwrap_or(0);
                let right_size = std::fs::metadata(&right_output)
                    .map(|m| m.len())
                    .unwrap_or(0);

                Ok(ProcessingResult {
                    input: video,
                    left_output,
                    right_output,
                    left_size,
                    right_size,
                    duration: start_time.elapsed(),
                    encoder_used: encoder,
                })
            },
            Message::VideoProcessed,
        )
    }

    /// Render the current view.
    pub fn view(&self) -> Element<'_, Message> {
        let content = match self.screen {
            Screen::FileSelection => self.view_file_selection(),
            Screen::Settings => self.view_settings(),
            Screen::Processing => self.view_processing(),
            Screen::Results => self.view_results(),
        };

        container(content)
            .width(Fill)
            .height(Fill)
            .padding(30)
            .style(|_| container::Style {
                background: Some(iced::Background::Color(colors::BACKGROUND)),
                ..Default::default()
            })
            .into()
    }

    /// File selection screen view.
    fn view_file_selection(&self) -> Element<'_, Message> {
        // Header row
        let title = text("OBS-Cutter").size(32).color(colors::TEXT_PRIMARY);

        let settings_btn = button(text("Settings").size(14).color(colors::TEXT_PRIMARY))
            .padding([10, 20])
            .style(theme::secondary_button)
            .on_press(Message::GoToSettings);

        let header = row![title, horizontal_space(), settings_btn].align_y(Alignment::Center);

        // Encoder status
        let status_content = if !self.ffmpeg_checked {
            text("Checking FFmpeg...")
                .size(14)
                .color(colors::TEXT_MUTED)
        } else if self.ffmpeg_available {
            text(format!("Ready - Using encoder: {}", self.encoder.name()))
                .size(14)
                .color(colors::SUCCESS)
        } else {
            text("FFmpeg not found! Please check installation.")
                .size(14)
                .color(colors::DANGER)
        };

        // File selection zone
        let selection_zone_content = column![
            Space::with_height(20),
            text("Select Videos to Split")
                .size(20)
                .color(colors::TEXT_PRIMARY),
            Space::with_height(16),
            button(text("Browse Files").size(15))
                .padding([12, 32])
                .style(theme::primary_button)
                .on_press_maybe(if self.ffmpeg_available {
                    Some(Message::OpenFilePicker)
                } else {
                    None
                }),
            Space::with_height(20),
        ]
        .align_x(Alignment::Center)
        .width(Fill);

        let selection_zone = container(selection_zone_content)
            .width(Fill)
            .style(theme::drop_zone)
            .padding(20);

        // Selected files header
        let file_count = text(format!("Selected Videos ({})", self.videos.len()))
            .size(18)
            .color(colors::TEXT_PRIMARY);

        let clear_btn = if !self.videos.is_empty() {
            button(text("Clear All").size(13).color(colors::TEXT_PRIMARY))
                .padding([8, 16])
                .style(theme::danger_button)
                .on_press(Message::ClearFiles)
        } else {
            button(text("Clear All").size(13).color(colors::TEXT_MUTED))
                .padding([8, 16])
                .style(theme::secondary_button)
        };

        let files_header =
            row![file_count, horizontal_space(), clear_btn].align_y(Alignment::Center);

        // Files list
        let files_content: Element<'_, Message> = if self.videos.is_empty() {
            container(
                text("No videos selected - add some videos to get started")
                    .size(14)
                    .color(colors::TEXT_MUTED),
            )
            .width(Fill)
            .padding(30)
            .center_x(Fill)
            .into()
        } else {
            let mut files_column = column![].spacing(8);

            for (index, path) in self.videos.iter().enumerate() {
                let filename = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                let file_size = std::fs::metadata(path)
                    .map(|m| format_file_size(m.len()))
                    .unwrap_or_else(|_| "Unknown size".to_string());

                let remove_btn = button(text("Remove").size(12).color(colors::TEXT_PRIMARY))
                    .padding([6, 12])
                    .style(theme::secondary_button)
                    .on_press(Message::RemoveFile(index));

                let file_row = container(
                    row![
                        text(format!("{}.", index + 1))
                            .size(14)
                            .color(colors::TEXT_MUTED)
                            .width(Length::Fixed(30.0)),
                        column![
                            text(filename).size(14).color(colors::TEXT_PRIMARY),
                            text(file_size).size(12).color(colors::TEXT_SECONDARY),
                        ]
                        .spacing(2)
                        .width(Fill),
                        remove_btn,
                    ]
                    .spacing(12)
                    .align_y(Alignment::Center)
                    .padding(12),
                )
                .style(theme::file_row);

                files_column = files_column.push(file_row);
            }

            scrollable(files_column).height(200).into()
        };

        // Start button
        let can_start = !self.videos.is_empty() && self.ffmpeg_available;
        let start_btn = button(text("Start Processing").size(16))
            .padding([14, 40])
            .style(if can_start {
                theme::success_button
            } else {
                theme::secondary_button
            })
            .on_press_maybe(if can_start {
                Some(Message::StartProcessing)
            } else {
                None
            });

        let actions = row![horizontal_space(), start_btn];

        // Main layout
        column![
            header,
            Space::with_height(8),
            status_content,
            Space::with_height(24),
            selection_zone,
            Space::with_height(24),
            files_header,
            Space::with_height(12),
            files_content,
            Space::with_height(24),
            actions,
        ]
        .into()
    }

    /// Settings screen view.
    fn view_settings(&self) -> Element<'_, Message> {
        // Header
        let title = text("Settings").size(32).color(colors::TEXT_PRIMARY);
        let back_btn = button(text("Back").size(14).color(colors::TEXT_PRIMARY))
            .padding([10, 20])
            .style(theme::secondary_button)
            .on_press(Message::GoToFileSelection);

        let header = row![title, horizontal_space(), back_btn].align_y(Alignment::Center);

        // Quality section
        let quality_title = text("Quality Preset").size(18).color(colors::TEXT_PRIMARY);
        let quality_radios = column![
            radio(
                "Lossless - Largest files, best quality",
                Quality::Lossless,
                Some(self.settings.quality),
                Message::SetQuality,
            )
            .size(18),
            radio(
                "High - Good balance of size and quality",
                Quality::High,
                Some(self.settings.quality),
                Message::SetQuality,
            )
            .size(18),
            radio(
                "Medium - Smaller files, decent quality",
                Quality::Medium,
                Some(self.settings.quality),
                Message::SetQuality,
            )
            .size(18),
        ]
        .spacing(12);

        let quality_section =
            container(column![quality_title, Space::with_height(12), quality_radios].padding(16))
                .style(theme::card)
                .width(Fill);

        // Output format section
        let format_title = text("Output Format").size(18).color(colors::TEXT_PRIMARY);
        let format_options = vec![
            "Same as input".to_string(),
            "mp4".to_string(),
            "mov".to_string(),
            "mkv".to_string(),
        ];
        let current_format = self
            .settings
            .output_format
            .clone()
            .unwrap_or_else(|| "Same as input".to_string());

        let format_picker = pick_list(format_options, Some(current_format), |s: String| {
            if s == "Same as input" {
                Message::SetOutputFormat(None)
            } else {
                Message::SetOutputFormat(Some(s))
            }
        })
        .padding(10)
        .width(Length::Fixed(200.0))
        .style(theme::pick_list_style)
        .menu_style(theme::pick_list_menu);

        let format_section =
            container(column![format_title, Space::with_height(12), format_picker].padding(16))
                .style(theme::card)
                .width(Fill);

        // Output directory section
        let dir_title = text("Output Directory")
            .size(18)
            .color(colors::TEXT_PRIMARY);
        let dir_text = self
            .settings
            .output_dir
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "Same as input file location".to_string());

        let browse_btn = button(text("Browse").size(14).color(colors::TEXT_PRIMARY))
            .padding([10, 20])
            .style(theme::secondary_button)
            .on_press(Message::SelectOutputDir);

        let dir_row = row![
            text(dir_text)
                .size(14)
                .color(colors::TEXT_SECONDARY)
                .width(Fill),
            browse_btn,
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let dir_section =
            container(column![dir_title, Space::with_height(12), dir_row].padding(16))
                .style(theme::card)
                .width(Fill);

        // Hardware acceleration section
        let hw_title = text("Hardware Acceleration")
            .size(18)
            .color(colors::TEXT_PRIMARY);
        let hw_status = if self.settings.use_hardware_accel {
            format!("Enabled - {}", self.encoder.name())
        } else {
            "Disabled - using software encoding".to_string()
        };

        let hw_row = row![
            toggler(self.settings.use_hardware_accel)
                .on_toggle(Message::ToggleHardwareAccel)
                .size(24),
            Space::with_width(12),
            text(hw_status).size(14).color(colors::TEXT_SECONDARY),
        ]
        .align_y(Alignment::Center);

        let hw_section = container(column![hw_title, Space::with_height(12), hw_row].padding(16))
            .style(theme::card)
            .width(Fill);

        // Note: Settings are saved automatically when changed

        column![
            header,
            Space::with_height(30),
            quality_section,
            Space::with_height(16),
            format_section,
            Space::with_height(16),
            dir_section,
            Space::with_height(16),
            hw_section,
        ]
        .into()
    }

    /// Processing screen view.
    fn view_processing(&self) -> Element<'_, Message> {
        let title = text("Processing Videos")
            .size(32)
            .color(colors::TEXT_PRIMARY);

        let current = self.processing_state.current_video + 1;
        let total = self.processing_state.total_videos;

        let progress_text = text(format!("Video {} of {}", current.min(total), total))
            .size(20)
            .color(colors::TEXT_PRIMARY);

        let status = text(&self.processing_state.current_status)
            .size(14)
            .color(colors::TEXT_SECONDARY);

        // Calculate overall progress:
        // Each video has 2 sides (left + right), so 2 phases per video
        let phases_per_video = 2.0_f32;
        let total_phases = total as f32 * phases_per_video;
        let completed_phases = self.processing_state.current_video as f32 * phases_per_video
            + if self.processing_state.current_side == Side::Right {
                1.0
            } else {
                0.0
            };
        let current_phase_progress = self.processing_state.encoding_percentage / 100.0;
        let progress_value = if total_phases > 0.0 {
            (completed_phases + current_phase_progress) / total_phases
        } else {
            0.0
        };

        let progress = progress_bar(0.0..=1.0, progress_value)
            .height(24)
            .style(theme::progress);

        let percentage = text(format!("{}%", (progress_value * 100.0) as u32))
            .size(16)
            .color(colors::TEXT_PRIMARY);

        // Format encoding stats
        let speed_text = if self.processing_state.encoding_speed > 0.0 {
            format!("Speed: {:.2}x", self.processing_state.encoding_speed)
        } else {
            "Speed: --".to_string()
        };

        let fps_text = if self.processing_state.encoding_fps > 0.0 {
            format!("FPS: {:.0}", self.processing_state.encoding_fps)
        } else {
            "FPS: --".to_string()
        };

        let eta_text = match self.processing_state.eta_secs {
            Some(secs) if secs > 0.0 => {
                let mins = (secs / 60.0) as u32;
                let remaining_secs = (secs % 60.0) as u32;
                if mins > 0 {
                    format!("ETA: ~{}:{:02}", mins, remaining_secs)
                } else {
                    format!("ETA: ~{}s", remaining_secs)
                }
            }
            _ => "ETA: --".to_string(),
        };

        let stats_row = row![
            text(speed_text).size(13).color(colors::TEXT_SECONDARY),
            text("  |  ").size(13).color(colors::TEXT_MUTED),
            text(fps_text).size(13).color(colors::TEXT_SECONDARY),
            text("  |  ").size(13).color(colors::TEXT_MUTED),
            text(eta_text).size(13).color(colors::TEXT_SECONDARY),
        ]
        .align_y(Alignment::Center);

        // Completed videos list
        let completed_content: Element<'_, Message> = if !self.results.is_empty() {
            let mut completed_list = column![].spacing(8);
            for result in &self.results {
                let name = result
                    .input
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy();

                completed_list = completed_list.push(
                    row![
                        text("✓").size(14).color(colors::SUCCESS),
                        Space::with_width(8),
                        text(name.to_string()).size(14).color(colors::TEXT_PRIMARY),
                        horizontal_space(),
                        text(format!(
                            "{} | {}",
                            format_file_size(result.left_size),
                            format_file_size(result.right_size)
                        ))
                        .size(12)
                        .color(colors::TEXT_SECONDARY),
                    ]
                    .align_y(Alignment::Center),
                );
            }

            container(scrollable(completed_list).height(150))
                .style(theme::card)
                .padding(16)
                .width(Fill)
                .into()
        } else {
            Space::with_height(0).into()
        };

        let cancel_btn = button(text("Cancel").size(14))
            .padding([12, 32])
            .style(theme::danger_button)
            .on_press(Message::CancelProcessing);

        center(
            column![
                title,
                Space::with_height(40),
                progress_text,
                Space::with_height(8),
                status,
                Space::with_height(24),
                progress,
                Space::with_height(8),
                percentage,
                Space::with_height(12),
                stats_row,
                Space::with_height(30),
                if !self.results.is_empty() {
                    column![
                        text("Completed:").size(16).color(colors::TEXT_PRIMARY),
                        Space::with_height(12),
                        completed_content,
                    ]
                } else {
                    column![]
                },
                Space::with_height(30),
                cancel_btn,
            ]
            .align_x(Alignment::Center)
            .max_width(500),
        )
        .into()
    }

    /// Results screen view.
    fn view_results(&self) -> Element<'_, Message> {
        let success_count = self.results.len();
        let error_count = self.errors.len();

        let title = if error_count == 0 {
            text("Processing Complete!").size(32).color(colors::SUCCESS)
        } else if success_count == 0 {
            text("Processing Failed").size(32).color(colors::DANGER)
        } else {
            text("Processing Finished").size(32).color(colors::WARNING)
        };

        let summary = text(format!(
            "{} successful, {} failed",
            success_count, error_count
        ))
        .size(16)
        .color(colors::TEXT_SECONDARY);

        // Results list
        let results_content: Element<'_, Message> = {
            let mut col = column![].spacing(12);

            for result in &self.results {
                let name = result
                    .input
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy();

                let result_row = container(
                    column![
                        row![
                            text("✓").size(16).color(colors::SUCCESS),
                            Space::with_width(8),
                            text(name.to_string()).size(15).color(colors::TEXT_PRIMARY),
                        ]
                        .align_y(Alignment::Center),
                        row![
                            text(format!("Left: {}", format_file_size(result.left_size)))
                                .size(13)
                                .color(colors::TEXT_SECONDARY),
                            text("  |  ").size(13).color(colors::TEXT_MUTED),
                            text(format!("Right: {}", format_file_size(result.right_size)))
                                .size(13)
                                .color(colors::TEXT_SECONDARY),
                        ],
                    ]
                    .spacing(4)
                    .padding(12),
                )
                .style(theme::file_row);

                col = col.push(result_row);
            }

            for (path, error) in &self.errors {
                let name = path.file_name().unwrap_or_default().to_string_lossy();

                let error_row = container(
                    column![
                        row![
                            text("✗").size(16).color(colors::DANGER),
                            Space::with_width(8),
                            text(name.to_string()).size(15).color(colors::TEXT_PRIMARY),
                        ]
                        .align_y(Alignment::Center),
                        text(error).size(12).color(colors::DANGER),
                    ]
                    .spacing(4)
                    .padding(12),
                )
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(
                        0.25, 0.15, 0.15,
                    ))),
                    border: iced::Border {
                        color: colors::DANGER,
                        width: 1.0,
                        radius: 6.0.into(),
                    },
                    ..Default::default()
                });

                col = col.push(error_row);
            }

            if success_count == 0 && error_count == 0 {
                text("No videos were processed.")
                    .size(14)
                    .color(colors::TEXT_MUTED)
                    .into()
            } else {
                scrollable(col).height(250).into()
            }
        };

        // Action buttons
        let open_btn = button(text("Open Folder").size(14))
            .padding([12, 24])
            .style(theme::primary_button)
            .on_press(Message::OpenOutputDir);

        let more_btn = button(text("Process More").size(14).color(colors::TEXT_PRIMARY))
            .padding([12, 24])
            .style(theme::secondary_button)
            .on_press(Message::ProcessMore);

        let exit_btn = button(text("Exit").size(14).color(colors::TEXT_PRIMARY))
            .padding([12, 24])
            .style(theme::secondary_button)
            .on_press(Message::Exit);

        let actions = row![open_btn, more_btn, exit_btn].spacing(16);

        center(
            column![
                title,
                Space::with_height(12),
                summary,
                Space::with_height(30),
                results_content,
                Space::with_height(30),
                actions,
            ]
            .align_x(Alignment::Center)
            .max_width(500),
        )
        .into()
    }
}
