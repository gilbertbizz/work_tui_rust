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

// struct TableData {
//     id: u16,
//     name: &'static str,
//     activity: &'static str,
//     production: f64,
// }

enum Page {
    HomePage,
    EmailPage,
    TraceRouteTab 
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
    let items: Vec<ListItem> = TITLES
        .iter()
        .map(|t| {
            ListItem::new(Text::from(*t))
        })
        .collect();

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




//    let data = production.get(selected_row);
  //  let block = Block::new()
  //      .style(THEME.email.body)
    //    .padding(Padding::new(2, 2, 0, 0))
      //  .borders(Borders::LEFT)
       // .border_type(BorderType::Thick);
    //let inner = block.inner(area);
    //block.render(area, buf);

    //match data {
      //  Some(data) => {
        //    let area = PageLayout::new(inner, Direction::Horizontal, vec![15, 0]);
          //  data.render(area[1], buf);
      //  None => {
        //    Paragraph::new("No data").render(inner, buf)
      //  }
   // }

}

