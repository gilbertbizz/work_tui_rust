use ratatui::{prelude::*, widgets::*};

use crate::app::AppContext;
use crate::components::page_layout::PageLayout;
use crate::components::navbar::NavBar;
use crate::components::footer::Footer;
use crate::styles::theme::THEME;
use crate::pages::home::HomePage;
use crate::pages::email::EmailPage;
use crate::pages::traceroute::TraceRouteTab;
use crate::pages::dashboard::Dashboard;
use crate::styles::constants::{TITLES, KEYS};

pub struct Root<'a> {
    context: &'a AppContext,
}

impl<'a> Root<'a> {
    pub fn new(context: &'a AppContext) -> Self {
        Self {
            context
        }
    }
}

impl Widget for Root<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::new()
            .style(THEME.root)
            .render(area, buf);
        let area = PageLayout::new(area, Direction::Vertical, vec![1, 0, 1]);
        NavBar::new(area[0], buf, &TITLES, self.context);
        self.render_selected_tab(area[1], buf);
        Footer::new(area[2], buf, KEYS);
    }
}

impl Root<'_> {
    fn render_selected_tab(&self, area: Rect, buf: &mut Buffer) {
        let row_index = self.context.row_idx;

        match self.context.tab_idx {
            0 => HomePage::new(row_index).render(area, buf),
            1 => EmailPage::new(row_index).render(area, buf),
            2 => TraceRouteTab::new(row_index).render(area, buf),
            3 => Dashboard::new(row_index).render(area, buf),
            _ => {}
        }
    }
}
