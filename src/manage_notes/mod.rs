use colored::Colorize;
use rusqlite::{ Connection, params };

pub fn add_note(conn: &Connection, title: String, content: String, is_private: bool, user_id: i32) {
    conn.execute(
        "INSERT INTO notes (title, content, is_private, author_id) VALUES (?1, ?2, ?3, ?4)",
        params![title.trim(), content.trim(), is_private, user_id]
    ).expect(format!("{}", "Failed to add note".red()).as_str());
}

pub fn remove_note(conn: &Connection, title: String, user_id: i32) {
    conn.execute(
        "DELETE FROM notes WHERE title =?1 AND author_id = ?2",
        params![title, user_id]
    ).expect(format!("{}", "Failed to remove note".red()).as_str());
}
