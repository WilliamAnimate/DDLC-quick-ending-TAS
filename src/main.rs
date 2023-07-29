use std::{fs, process::Command, thread};
// use std::{fs, thread};

fn main() {
	// Spawn a new thread to delete the file, while the main thread instantly launches DDLC.exe.
	thread::spawn(move || {
		// You can edit "sayori.chr" to "monika.chr" for the other ending where Sayori finds out she's in a game.
		match fs::remove_file("characters/sayori.chr") {
			Ok(_) => println!("Successfully deleted character file."), // sorry
			Err(err) => {
				eprintln!("FATAL: Can't delete character file: {}. Does it exist?\nPress any key to close.", err);
				let _ = std::io::stdin(); // ignore warning
				std::process::exit(1);
			}
		};
	});

	// launch and post actions
	match Command::new("DDLC.exe").spawn() {
		Ok(_) => {
			match Command::new("C:\\Windows\\System32\\wbem\\WMIC.exe").args(["process", "where", "name='DDLC.exe'", "CALL", "setpriority", "realtime"]).spawn() {
				Ok(_) => println!("Set max process priority for DDLC"),
				Err(err) => eprintln!("Error: unable to set priority for DDLC.exe: {}. Does wmic exist?", err)
			}
		}
		Err(err) => {
			eprintln!("FATAL: Error starting DDLC.exe: {}. Does it exist?\nPress any key to close.", err);
			let _ = std::io::stdin(); // ignore warning v2
			std::process::exit(1);
		}
	};
}