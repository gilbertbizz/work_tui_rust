use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};

use crate::download::{Downloads, NUM_DOWNLOADS};

pub fn ui<B: Backend>(f: &mut Frame<B>, downloads: &Downloads) {
    let size = f.size();

    let block = Block::default()
        .title(block::Title::from("Progress").alignment(Alignment::Center));
    f.render_widget(block, size);

    let chunks = Layout::default()
        .constraints([Constraint::Length(2), Constraint::Length(4)].as_ref())
        .margin(1)
        .split(size);
    
    let done = NUM_DOWNLOADS - downloads.pending.len() - downloads.in_progress.len();
    let progress = LineGauge::default()
        .gauge_style(Style::default().fg(Color::Blue))
        .label(format!("{done}/{NUM_DOWNLOADS}"))
        .ratio(done as f64 / NUM_DOWNLOADS as f64);
    f.render_widget(progress, chunks[0]);

    let chunk2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .margin(1)
        .split(chunks[1]);

    let items = downloads
        .in_progress
        .values()
        .map(|download| {
            ListItem::new(Line::from(vec![
                Span::raw(symbols::DOT),
                Span::styled(
                    format!(" download {:>2}", download.id),
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD)
                ),
                Span::raw(format!(" ({}ms)", download.started_at.elapsed().as_millis()))
            ]))
        }).collect_vec();

    let list = List::new(items);
    f.render_widget(list, chunk2[0]);

    for (i, (_, download)) in downloads.in_progress.iter().enumerate() {
        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(Color::Yellow))
            .ratio(download.progress / 100.0);
        if chunk2[1].top().saturating_add(i as u16) > size.bottom() {
                continue;
        }
        f.render_widget(gauge, Rect {
            x: chunk2[1].left(),
            y: chunk2[1].top().saturating_add(i as u16),
            width: chunk2[1].width,
            height: 1,
        });
    }
    
}
