use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};

use crate::{app::Context, styles::theme::THEME, constants::header::TAB_TITLES};

use super::page_layout::PageLayout;

pub struct TabBar<'a> {
    context: &'a Context,
}

impl<'a> TabBar<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context
        }
    }
}

impl Widget for TabBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = PageLayout::new(area, Direction::Horizontal, vec![0, 45]);
        render_tabs(self.context.tab_idx, area[1], buf, TAB_TITLES);

    }
}


fn render_tabs(tab_index: usize, area: Rect, buf: &mut Buffer, tab_titles: &[&str]) {

        Tabs::new(tab_titles.to_vec())
            .style(THEME.tabs)
            .highlight_style(THEME.tabs_selected)
            .select(tab_index)
            .divider("")
            .render(area, buf)
    }
