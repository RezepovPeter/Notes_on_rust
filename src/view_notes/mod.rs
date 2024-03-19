use crate::structs::Note;
use colored::Colorize;
use rusqlite::Connection;

pub fn view_all_notes(conn: &Connection, user_id: i32) -> Vec<Note> {
    let mut collected_notes: Vec<Note> = Vec::new();
    let mut strt = conn
        .prepare("SELECT * FROM notes")
        .expect(format!("{}", "Failed to prepare statement".red()).as_str());
    let notes = strt
        .query_map([], |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                posted_at: row.get(3)?,
                is_private: row.get(4)?,
                author_id: row.get(5)?,
            })
        })
        .expect(format!("{}", "query_map error".red()).as_str());
    for note in notes {
        let unwrap_note = note.unwrap();
        if !(unwrap_note.is_private && unwrap_note.author_id != user_id) {
            collected_notes.push(unwrap_note);
        }
    }
    return collected_notes;
}

// TODO: fix vulnerable code
pub fn view_note(conn: &Connection, title: String, user_id: i32) -> Option<Note> {
    let mut stmt = conn
        .prepare("SELECT * FROM notes WHERE title = ?1")
        .expect(format!("{}", "Failed to execute statement".red()).as_str());
    let mut notes = stmt
        .query_map([title.trim()], |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                posted_at: row.get(3)?,
                is_private: row.get(4)?,
                author_id: row.get(5)?,
            })
        })
        .expect(format!("{}", "Query failed".red()).as_str());
    if let Some(note) = notes.next() {
        let unwrap_note = note.unwrap();
        if !unwrap_note.is_private || unwrap_note.author_id == user_id {
            return Some(unwrap_note);
        } else {
            return None;
        }
    } else {
        return None;
    }
}
