//! obs-cutter GUI entry point.
//!
//! This is the main entry point for the graphical user interface version
//! of obs-cutter, built with the iced library.

use iced::{application, Size};

use obs_cutter::gui::App;

fn main() -> iced::Result {
    application("OBS-Cutter", App::update, App::view)
        .theme(App::theme)
        .window_size(Size::new(600.0, 700.0))
        .run_with(App::new)
}
