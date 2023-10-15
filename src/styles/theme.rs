use ratatui::prelude::{Style, Color, Modifier};

use crate::constants::colors::*;

pub struct Theme {
    pub background: Style,
    pub app_title: Style,
    pub tabs: Style,
    pub tabs_selected: Style,
    pub key_binding: KeyBinding,
    pub dashboard: Dashboard,
    pub form: Form,
    pub records: Records
}

pub struct KeyBinding {
    pub key: Style,
    pub description: Style,
}

pub struct Dashboard {
    pub tabs: Style,
    pub tabs_selected: Style,
}

pub struct Form {
    pub selected: Style,
    pub header: Style,
}

pub struct Records {
    pub selected: Style
}

pub const THEME: Theme = Theme {
    background: Style::new()
        .bg(JUNGLE_GREEN),
    app_title: Style::new()
        .fg(WHITE)
        .bg(JUNGLE_GREEN)
        .add_modifier(Modifier::BOLD),
    tabs: Style::new()
        .fg(MID_GRAY)
        .bg(JUNGLE_GREEN),
    tabs_selected: Style::new()
        .fg(WHITE)
        .bg(JUNGLE_GREEN)
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::REVERSED),
    key_binding: KeyBinding { 
        key: Style::new()
            .fg(BLACK)
            .bg(DARK_GRAY), 
        description: Style::new()
            .fg(DARK_GRAY)
            .bg(BLACK),
     },
     dashboard: Dashboard { 
        tabs: Style::new()
            .fg(MID_GRAY)
            .bg(JUNGLE_GREEN), 
        tabs_selected: Style::new()
            .fg(BLACK)
            .bg(JUNGLE_GREEN),
     },
     form: Form { 
        selected: Style::new()
            .fg(KELLY_GREEN), 
        header: Style::new()
            .bg(JUNGLE_GREEN)
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::UNDERLINED),
     },
     records: Records { 
        selected: Style::new()
            .fg(KELLY_GREEN),
     }

};