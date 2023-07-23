use std::{fs, process::Command, thread};
fn main() {
    // you can edit "sayori.chr" to "monika.chr" for the other quick ending.
    // Spawn a new thread to delete the file
    thread::spawn(move || {
        match fs::remove_file("characters/sayori.chr") {
            Ok(_) => println!("ok"),
            Err(err) => eprintln!("Error deleting file: {}", err),
        };
    });

    // Launch the game
    Command::new("DDLC.exe");
}
