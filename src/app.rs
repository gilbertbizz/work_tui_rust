use std::io::{Stdout, stdout};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyEvent, KeyEventKind, KeyCode, KeyModifiers}, 
    terminal::{disable_raw_mode, LeaveAlternateScreen, enable_raw_mode, EnterAlternateScreen},
    ExecutableCommand
};
use ratatui::{
    Terminal, 
    prelude::{CrosstermBackend, Rect},
};

use crate::{
    components::window::Window,
    constants::header::TAB_TITLES
};

type Term = Terminal<CrosstermBackend<Stdout>>;

#[derive(Debug, Default, Clone, Copy)]
pub struct Context {
    pub tab_idx: usize,
    pub row_idx: usize,
}

pub struct App {
    terminal: Term,
    context: Context,
    quit: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        let terminal = Self::start_terminal()?;

        Ok(Self {
            terminal,
            context: Context::default(),
            quit: false,
        })

    } 

    fn draw(&mut self) -> Result<()> {
        self.terminal
            .draw(|f| f.render_widget(Window::new(&self.context), f.size()))?;

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
            KeyCode::Tab | KeyCode::BackTab if key.modifiers.contains(KeyModifiers::SHIFT) => {
                let tab_index = self.context.tab_idx + TAB_TITLES.len();
                self.context.tab_idx = tab_index.saturating_sub(1) % TAB_TITLES.len();
                self.context.row_idx = 0;
            }
            KeyCode::Tab | KeyCode::Right => {
                self.context.tab_idx = self.context.tab_idx.saturating_add(1) % TAB_TITLES.len();
                self.context.row_idx = 0;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.context.row_idx = self.context.row_idx.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.context.row_idx = self.context.row_idx.saturating_add(1);
            }
            _  => {}
        }

        Ok(())
    }

    pub fn run() -> Result<()> {
        let mut app = Self::new()?;

        while !app.quit {
            app.draw()?;
            app.handle_events()?;
        }

        Self::stop_terminal()?;

        Ok(())
    }


    fn start_terminal() -> Result<Term> {
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend)?;
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;

        Ok(terminal)
    }

    fn stop_terminal() -> Result<()> {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    fn next_event() -> Result<Option<Event>> {
        let event = event::read()?;

        Ok(Some(event))
    }
}
