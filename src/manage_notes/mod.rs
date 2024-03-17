use colored::Colorize;
use rusqlite::{ Connection, params };

pub fn add_note(conn: &Connection) {
    let mut tittle = String::new();
    let mut content = String::new();
    let mut stmt = conn
        .prepare("SELECT MAX(id) FROM notes")
        .expect(format!("{}", "Failed to prepare a statement".red()).as_str());
    let max_id = stmt.query_row([], |row| row.get(0)).unwrap_or(0);

    std::io
        ::stdin()
        .read_line(&mut tittle)
        .expect(format!("{}", "Failed to read a string".red()).as_str());
    std::io
        ::stdin()
        .read_line(&mut content)
        .expect(format!("{}", "Failed to read a string".red()).as_str());

    conn.execute(
        "INSERT INTO notes (id, title, content) VALUES (?1, ?2, ?3)",
        params![max_id + 1, tittle.trim(), content.trim()]
    ).expect(format!("{}", "Failed to add note".red()).as_str());
}

pub fn remove_note(conn: &Connection, id: String) {
    conn.execute("DELETE FROM notes WHERE id =?1", params![id.trim()]).expect(
        format!("{}", "Failed to remove note".red()).as_str()
    );
    conn.execute("UPDATE notes SET id = id - 1 WHERE id > ?1", params![id.trim()]).expect(
        format!("{}", "Failed to update ids".red()).as_str()
    );
}
