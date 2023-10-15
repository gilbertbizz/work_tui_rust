use ratatui::{prelude::*, widgets::*};
use crate::styles::colors::RgbSwatch;
use crate::styles::theme::THEME;
use crate::components::page_layout::PageLayout;

pub struct HomePage {
    selected_row: usize
}
impl HomePage {
    pub fn new(row: usize) -> Self {
        Self {
            selected_row: row 
        }
    }
}

impl Widget for HomePage {
    fn render(self, area: Rect, buf: &mut Buffer) {
        RgbSwatch.render(area, buf);
        let area = PageLayout::new(area, Direction::Horizontal, vec![34, 0]);
        render_create_description(area[1], buf);
    }
}

fn render_create_description(area: Rect, buf: &mut Buffer) {
    let area = area.inner(&Margin {
        vertical: 4,
        horizontal: 2,
    });

    Clear.render(area, buf);
    Block::new()
        .style(THEME.content)
        .render(area, buf);
    
    let area = area.inner(&Margin {
        vertical: 1,
        horizontal: 2
    });

    let text = "- ksine leiennd ekdienekdieldek eeidne elkdeiend ";
    Paragraph::new(text)
        .style(THEME.description)
        .block(
            Block::new()
                .title(" GilFarm ")
                .title_alignment(Alignment::Center)
                .borders(Borders::TOP)
                .border_style(THEME.description_title)
                .padding(Padding::new(0, 0, 0, 0))   
        )
        .wrap(Wrap { trim: true })
        .scroll((0, 0))
        .render(area, buf);
}
