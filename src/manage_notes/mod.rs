use colored::Colorize;
use rusqlite::{ Connection, params };

pub fn add_note(conn: &Connection) {
    let mut tittle = String::new();
    let mut content = String::new();

    std::io
        ::stdin()
        .read_line(&mut tittle)
        .expect(format!("{}", "Failed to read a string".red()).as_str());
    std::io
        ::stdin()
        .read_line(&mut content)
        .expect(format!("{}", "Failed to read a string".red()).as_str());

    conn.execute(
        "INSERT INTO notes (title, content) VALUES (?1, ?2)",
        params![tittle.trim(), content.trim()]
    ).expect("Failed to add note");
}

pub fn remove_note(conn: &Connection, id: String) {
    conn.execute("DELETE FROM notes WHERE id =?1", params![id.trim()]).expect(
        "Failed to remove note"
    );

    recalculate_ids(conn, id);
}

fn recalculate_ids(conn: &Connection, deleted_id: String) {
    let mut stmt = conn
        .prepare(
            "UPDATE notes SET id = (SELECT rowid - 1 FROM notes WHERE rowid > ?) WHERE rowid > ?"
        )
        .expect("Unable to prepare statement while recalculating ids");
    stmt.execute(&[&deleted_id, &deleted_id]).expect("failed to recalculate ids");
}
