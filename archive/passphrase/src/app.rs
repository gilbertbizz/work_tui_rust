use crate::{db::Database, password_model::Password};
use ratatui::widgets::ListState;
use rusqlite::ErrorCode;

pub enum InputMode {
    Normal,
    Title,
    Username,
    Password,
    Submit,
    Search,
    List,
    Delete
}

pub struct App {
    pub db: Database,
    pub mode: InputMode,
    pub list_state: ListState,
    pub passwords: Vec<Password>,
    pub search_text: String,
    pub search_list: Vec<Password>,
    pub new_title: String,
    pub new_username: String,
    pub new_pass: String,
    pub edit_mode: bool,
    pub edit_index: Option<usize>
}

impl App {
    pub fn new(key: String) -> Self {
        let db = match Database::new(key) {
            Ok(db) => db,
            Err(e) => {
                if e.sqlite_error_code().unwrap() == ErrorCode::NotADatabase {
                    println!("passphrase is not valid");
                    std::process::exit(1);
                } else {
                    println!("{}", e.to_string());
                    std::process::exit(1);
                }
            }
        };

        let passwords = db.load();

        Self {
            db,
            mode: InputMode::Normal,
            list_state: ListState::default(),
            passwords,
            search_text: String::new(),
            search_list: vec![],
            new_title: String::new(),
            new_username: String::new(),
            new_pass: String::new(),
            edit_mode: false,
            edit_index: None
        }
    }

    pub fn change_mode(&mut self, mode: InputMode) {
        self.mode = mode;
    }

    pub fn clear_fields(&mut self) {
        self.new_title.clear();
        self.new_username.clear();
        self.new_pass.clear();
    }

    pub fn add_password(&mut self) {
        let password = Password::new(
                self.new_title.to_owned(),
                self.new_username.to_owned(),
                self.new_pass.to_owned()
            );

        self.db.insert(&password);
        self.passwords.push(password);
        self.clear_fields();
        self.change_mode(InputMode::Normal);
    }

    pub fn start_edit_mode(&mut self) {
        if let Some(index) = self.list_state.selected() {
            let password = &self.passwords[index];

            self.new_title = password.title.to_owned();
            self.new_username = password.username.to_owned();
            self.new_pass = password.password.to_owned();
            self.edit_mode = true;
            self.edit_index = Some(index);
            self.change_mode(InputMode::Title);
        }
    }

    pub fn edit_password(&mut self) {
        let index = self.edit_index.unwrap();
        let id = self.passwords[index].id;
        let password = Password::new(
                self.new_title.to_owned(),
                self.new_username.to_owned(),
                self.new_pass.to_owned()
            );

        self.db.update(id, &password);
        self.passwords[index] = password;
        self.end_edit_mode();
        self.change_mode(InputMode::List);
    }

    pub fn end_edit_mode(&mut self) {
        if self.edit_mode {
            self.edit_mode = false;
            self.edit_index = None;
        }
    }

    pub fn check_delete(&mut self) {
        if self.list_state.selected().is_some() {
            self.change_mode(InputMode::Delete);
        }
    }

    pub fn delete_password(&mut self) {
        if let Some(index) = self.list_state.selected() {
            let id = self.passwords[index].id;
            self.passwords.remove(index);
            self.db.delete(id);

            if index > 0 {
                self.list_state.select(Some(0));
            } else {
                self.list_state.select(None);
            }

            self.change_mode(InputMode::List);
        }
    }

    pub fn search_password(&mut self) {
        self.search_list = self.passwords.clone().into_iter()
            .filter(|item| item.title.starts_with(&self.search_text.to_owned()))
            .collect();
    }

    pub fn copy(content: String) {
        let mut clipboard = arboard::Clipboard::new().unwrap();
        clipboard.set_text(content).unwrap();
    }

    pub fn copy_username(&self) {
        if let Some(index) = self.list_state.selected() {
            let username = self.passwords[index].username.to_owned();
            App::copy(username);
        }
    }

    pub fn copy_password(&self) {
        if let Some(index) = self.list_state.selected() {
            let password = self.passwords[index].password.to_owned();

            App::copy(password);
        }
    }


    pub fn move_up(&mut self) {
        let selected = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    Some(i)
                } else {
                    Some(i - 1)
                }
            }
            None => { Some(0) }
        };
        self.list_state.select(selected);
    }

    pub fn move_down(&mut self) {
        let selected = match self.list_state.selected() {
            Some(i) => {
                if i == self.passwords.len() - 1 {
                    Some(i)
                } else {
                    Some(i + 1)
                }
            }
            None => { Some(0) }
        };

        self.list_state.select(selected);
    }
}
