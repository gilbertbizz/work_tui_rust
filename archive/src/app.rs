use std::io::{self, Stdout, stdout};

use anyhow::{Result, Context, Ok};
use crossterm::{
    event::{self, Event, KeyEvent, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen, EnterAlternateScreen}, 
    ExecutableCommand
};
use ratatui::{Terminal, prelude::*};

use crate::pages::root::Root;
use crate::styles::constants::TITLES;


type Term = Terminal<CrosstermBackend<Stdout>>;

#[derive(Debug, Default, Clone, Copy)]
pub struct AppContext {
   pub row_idx: usize,
   pub tab_idx: usize,
}

pub struct App {
    terminal: Term,
    context: AppContext,
    quit: bool,
}

impl App {

    pub fn new() -> Result<Self> {
        let terminal = Self::start_term()?;

        Ok(
            Self { 
                terminal, 
                context: AppContext::default(), 
                quit: false,
             })
    }

    pub fn run_app() -> Result<()> {
        let mut app = Self::new()?;

        while !app.quit {
            app.draw()?;
            app.handle_events()?;
        }
        
        Self::stop_term()?;
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        self.terminal
            .draw(|f| f.render_widget(Root::new(&self.context), f.size()))
            .context("terminal drawing")?;

        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        match Self::next_event()? {
            Some(Event::Key(key)) => self.handle_key_event(key),
            Some(Event::Resize(width, height)) => {
                Ok(self.terminal.resize(Rect::new(0, 0, width, height))?)
            }
            _ => Ok(())
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        if key.kind != KeyEventKind::Press {
            return Ok(());
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.quit = true;
            }
            KeyCode::Tab | KeyCode::BackTab if key.modifiers.contains(KeyModifiers::SHIFT)  => {
               let tab_index = self.context.tab_idx + TITLES.len();
               self.context.tab_idx = tab_index.saturating_sub(1) % TITLES.len();
               self.context.row_idx = 0;
            }
            KeyCode::Tab | KeyCode::Right => {
                self.context.tab_idx = self.context.tab_idx.saturating_add(1) % TITLES.len();
                self.context.row_idx = 0;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.context.row_idx = self.context.row_idx.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.context.row_idx = self.context.row_idx.saturating_add(1);
            }
            _ => {}
        }

        Ok(())
    }

    fn start_term() -> Result<Term> {
        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend)?;
        enable_raw_mode().context("enable raw mode")?;
        stdout()
            .execute(EnterAlternateScreen)
            .context("enter alternate screen")?;

        Ok(terminal)
    }

    fn stop_term() -> Result<()> {
        disable_raw_mode().context("disable raw mode")?;
        stdout()
            .execute(LeaveAlternateScreen)
            .context("leave alternate screen")?;

        Ok(())
    }

     fn next_event() -> anyhow::Result<Option<Event>> {
        let event = event::read()?;
        Ok(Some(event))
    }
}
