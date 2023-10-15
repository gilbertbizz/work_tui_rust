use ratatui::prelude::Color;

// TITLES
pub const TITLES: &[&str] = &["Home", "About", "Contact", "Data"];

// FOOTER KEYS
pub const KEYS: &[(&str, &str)] = &[
    ("Q", "Quit"),
    ("U", "Up"),
    ("D", "Down")
];

// COLORS
pub const DARK_BLUE: Color = Color::Rgb(16, 24, 48);
pub const LIGHT_BLUE: Color = Color::Rgb(64, 96, 192);
pub const LIGHT_YELLOW: Color = Color::Rgb(192, 192, 96);
pub const LIGHT_GREEN: Color = Color::Rgb(64, 192, 96);
pub const LIGHT_RED: Color = Color::Rgb(192, 96, 96);
pub const RED: Color = Color::Indexed(160);
pub const BLACK: Color = Color::Indexed(232);
pub const DARK_GRAY: Color = Color::Indexed(238);
pub const MID_GRAY: Color = Color::Indexed(244);
pub const LIGHT_GRAY: Color = Color::Indexed(250);
pub const WHITE: Color = Color::Indexed(255);

