pub struct Note {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub posted_at: String,
    pub is_private: bool,
    pub author_id: i32,
}

pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}
