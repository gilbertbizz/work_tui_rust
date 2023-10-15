use ratatui::{prelude::*, widgets::*};

use crate::{app::Context, components::page_layout::PageLayout};

pub struct FormPage<'a> {
    context: &'a Context
}

impl<'a> FormPage<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context
        }
    }
}

impl Widget for FormPage<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
       let area = PageLayout::new(area, Direction::Vertical, vec![0]);

       render_title(area[0], buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    Paragraph::new("Form Page")
        .render(area, buf);
}
