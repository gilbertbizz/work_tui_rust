use ratatui::{prelude::*, widgets::*};

use super::page_layout::PageLayout;

pub struct Footer;

impl Widget for Footer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = PageLayout::new(area, Direction::Horizontal, vec![0]);
        render_footer(area[0], buf);
    }
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Gray));

    Paragraph::new("Footer")
        .block(block)
        .render(area, buf);
    
}



