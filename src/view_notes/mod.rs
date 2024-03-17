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
        .expect("query_map error");
    for note in notes {
        collected_notes.push(note.unwrap());
    }
    return collected_notes;
}
