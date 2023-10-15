use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};

use crate::styles::theme::THEME;
pub struct Footer;

impl Footer {
    pub fn new(area: Rect, buf: &mut Buffer, keys: &[(&str, &str)]) {
        let spans = keys
            .iter()
            .flat_map(|(key, desc)| {
                let key = Span::styled(format!(" {} ", *key), THEME.key_binding.key);
                let desc = Span::styled(format!(" {} ", *desc), THEME.key_binding.description);

                [key, desc]
            }).collect_vec();
        
        Paragraph::new(Line::from(spans))
            .alignment(Alignment::Center)
            .fg(Color::Indexed(236))
            .bg(Color::Indexed(232))
            .render(area, buf);
    }
}