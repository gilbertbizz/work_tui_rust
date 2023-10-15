#[derive(Clone)]
pub struct Password {
   pub id: usize,
   pub title: String,
   pub username: String,
   pub password: String 
}

impl Password {
    pub fn new(title: String, username: String, password: String) -> Self {
        Self {
            id: 0, 
            title,
            username,
            password
        }
    }

   pub fn new_with_id(id: usize, title: String, username: String, password: String) -> Self {
        Self {
            id,
            title,
            username,
            password
        }
   }
}
