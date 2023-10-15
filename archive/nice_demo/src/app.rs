use std::time::Duration;

use anyhow::{Context, Result};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::prelude::Rect;

use crate::{root, term::Term};


pub struct App {
    term: Term,
    should_quit: bool,
    context: AppContext,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct AppContext {
    pub tab_index: usize,
    pub row_index: usize,
}

impl App {
    fn new() -> Result<Self> {
        Ok(Self {
            term: Term::start()?,
            should_quit: false,
            context: AppContext::default(),
        })
    }

    pub fn run() -> Result<()> {
         let mut app = Self::new()?;
         while !app.should_quit {
           app.draw()?;
           app.handle_events()?;
         }
        Term::stop()?;

        Ok(())
    }

    pub fn draw(&mut self) -> Result<()> {
        self.term
            .draw(|f| f.render_widget(root::Root::new(&self.context), f.size()))
            .context("terminal.draw")?;

        Ok(())
    }

    pub fn handle_events(&mut self) -> Result<()> {
        match Term::next_event(Duration::from_millis(16))? {
            Some(Event::Key(key)) => self.handle_key_event(key),
            Some(Event::Resize(width, height)) => {
               Ok(self.term.resize(Rect::new(0, 0, width, height))?) 
            }
            _ => Ok(())

        }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
       if key.kind != KeyEventKind::Press {
            return Ok(());
       }

       let context = &mut self.context;
       const TAB_COUNT: usize = 5;

       match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Tab | KeyCode::BackTab if key.modifiers.contains(KeyModifiers::SHIFT) => {
                let tab_index = context.tab_index + TAB_COUNT;
                context.tab_index = tab_index.saturating_sub(1) % TAB_COUNT;
                context.row_index = 0;
            }
            KeyCode::Tab | KeyCode::BackTab => {
                context.tab_index = context.tab_index.saturating_add(1) % TAB_COUNT;
                context.row_index = 0;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                context.row_index = context.row_index.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                context.row_index = context.row_index.saturating_add(1);
            }
            _ => {}
       }

       Ok(())
    }
}



