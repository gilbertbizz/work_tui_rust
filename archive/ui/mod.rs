use ratatui::{prelude::*, widgets::*};
use crate::app::App;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .direction(Direction::Vertical)
        .split(size);

    let background = Block::new()
        .style(Style::default().bg(Color::Black));
    f.render_widget(background, size);

        tab_navigation(f, app, chunks[0]);
        main_content(f, app, chunks[1]);
    }


fn tab_navigation<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints([Constraint::Min(0)].as_ref())
        .split(area);

    let titles: Vec<Line> = app.tab.titles
        .iter()
        .map(|t| {
            Line::from(*t)
        }).collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Navigations"))
        .select(app.tab.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(Color::White));

    f.render_widget(tabs, chunks[0]);

}

fn main_content<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints([Constraint::Min(0)].as_ref())
        .split(area);

    match app.tab.index {
        0 => display_data_table(f, app, chunks[1]),
        1 => display_list(f, app, chunks[1]),
        2 => display_text(f, app, chunks[1]),
        _ => unreachable!()
    };

}


fn display_data_table<'a, B: Backend,>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints([Constraint::Min(0)].as_ref())
        .direction(Direction::Vertical)
        .split(area);

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    let headers = ["ID", "Name 1", "Name 2", "Name 3", "Name 4", "Name 5"]
            .iter()
            .map(|h| {
                Cell::from(*h).style(Style::default().fg(Color::Red))
            });

    let header = Row::new(headers)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);

    let rows = app.table.items
            .iter()
            .map(|item| {
                let height = item.iter()
                    .map(|content| content.chars().filter(|c| *c == '\n').count())
                    .max()
                    .unwrap_or(0)
                    + 1;

               let cells = item.iter().map(|c| Cell::from(*c));
               Row::new(cells).height(height as u16).bottom_margin(1)
            });

    let table = Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title("Table"))
            .highlight_style(selected_style)
            .highlight_symbol("> ");

    f.render_stateful_widget(table, chunks[0], &mut app.table.state);

    
}

fn display_list<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect)  {
    let chunks = Layout::default()
        .constraints([].as_ref())
        .split(area);

 
}

fn display_text<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {

}