mod manage_notes;
mod manage_users;
mod view_notes;
mod structs;

use std::io;
use rusqlite::*;
use colored::*;
use manage_notes::add_note;
use manage_notes::remove_note;
use structs::Note;
use structs::User;
use manage_users::login;
use manage_users::register;
use view_notes::view_note;
use view_notes::view_all_notes;

fn main() {
    println!("{}", "========A program for notes=========".blue());
    let conn = Connection::open("../notes/database.db").expect(
        format!("{}", "Failed to open database".red()).as_str()
    );
    let notes_create_table = std::fs
        ::read_to_string("../notes/src/notes_create_table.sql")
        .expect(format!("{}", "Failed to read file".red()).as_str());

    let users_create_table = std::fs
        ::read_to_string("../notes/src/users_create_table.sql")
        .expect(format!("{}", "Failed to read file".red()).as_str());

    conn.execute_batch(&notes_create_table).expect(
        format!("{}", "Failed to create notes table".red()).as_str()
    );

    conn.execute_batch(&users_create_table).expect(
        format!("{}", "Failed to create notes table".red()).as_str()
    );

    let mut is_not_logged_in = true;
    let mut user = User { id: 0, username: String::new(), password: String::new() };

    while is_not_logged_in {
        println!("Please\n1 - log in or\n2 - create an account");
        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect(format!("{}", "Failed to read option".red()).as_str());

        let mut username = String::new();
        let mut password = String::new();
        println!("Please enter your username");
        io::stdin()
            .read_line(&mut username)
            .expect(format!("{}", "Failed to read username".red()).as_str());
        println!("Please enter your password");
        io::stdin()
            .read_line(&mut password)
            .expect(format!("{}", "Failed to read password".red()).as_str());

        match option.trim() {
            "1" => {
                user = match login(&conn, username, password) {
                    Ok(user) => user,
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };
                println!("Welcome {}", user.username);
                is_not_logged_in = false;
            }

            "2" => {
                register(&conn, username, password);
            }
            _ => {
                println!("Invalid option");
            }
        }
    }

    loop {
        println!(
            "\nPlease choose an option:\n1 - add a note\n2 - remove note\n3 - view the note\n4 - list notes\n5 - exit\n"
        );
        let mut option = String::new();
        match io::stdin().read_line(&mut option) {
            Ok(_) => {}
            Err(_) => {
                println!("{}", "Failed to read a string".red());
                continue;
            }
        }

        match option.trim() {
            "1" => {
                println!("Enter the title of the note");
                let mut title = String::new();
                std::io
                    ::stdin()
                    .read_line(&mut title)
                    .expect(format!("{}", "Failed to read a string".red()).as_str());
                println!("enter the content of the note");
                let mut content = String::new();
                std::io
                    ::stdin()
                    .read_line(&mut content)
                    .expect(format!("{}", "Failed to read a string".red()).as_str());
                println!("Do you want to make the note private? (y/n)");
                let mut is_private = String::new();
                std::io
                    ::stdin()
                    .read_line(&mut is_private)
                    .expect(format!("{}", "Failed to read a string".red()).as_str());
                match is_private.trim() {
                    "y" => {
                        add_note(&conn, title, content, true, user.id);
                    }
                    "n" => {
                        add_note(&conn, title, content, false, user.id);
                    }
                    _ => {
                        println!("Invalid option");
                    }
                }
            }
            "2" => {
                println!("enter the title of the note you want to delete");
                let mut title = String::new();
                std::io
                    ::stdin()
                    .read_line(&mut title)
                    .expect(format!("{}", "Failed to read a string".red()).as_str());
                remove_note(&conn, title, user.id);
            }
            "3" => {
                println!("enter the title of the note you want to view");
                let mut title = String::new();
                std::io
                    ::stdin()
                    .read_line(&mut title)
                    .expect(format!("{}", "Failed to read a string".red()).as_str());
                let note = match view_note(&conn, title, user.id) {
                    Some(note) => note,
                    None => {
                        println!("{}", "Failed to find the note".red());
                        continue;
                    }
                };
                println!("title: {}\ncontent: {}", note.title, note.content);
            }
            "4" => {
                let all_notes: Vec<Note> = view_all_notes(&conn, user.id);
                for note in all_notes {
                    if !note.is_private || note.author_id == user.id {
                        println!("title: {}", note.title);
                    }
                }
            }
            "5" => {
                break;
            }
            _ => println!("{}", "Invalid option".red()),
        }
    }
}
