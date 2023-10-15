use ratatui::{prelude::*, widgets::*};

use crate::components::page_layout::PageLayout;
use crate::styles::theme::THEME;
use crate::app::AppContext;

pub struct NavBar;

impl NavBar {
    pub fn new(area: Rect, buf: &mut Buffer, titles: &[&str], context: &AppContext) {
        let area = PageLayout::new(area, Direction::Horizontal, vec![0, 45]);
        
        Paragraph::new(Span::styled("Gilbert Designs", THEME.app_title))
            .render(area[0], buf);

        Tabs::new(titles.to_vec())
            .style(THEME.tabs)
            .highlight_style(THEME.tabs_selected)
            .select(context.tab_idx)
            .divider("")
            .render(area[1], buf);
    }
}