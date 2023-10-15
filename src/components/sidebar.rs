use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};

use crate::{styles::theme::THEME, app::Context, constants::header::SIDEBAR_TITLES};

use super::page_layout::PageLayout;

pub struct Sidebar<'a> {
    context: &'a Context,
}

impl<'a> Sidebar<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context
        }
    }
}

impl Widget for Sidebar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = PageLayout::new(area, Direction::Vertical, vec![0]);
       render_titles(self.context.row_idx, area[0], buf, &SIDEBAR_TITLES);
    }
}


fn render_titles(selected_row: usize, area: Rect, buf: &mut Buffer, titles: &[&str]) {
         let titles = titles
            .iter()
            .map(|t| {
                ListItem::new(Text::from(*t))
            }).collect_vec();

        let mut list_state = ListState::default().with_selected(Some(selected_row));

        StatefulWidget::render(
            List::new(titles)
                .style(THEME.dashboard.tabs_selected)
                .block(Block::default()
                       .title("Sidebar")
                       .borders(Borders::ALL)
                       .style(Style::default().fg(Color::Gray)))
                .highlight_style(THEME.dashboard.tabs)
                .highlight_symbol(">>"),
             area, 
             buf, 
             &mut list_state);
}
