use std::{fs, process::Command, thread};
fn main() {
	// Spawn a new thread to delete the file, while the main thread instantly launches DDLC.exe.
	// N.B. this code may or may not start DDLC.exe before the file is deleted, but I pray to the developers of the filesystem drivers and the NT kernel that they coded this one specific function properly. Of course, the NT kernel is designed better than the rest of this OS, obviously.
	thread::spawn(move || {
		// You can edit "sayori.chr" to "monika.chr" for the other ending where Sayori finds out she's in a game.
		// But you'll need a rust compiler for that.
		match fs::remove_file("characters/sayori.chr") {
			Ok(_) => println!("Successfully deleted character file."), // sorry
			Err(err) => eprintln!("Error deleting file: {}. Does it exist?", err),
		};
	});
	// TODO: max priority IO and this process HERE.

	// Launch & Post launch code, including max priority.
	match Command::new("DDLC.exe").spawn() {
		Ok(_) => second_main(),
		Err(err) => eprintln!("Error starting DDLC.exe: {}. Does it exist?", err),
	};
}
fn second_main() {
	// What this does is set priority to max for DDLC.exe.
	// I can't get this to work properly.
	// Can the rust compiler please pick this up and optimize it?
	println!("Setting max priority");
	match Command::new("C:\\Windows\\System32\\wbem\\WMIC.exe process where name='DDLC.exe' CALL setpriority 256").spawn() {
		Ok(_) => println!("Sayonara, Sayori."), // Goodbye (forever), Sayori.
		Err(err) => eprintln!("WMIC isn't in your system, but WMIC has been deprecated since windows 11, so I really don't know what to say: {}", err),
	};
}