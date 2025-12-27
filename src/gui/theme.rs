//! Custom theme and styling for the OBS-Cutter GUI.

use iced::widget::overlay::menu;
use iced::widget::{button, container, pick_list, progress_bar, text_input};
use iced::{Background, Border, Color, Theme};

/// Color palette for the application.
pub mod colors {
    use iced::Color;

    pub const BACKGROUND: Color = Color::from_rgb(0.11, 0.11, 0.13);
    pub const SURFACE: Color = Color::from_rgb(0.16, 0.16, 0.19);
    pub const SURFACE_LIGHT: Color = Color::from_rgb(0.22, 0.22, 0.26);
    pub const BORDER: Color = Color::from_rgb(0.3, 0.3, 0.35);

    pub const PRIMARY: Color = Color::from_rgb(0.35, 0.55, 0.95);
    pub const PRIMARY_HOVER: Color = Color::from_rgb(0.45, 0.65, 1.0);
    pub const PRIMARY_DARK: Color = Color::from_rgb(0.25, 0.45, 0.85);

    pub const SUCCESS: Color = Color::from_rgb(0.3, 0.75, 0.45);
    pub const WARNING: Color = Color::from_rgb(0.95, 0.7, 0.2);
    pub const DANGER: Color = Color::from_rgb(0.9, 0.35, 0.35);

    pub const TEXT_PRIMARY: Color = Color::from_rgb(0.95, 0.95, 0.97);
    pub const TEXT_SECONDARY: Color = Color::from_rgb(0.65, 0.65, 0.7);
    pub const TEXT_MUTED: Color = Color::from_rgb(0.45, 0.45, 0.5);
}

/// Primary button style (blue, filled).
pub fn primary_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(colors::PRIMARY)),
        text_color: Color::WHITE,
        border: Border {
            color: colors::PRIMARY_DARK,
            width: 1.0,
            radius: 6.0.into(),
        },
        shadow: Default::default(),
    };

    match status {
        button::Status::Active => base,
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(colors::PRIMARY_HOVER)),
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(colors::PRIMARY_DARK)),
            ..base
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.25, 0.25, 0.3))),
            text_color: colors::TEXT_MUTED,
            ..base
        },
    }
}

/// Secondary button style (outlined).
pub fn secondary_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(colors::SURFACE)),
        text_color: colors::TEXT_PRIMARY,
        border: Border {
            color: colors::BORDER,
            width: 1.0,
            radius: 6.0.into(),
        },
        shadow: Default::default(),
    };

    match status {
        button::Status::Active => base,
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(colors::SURFACE_LIGHT)),
            border: Border {
                color: colors::PRIMARY,
                ..base.border
            },
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(colors::SURFACE)),
            ..base
        },
        button::Status::Disabled => button::Style {
            text_color: colors::TEXT_MUTED,
            ..base
        },
    }
}

/// Danger button style (red).
pub fn danger_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(Color::from_rgb(0.6, 0.2, 0.2))),
        text_color: Color::WHITE,
        border: Border {
            color: colors::DANGER,
            width: 1.0,
            radius: 6.0.into(),
        },
        shadow: Default::default(),
    };

    match status {
        button::Status::Active => base,
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(colors::DANGER)),
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.7, 0.25, 0.25))),
            ..base
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.3, 0.2, 0.2))),
            text_color: colors::TEXT_MUTED,
            ..base
        },
    }
}

/// Success button style (green).
pub fn success_button(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(colors::SUCCESS)),
        text_color: Color::WHITE,
        border: Border {
            color: Color::from_rgb(0.2, 0.55, 0.35),
            width: 1.0,
            radius: 6.0.into(),
        },
        shadow: Default::default(),
    };

    match status {
        button::Status::Active => base,
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.35, 0.8, 0.5))),
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.25, 0.65, 0.4))),
            ..base
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(Color::from_rgb(0.2, 0.35, 0.25))),
            text_color: colors::TEXT_MUTED,
            ..base
        },
    }
}

/// Drop zone container style.
pub fn drop_zone(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::SURFACE)),
        border: Border {
            color: colors::BORDER,
            width: 2.0,
            radius: 12.0.into(),
        },
        text_color: Some(colors::TEXT_PRIMARY),
        shadow: Default::default(),
    }
}

/// Card container style.
pub fn card(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::SURFACE)),
        border: Border {
            color: colors::BORDER,
            width: 1.0,
            radius: 8.0.into(),
        },
        text_color: Some(colors::TEXT_PRIMARY),
        shadow: Default::default(),
    }
}

/// File row container style.
pub fn file_row(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(colors::SURFACE_LIGHT)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 6.0.into(),
        },
        text_color: Some(colors::TEXT_PRIMARY),
        shadow: Default::default(),
    }
}

/// Progress bar style.
pub fn progress(_theme: &Theme) -> progress_bar::Style {
    progress_bar::Style {
        background: Background::Color(colors::SURFACE_LIGHT),
        bar: Background::Color(colors::PRIMARY),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 4.0.into(),
        },
    }
}

/// Input field style.
pub fn text_input_style(_theme: &Theme, status: text_input::Status) -> text_input::Style {
    let base = text_input::Style {
        background: Background::Color(colors::SURFACE),
        border: Border {
            color: colors::BORDER,
            width: 1.0,
            radius: 6.0.into(),
        },
        icon: colors::TEXT_SECONDARY,
        placeholder: colors::TEXT_MUTED,
        value: colors::TEXT_PRIMARY,
        selection: colors::PRIMARY,
    };

    match status {
        text_input::Status::Active => base,
        text_input::Status::Hovered => text_input::Style {
            border: Border {
                color: colors::PRIMARY,
                ..base.border
            },
            ..base
        },
        text_input::Status::Focused => text_input::Style {
            border: Border {
                color: colors::PRIMARY,
                width: 2.0,
                ..base.border
            },
            ..base
        },
        text_input::Status::Disabled => text_input::Style {
            background: Background::Color(Color::from_rgb(0.12, 0.12, 0.14)),
            value: colors::TEXT_MUTED,
            ..base
        },
    }
}

/// Pick list (dropdown) style.
pub fn pick_list_style(_theme: &Theme, status: pick_list::Status) -> pick_list::Style {
    let base = pick_list::Style {
        background: Background::Color(colors::SURFACE),
        text_color: colors::TEXT_PRIMARY,
        placeholder_color: colors::TEXT_MUTED,
        handle_color: colors::TEXT_SECONDARY,
        border: Border {
            color: colors::BORDER,
            width: 1.0,
            radius: 6.0.into(),
        },
    };

    match status {
        pick_list::Status::Active => base,
        pick_list::Status::Hovered => pick_list::Style {
            border: Border {
                color: colors::PRIMARY,
                ..base.border
            },
            ..base
        },
        pick_list::Status::Opened => pick_list::Style {
            border: Border {
                color: colors::PRIMARY,
                width: 2.0,
                ..base.border
            },
            ..base
        },
    }
}

/// Pick list menu style.
pub fn pick_list_menu(_theme: &Theme) -> menu::Style {
    menu::Style {
        background: Background::Color(colors::SURFACE),
        text_color: colors::TEXT_PRIMARY,
        border: Border {
            color: colors::BORDER,
            width: 1.0,
            radius: 6.0.into(),
        },
        selected_background: Background::Color(colors::PRIMARY),
        selected_text_color: Color::WHITE,
    }
}
