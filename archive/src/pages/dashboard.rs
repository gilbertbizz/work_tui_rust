use itertools::Itertools;
use ratatui::{
    prelude::*,
    widgets::*
};
use crate::components::page_layout::PageLayout;
use crate::styles::theme::THEME;
use crate::pages::email::EmailPage;
use crate::pages::home::HomePage;
use crate::pages::traceroute::TraceRouteTab;

const TITLES: &[&str] = &[
    "Home",
    "Email",
    "Route",
];

enum NewPage {
    Home(HomePage),
    Email(EmailPage),
    TraceRoute(TraceRouteTab)
}

pub struct Dashboard {
    selected_row: usize
}

impl Dashboard {
    pub fn new(selected_row: usize) -> Self {
        Self {
            selected_row
        }
    }
}

impl Widget for Dashboard {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = PageLayout::new(area, Direction::Horizontal, vec![15, 0]);
        render_sidebar(self.selected_row, area[0], buf);
        render_content(self.selected_row, area[1], buf); 
    }
}

fn render_sidebar(selected_row: usize, area: Rect, buf: &mut Buffer) {
    let area = PageLayout::new(area, Direction::Vertical, vec![1, 0]);
    let items = TITLES
        .iter()
        .map(|t| {
            ListItem::new(Text::from(*t))
        }).collect_vec();

    let mut state = ListState::default().with_selected(Some(selected_row));
      StatefulWidget::render(List::new(items)
                           .style(THEME.email.inbox)
                           .highlight_style(THEME.email.selected_item)
                           .highlight_symbol(">>"), 
                           area[1],
                           buf, 
                           &mut state,
                        );
}

fn render_content(selected_row: usize, area: Rect, buf: &mut Buffer) {
    let pages = &[
        NewPage::Home(HomePage::new(selected_row)),
        NewPage::Email(EmailPage::new(selected_row)),
        NewPage::TraceRoute(TraceRouteTab::new(selected_row))
    ];

    let page = pages.get(selected_row);

    let block = Block::new()
        .style(THEME.email.body)
        .padding(Padding::new(2, 2, 0, 0))
        .borders(Borders::LEFT)
        .border_type(BorderType::Thick);

    let inner = block.inner(area);
    block.render(area, buf);

    match page {
        Some(d) => {
            let area = PageLayout::new(inner, Direction::Horizontal, vec![1, 0]);
            
            match d {
                NewPage::Home(_) => HomePage::new(selected_row).render(area[1], buf),
                NewPage::Email(_) => EmailPage::new(selected_row).render(area[1], buf),
                NewPage::TraceRoute(_) => TraceRouteTab::new(selected_row).render(area[1], buf)
            }
        }
        None => {
             Paragraph::new("Empty Page. Move up").render(inner, buf)
        }
    }

}

/* This also works */

// fn render_content(selected_row: usize, area: Rect, buf: &mut Buffer) {
//     let page: Vec<Box<dyn Widget>> = vec![
//         Box::new(HomePage::new(selected_row)),
//         Box::new(EmailPage::new(selected_row)),
//         Box::new(TraceRouteTab::new(selected_row))
//     ];

//     let d = page.get(selected_row);

//     let block = Block::new()
//         .style(THEME.email.body)
//         .padding(Padding::new(2, 2, 0, 0))
//         .borders(Borders::LEFT)
//         .border_type(BorderType::Thick);

//     let inner = block.inner(area);
//     block.render(area, buf);

//     match d {
//        Some(_) => {
//         let area = PageLayout::new(inner, Direction::Horizontal, vec![1, 0]);
//             match selected_row {
//                 0 => HomePage::new(selected_row).render(area[1], buf),
//                 1 => EmailPage::new(selected_row).render(area[1], buf),
//                 2 => TraceRouteTab::new(selected_row).render(area[1], buf),
//                 _ => HomePage::new(selected_row).render(area[1], buf)
//             }
//        },
//        None => {
//             HomePage::new(selected_row).render(inner, buf)
//        }
//     }
// }