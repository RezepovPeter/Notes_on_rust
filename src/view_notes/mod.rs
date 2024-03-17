use crate::note_struct::Note;
use colored::Colorize;
use rusqlite::Connection;

pub fn view_all_notes(conn: &Connection) -> Vec<Note> {
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
            })
        })
        .expect(format!("{}", "query_map error".red()).as_str());
    for note in notes {
        collected_notes.push(note.unwrap());
    }
    return collected_notes;
}

pub fn view_note(conn: &Connection, id: String) -> Note {
    let mut stmt = conn
        .prepare("SELECT * FROM notes WHERE id = ?1")
        .expect(format!("{}", "Failed to execute statement".red()).as_str());
    let mut notes = stmt
        .query_map([id], |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
            })
        })
        .expect(format!("{}", "Query failed".red()).as_str());

    return notes.next().unwrap().expect(format!("{}", "Failed to take a note".red()).as_str());
}
