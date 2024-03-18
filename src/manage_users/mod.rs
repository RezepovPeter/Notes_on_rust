use crate::structs::User;
use rusqlite::Connection;
use colored::*;
use std::io;

pub fn login(conn: &Connection, username: String, password: String) -> Result<User, io::Error> {
    let mut stmt = conn
        .prepare("SELECT * FROM users WHERE username = ? AND password = ?")
        .expect(format!("{}", "Failed to prepare statement".red()).as_str());

    let rows = stmt
        .query_map([username.trim(), password.trim()], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
            })
        })
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid username or password"))?;

    for row in rows {
        let user = row.unwrap();
        return Ok(user);
    }

    Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid username or password"))
}

pub fn register(conn: &Connection, username: String, password: String) {
    conn.execute("INSERT INTO users (username, password) Values (?1, ?2)", [
        username.trim(),
        password.trim(),
    ]).expect(format!("{}", "Failed to register user".red()).as_str());
}
