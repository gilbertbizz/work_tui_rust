use ratatui::Frame;
use ratatui::{
    widgets::{Block, Borders, BorderType, Clear, List, ListItem, Paragraph},
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::Span,
    style::{Color, Modifier, Style}
};

use crate::app::{App, InputMode};
use crate::constants::APP_KEYS_DESC;


pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50)
            ].as_ref()
        )
        .split(f.size());

    let new_section_block = Block::default()
        .title("New Password")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(new_section_block, chunk[0]);
    new_section(f, app, chunk[0]);

    let list_section_block = Block::default()
        .title("List of passwords")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(list_section_block, chunk[1]);
    list_section(f, app, chunk[1]);

    delete_popup(f, app);
}

fn new_section<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunk = Layout::default()
        .margin(2)
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(4),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3)
            ].as_ref()
        )
        .split(area);

    let description = Paragraph::new(APP_KEYS_DESC);
    f.render_widget(description, chunk[0]);

    let title_input = Paragraph::new(app.new_title.to_owned())
        .block(Block::default().title("Title").borders(Borders::ALL).border_type(BorderType::Rounded))
        .style(match app.mode {
            InputMode::Title => Style::default().fg(Color::Yellow),
            _ => Style::default()
        });
    f.render_widget(title_input, chunk[1]);

    let username_input = Paragraph::new(app.new_username.to_owned())
        .block(Block::default().title("Username").borders(Borders::ALL).border_type(BorderType::Rounded))
        .style(match app.mode {
            InputMode::Username => Style::default().fg(Color::Yellow),
            _ => Style::default()
        });
    f.render_widget(username_input, chunk[2]);

    let password_input = Paragraph::new(app.new_pass.to_owned())
        .block(Block::default().title("Password").borders(Borders::ALL).border_type(BorderType::Rounded))
        .style(match app.mode {
            InputMode::Password => Style::default().fg(Color::Yellow),
            _ => Style::default()
        });
    f.render_widget(password_input, chunk[3]);

    let submit_btn = Paragraph::new("Submit")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded))
        .style(match app.mode {
            InputMode::Submit => Style::default().fg(Color::Yellow),
            _ => Style::default()
        });
    f.render_widget(submit_btn, chunk[4]);
}

fn list_section<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunk = Layout::default()
        .margin(2)
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(1)
            ].as_ref()
        )
        .split(area);

    let list_to_show = if app.search_list.is_empty() {
        app.passwords.to_owned()
    } else {
        app.search_list.to_owned()
    };

    let items: Vec<ListItem> = list_to_show.into_iter()
        .map(|item| {
            match app.mode {
                InputMode::List => {
                    ListItem::new(format!("{}: {} - {}", item.title.to_owned(), item.username.to_owned(), item.password.to_owned()))
                }
                _ => {
                    ListItem::new(Span::from(item.title))
                }
            }
        }).collect();

    let search_input = Paragraph::new(app.search_text.to_owned())
        .block(Block::default().title("Search").borders(Borders::ALL).border_type(BorderType::Rounded))
        .style(match app.mode {
            InputMode::Search => Style::default().fg(Color::Yellow),
            _ => Style::default()
        });
    f.render_widget(search_input, chunk[0]);

    let list = List::new(items)
        .block(Block::default())
        .highlight_symbol("->")
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_stateful_widget(list, chunk[1], &mut app.list_state);
}

fn delete_popup<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    if let InputMode::Delete = app.mode {
        let block = Block::default()
            .title("DELETE")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let area = center_rect(60, 25, f.size());
        f.render_widget(Clear, area);
        f.render_widget(block, area);

        let chunk = Layout::default()
            .margin(2)
            .constraints(
                [
                    Constraint::Length(2),
                    Constraint::Length(2)
                ].as_ref()
            )
            .split(area);

        let text = Paragraph::new("Are your sure?")
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center);
        f.render_widget(text, chunk[0]);

        let keys_desc = Paragraph::new("Press (Y) for Yes and (N) for No")
            .alignment(Alignment::Center);
        f.render_widget(keys_desc, chunk[1]);
    }
}

fn center_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2)
            ].as_ref()
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2)
            ].as_ref()
        )
        .split(chunk[1])[1]
}
