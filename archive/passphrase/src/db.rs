use rusqlite::{Connection, DatabaseName, params};
use crate::password_model::Password;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(key: String) -> Result<Database, rusqlite::Error> {
        let path = dirs::config_dir().unwrap().join("my_passmng");
        let conn = Connection::open(path)?;

        conn.pragma_update(Some(DatabaseName::Main), "KEY", key)?;
        let db = Database { conn };

        db.create_table()?;

        Ok(db)
    }

    pub fn create_table(&self) -> Result<(), rusqlite::Error> {
        self.conn.execute_batch("
            CREATE TABLE IF NOT EXISTS passwords(
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                username TEXT NOT NULL,
                password TEXT NOT NULL
            )        
        ")?;

        Ok(())
    }

    pub fn load(&self) -> Vec<Password> {
        let mut statement = self.conn.prepare("
                    SELECT * FROM passwords
                ").unwrap();
        let items: Vec<Password> = statement.query_map([], |row| {
            let password = Password::new_with_id(
                row.get("id").unwrap(),
                row.get("title").unwrap(),
                row.get("username").unwrap(),
                row.get("password").unwrap());

            Ok(password)
        }).unwrap().map(|i| i.unwrap()).collect();

        items
    }

    pub fn insert(&self, password: &Password) {
        self.conn.execute(
            "INSERT INTO passwords(title, username, password) VALUES(?1, ?2, ?3)",
            params![password.title, password.username, password.password]).unwrap();
    }

    pub fn update(&self, id: usize, password: &Password) {
        self.conn.execute(
            "UPDATE passwords SET title=?1, username=?2, password=?3 WHERE id=?4", 
            params![password.title, password.username, password.password, id]).unwrap();
    }

    pub fn delete(&self, id: usize) {
        self.conn.execute(
            "DELETE FROM passwords WHERE id=?1",
            params![id]).unwrap();
    }
}
