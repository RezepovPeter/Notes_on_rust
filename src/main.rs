mod manage_notes;
mod view_notes;
mod note_struct;

use std::io;
use rusqlite::*;
use colored::*;
use manage_notes::add_note;
use manage_notes::remove_note;
use note_struct::Note;
//use view_notes::view_note;
use view_notes::view_all_notes;

fn main() {
    println!("{}", "========A program for notes=========".blue());
    let conn = Connection::open("../notes/database.db").expect(
        format!("{}", "Failed to open database").as_str()
    );

    conn.execute(
        "
    CREATE TABLE IF NOT EXISTS notes (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL,
        content TEXT NOT NULL
    );",
        []
    ).expect("Failed to create table");

    loop {
        println!(
            "\nPlease choose an option:\n1 - add a note\n2 - remove note\n3 - view the note\n4 - list notes\n5 - exit\n"
        );
        let mut option = String::new();
        match io::stdin().read_line(&mut option) {
            Ok(_) => {
                println!("{}", "Succesful reading".green());
            }
            Err(_) => {
                println!("{}", "Failed to read a string".red());
                continue;
            }
        }

        match option.trim() {
            "1" => {
                println!("Enter the title of the note, then enter the content of the note");
                add_note(&conn);
            }
            "2" => {
                println!("enter the id of the note you want to delete");
                remove_note(&conn);
            }
            //"3" => view_note(&conn),
            "4" => {
                let all_notes: Vec<Note> = view_all_notes(&conn);
                for note in all_notes {
                    println!("{} - {}", note.id, note.title);
                }
            }
            "5" => {
                break;
            }
            _ => println!("{}", "Invalid option".red()),
        }
    }
}
