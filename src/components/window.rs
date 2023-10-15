use std::vec;

use ratatui::{prelude::*, widgets::*};
use crate::{
    app::Context,
    components::page_layout::PageLayout,
    styles::theme::THEME
};

use super::{sidebar::Sidebar, page_content::ContentPage};


pub struct Window<'a> {
    context: &'a Context
}

impl<'a> Window<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context
        }
    }
}

impl Widget for Window<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::new()
            .style(THEME.background)
            .render(area, buf);

        // split screen horizontally
        let area = PageLayout::new(area, Direction::Horizontal, vec![15, 0]);
        Sidebar::new(&self.context).render(area[0], buf);
        ContentPage::new(&self.context).render(area[1], buf);
    }
}

impl Window<'_> {

}
