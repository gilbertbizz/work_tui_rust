use ratatui::{prelude::*, widgets::*};

use crate::{app::Context, components::page_layout::PageLayout};

pub struct RecordsPage<'a> {
    context: &'a Context
}

impl<'a> RecordsPage<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context
        }
    }
}

impl Widget for RecordsPage<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = PageLayout::new(area, Direction::Vertical, vec![0]);

        render_title(area[0], buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    Paragraph::new("Records Page")
        .render(area, buf);
}
