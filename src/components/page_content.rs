use ratatui::{prelude::*, widgets::*};

use crate::{app::Context, styles::theme::THEME};

use super::{page_layout::PageLayout, header::TabBar, footer::Footer};
use crate::pages::{dashboard::Dashboard, form::FormPage, records::RecordsPage};


pub struct ContentPage<'a> {
    context: &'a Context,
}

impl<'a> ContentPage<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context
        }
    }
}

impl Widget for ContentPage<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = PageLayout::new(area, Direction::Vertical, vec![1, 0, 3]);
        TabBar::new(self.context).render(area[0], buf);
        main_page(self.context, area[1], buf);
        Footer.render(area[2], buf);
    }
}

fn main_page(context: &Context, area: Rect, buf: &mut Buffer) {
    let pages: Vec<Box<dyn Widget>> = vec![
        Box::new(Dashboard::new(context)),
        Box::new(FormPage::new(context)),
        Box::new(RecordsPage::new(context))
    ];       

    let selected_page = pages.get(context.row_idx);
    
    let block = Block::default()
        .style(THEME.dashboard.tabs)
        .padding(Padding::new(2, 2, 0, 0));

    let inner_layout = block.inner(area);
    block.render(area, buf);

    let area = PageLayout::new(inner_layout, Direction::Horizontal, vec![0]);
    if let Some(_) = selected_page {
        match context.row_idx {
            0 => Dashboard::new(context).render(area[0], buf),
            1 => FormPage::new(context).render(area[0], buf),
            2 => RecordsPage::new(context).render(area[0], buf),
            _ => Paragraph::new("No Page").render(area[0], buf),
        }
    } else {
        Paragraph::new("No Page").render(area[0], buf)
    };
}
