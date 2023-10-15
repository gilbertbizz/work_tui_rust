use ratatui::{prelude::*, widgets::*};

use crate::{app::Context, components::page_layout::PageLayout};

pub struct Dashboard<'a> {
    context: &'a Context
}

impl<'a> Dashboard<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context
        }
    }
}

impl Widget for Dashboard<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = PageLayout::new(area, Direction::Vertical, vec![1, 0]);
        render_title(area[0], buf);
        render_vertical_layout(area[1], buf, &self.context);
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    Paragraph::new("Dashboard Page")
        .render(area, buf);
}

fn render_vertical_layout(area: Rect, buf: &mut Buffer, context: &Context) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .direction(Direction::Vertical)
        .split(area);

    render_charts(chunks[0], buf);
    render_table(context.row_idx, chunks[1], buf)

}

fn render_charts(area: Rect, buf: &mut Buffer) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .direction(Direction::Horizontal)
        .split(area);

    let chart1 = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Gray));

   Paragraph::new("Chart 1")
       .block(chart1)
       .render(chunks[0], buf);

   let chart2 = Block::default()
       .borders(Borders::ALL)
       .style(Style::default().fg(Color::Gray));

   Paragraph::new("Chart 2")
       .block(chart2)
       .render(chunks[1], buf);
       
}

fn render_table(selected_row: usize, area: Rect, buf: &mut Buffer) {
       Paragraph::new("understand")
           .block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::Gray))).render(area, buf);
}

