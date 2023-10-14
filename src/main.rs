
extern crate clap;
use clap::{Arg, App, SubCommand};
use std::collections::HashMap;

fn main() {
    let matches = App::new("notekeeper")
        .version("0.1")
        .author("Your Name")
        .about("A simple notekeeping tool in Rust")
        .subcommand(SubCommand::with_name("add")
            .about("Adds a new note")
            .arg(Arg::with_name("NOTE")
                 .required(true)
                 .index(1)
                 .help("The content of the note to add")))
        .subcommand(SubCommand::with_name("view")
            .about("Views all notes"))
        .subcommand(SubCommand::with_name("delete")
            .about("Deletes a note by its ID")
            .arg(Arg::with_name("ID")
                 .required(true)
                 .index(1)
                 .help("The ID of the note to delete")))
        .get_matches();

    let mut notes = HashMap::new();
    let mut next_id = 1;

    match matches.subcommand() {
        ("add", Some(add_matches)) => {
            let content = add_matches.value_of("NOTE").unwrap();
            notes.insert(next_id, content.to_string());
            next_id += 1;
            println!("Note added!");
        },
        ("view", Some(_)) => {
            for (id, note) in &notes {
                println!("ID: {}, Content: {}", id, note);
            }
        },
        ("delete", Some(del_matches)) => {
            let id: usize = del_matches.value_of("ID").unwrap().parse().unwrap();
            if notes.remove(&id).is_some() {
                println!("Note deleted!");
            } else {
                println!("Note not found!");
            }
        },
        _ => unreachable!(),
    }
}
