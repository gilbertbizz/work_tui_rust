mod ui;
mod components;
mod app;


use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, DisableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::prelude::*;

use crate::app::App;
use crate::ui::ui;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_terminal()?;

    Ok(())
}


fn setup_terminal() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it 
    let app = App::new("Application Interface");
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }
    

    Ok(())
}

fn run_app<B: Backend>(
        terminal: &mut Terminal<B>,
        mut app: App
    ) -> std::io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

       if let Event::Key(key) = event::read()? {
           if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Left => app.tab.previous(),
                    KeyCode::Right => app.tab.next(),
                    _ => {}
                }
           } 
       } 
    }
}


