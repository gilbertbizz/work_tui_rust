use std::{
    io::{self, stdout, Stdout},
    ops::{Deref, DerefMut},
    time::Duration,
};

use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::prelude::*;
use ratatui::Terminal;


pub struct Term {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Term {
    pub fn start() -> Result<Self> {
        let options = TerminalOptions {
            viewport: Viewport::Fixed(Rect::new(0, 0, 81, 18)),
        };

        let terminal = Terminal::with_options(CrosstermBackend::new(io::stdout()), options)?;
        enable_raw_mode().context("enable raw mode")?;
        stdout()
            .execute(EnterAlternateScreen)
            .context("enter alternate screen")?;

        Ok(Self { terminal })
    }

    pub fn stop() -> Result<()> {
        disable_raw_mode().context("disable raw mode")?;
        stdout()
            .execute(LeaveAlternateScreen)
            .context("leave alternate screen")?;

        Ok(())
    }

    pub fn next_event(timeout: Duration) -> io::Result<Option<Event>> {
        if !event::poll(timeout)? {
            return Ok(None);
        }

        let event = event::read()?;
        Ok(Some(event))
    }
}

impl Deref for Term {
    type Target = Terminal<CrosstermBackend<Stdout>>;
    fn deref(&self) -> &Self::Target {
        &self.terminal 
    }
}

impl DerefMut for Term {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        let _ = Term::stop();
    }
}
