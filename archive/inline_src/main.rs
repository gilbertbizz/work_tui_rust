use std::{sync::mpsc, time::{Duration, Instant}, thread, error::Error};

use download::{Event, Worker, Downloads, workers, downloads};
use ratatui::{prelude::{Backend, CrosstermBackend}, Terminal, widgets::{Paragraph, Widget}, text::{Line, Span}, style::{Style, Modifier}, TerminalOptions};

mod download;
mod ui;


fn main() -> Result<(), Box<dyn Error>> {
    crossterm::terminal::enable_raw_mode()?;
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::with_options(
        backend, TerminalOptions {
            viewport: ratatui::Viewport::Inline(8)
        })?;

    let (tx, rx) = mpsc::channel();
    input_handling(tx.clone());
    let workers = workers(tx);
    let mut downloads = downloads();

    for w in &workers {
        let d = downloads.next(w.id).unwrap();
        w.tx.send(d).unwrap();
    }

    run_app(&mut terminal, workers, downloads, rx)?;

    crossterm::terminal::disable_raw_mode()?;
    terminal.clear()?;

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    workers: Vec<Worker>,
    mut downloads: Downloads,
    rx: mpsc::Receiver<Event>,
) -> Result<(), Box<dyn Error>> {
   let mut redraw = true;

   loop {
        if redraw {
            terminal.draw(|f| ui::ui(f, &downloads))?;
        }
        redraw = true;

        match rx.recv()? {
            Event::Input(event) => {
                if event.code == crossterm::event::KeyCode::Char('q') {
                    break;
                }
            }
            Event::Resize => {
                terminal.autoresize()?;
            }
            Event::Tick => {}
            Event::DownloadUpdate(worker_id, _download_id, progress) => {
                let download = downloads.in_progress.get_mut(&worker_id).unwrap();
                download.progress = progress;
                redraw = false 
            }
            Event::DownloadDone(worker_id, download_id) => {
                let download = downloads.in_progress.remove(&worker_id).unwrap();
                terminal.insert_before(1, |buf| {
                   Paragraph::new(Line::from(vec![
                        Span::from("Finished "),
                        Span::styled(
                            format!("download {download_id}"), 
                            Style::default().add_modifier(Modifier::BOLD)),
                        Span::from(format!(" in {}ms", download.started_at.elapsed().as_millis()))
                   ]))
                    .render(buf.area, buf);
                })?;
                match downloads.next(worker_id) {
                    Some(d) => workers[worker_id].tx.send(d).unwrap(),
                    None => {
                        if downloads.in_progress.is_empty() {
                            terminal.insert_before(1, |buf| {
                                Paragraph::new("Done!").render(buf.area, buf);
                            })?;
                            break;
                        }
                    }
                }
            }
        }
   }

    Ok(())
}

fn input_handling(tx: mpsc::Sender<Event>) {
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();

        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if crossterm::event::poll(timeout).unwrap() {
                match crossterm::event::read().unwrap() {
                     crossterm::event::Event::Key(key) => tx.send(Event::Input(key)).unwrap(),
                     crossterm::event::Event::Resize(_, _) => tx.send(Event::Resize).unwrap(),
                     _ => {}
                };
            }

            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });
}
