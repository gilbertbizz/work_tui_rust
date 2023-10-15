mod db;
mod password_model;
mod app;
mod ui;
mod constants;

use std::error::Error;
use crossterm::{
    execute,
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, Event::Key },
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal
};

use crate::app::{App, InputMode};
use crate::ui::ui;

fn main() -> Result<(), Box<dyn Error>> {
   let passphrase = rpassword::prompt_password("Enter Passphrase: ").unwrap();
   let mut app = App::new(passphrase);

   enable_raw_mode()?;
   execute!(
        std::io::stdout(),
        EnterAlternateScreen,
        EnableMouseCapture
    )?;

   let backend = CrosstermBackend::new(std::io::stdout());
   let mut terminal = Terminal::new(backend)?;

   let result = run_app(&mut terminal, &mut app);

   disable_raw_mode()?;
   execute!(
     terminal.backend_mut(),
     LeaveAlternateScreen,
     DisableMouseCapture
   )?;

   if let Err(e) = result {
        println!("{}", e.to_string());
   };

   Ok(())


}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), std::io::Error> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Key(key) = event::read()? {
            match app.mode {
                InputMode::Normal => {
                    match key.code {
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        KeyCode::Char('s') => {
                            app.change_mode(InputMode::Search);
                        }
                        KeyCode::Char('l') => {
                            app.change_mode(InputMode::List);
                        }
                        KeyCode::Insert => {
                            app.change_mode(InputMode::Title);
                        }
                        _ => {}
                    }
                }
                InputMode::Title => {
                    match key.code {
                        KeyCode::Esc => {
                            app.clear_fields();
                            app.change_mode(InputMode::Normal);
                        }
                        KeyCode::Char(c) => {
                            app.new_title.push(c);
                        }
                        KeyCode::Backspace => {
                            app.new_title.pop();
                        }
                        KeyCode::Tab => {
                            app.change_mode(InputMode::Username);
                        }
                        _ => {}
                    }
                }
                InputMode::Username => {
                    match key.code {
                        KeyCode::Esc => {
                            app.clear_fields();
                            app.change_mode(InputMode::Normal);
                        }
                        KeyCode::Char(c) => {
                            app.new_username.push(c);
                        }
                        KeyCode::Backspace => {
                            app.new_username.pop();
                        }
                        KeyCode::Tab => {
                            app.change_mode(InputMode::Password);
                        }
                        KeyCode::BackTab => {
                            app.change_mode(InputMode::Title);
                        }
                        _ => {}
                    }
                }
                InputMode::Password => {
                    match key.code {
                        KeyCode::Esc => {
                            app.clear_fields();
                            app.change_mode(InputMode::Normal);
                        }
                        KeyCode::Char(c) => {
                            app.new_pass.push(c);
                        }
                        KeyCode::Backspace => {
                            app.new_pass.pop();
                        }
                        KeyCode::Tab => {
                            app.change_mode(InputMode::Submit);
                        }
                        KeyCode::BackTab => {
                            app.change_mode(InputMode::Username);
                        }
                        _ => {}
                    }
                }
                InputMode::Submit => {
                    match key.code {
                        KeyCode::Esc => {
                            app.clear_fields();
                            app.change_mode(InputMode::Normal);
                        }
                        KeyCode::BackTab => {
                            app.change_mode(InputMode::Password);
                        }
                        KeyCode::Enter => {
                            if app.edit_mode {
                                app.edit_password();
                            } else {
                                app.add_password();
                            }
                        }
                        _ => {}
                    }
                }
                InputMode::Search => {
                    match key.code {
                        KeyCode::Esc => {
                            app.change_mode(InputMode::Normal);
                        }
                        KeyCode::Char(c) => {
                            app.search_text.push(c);
                            app.search_password();
                        }
                        KeyCode::Backspace => {
                            app.search_text.pop();
                            app.search_password();
                        }
                        _ => {}
                    }
                }
                InputMode::List => {
                    match key.code {
                        KeyCode::Esc => {
                            app.list_state.select(None);
                            app.change_mode(InputMode::Normal);
                        }
                        KeyCode::Up => {
                            app.move_up();
                        }
                        KeyCode::Down => {
                            app.move_down();
                        }
                        KeyCode::Char('u') => {
                            app.copy_username();
                        }
                        KeyCode::Char('e') => {
                            app.start_edit_mode();
                        }
                        KeyCode::Char('p') => {
                            app.copy_password();
                        }
                        KeyCode::Char('d') => {
                            app.check_delete();
                        }
                        _ => {}
                    }
                }
                InputMode::Delete => {
                    match key.code {
                        KeyCode::Char('n') => {
                            app.change_mode(InputMode::List);
                        }
                        KeyCode::Char('y') => {
                            app.delete_password();
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

