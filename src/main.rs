use std::{fs, process::Command, thread};
use std::fs::File;
use std::io::Write;
use std::fs::rename;

use std::env;
use std::path::Path;
fn main() {
	// thread::spawn(move || {
	// 	//
	// 	// okay, now i think, if i set max priority, won't DDLC.exe slow down?
	// 	match Command::new("C:\\Windows\\System32\\wbem\\WMIC.exe process where name='DDLC.exe' CALL setpriority 256").spawn() {
	// 		Ok(_) => println!("Set max priority for us."),
	// 		Err(err) => eprintln!("Can't set priority for our process: {}. does WMIC exist?", err),
	// 	};
	// });
	//
	// Spawn a new thread to delete the file, while the main thread instantly launches DDLC.exe.
	// N.B. this code may or may not start DDLC.exe before the file is deleted, but I pray to the developers of the filesystem drivers and the NT kernel that they coded this one specific function properly. Of course, the NT kernel is designed better than the rest of this OS, obviously.
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

	// Skip the content warning at the beginning of the game.
	thread::spawn(move || {
		if let Ok(appdata) = env::var("appdata") {
			let full_path = Path::new(&appdata).join("RenPy").join("DDLC-1454445547").join("persistent");
			let not_full_path = Path::new(&appdata).join("RenPy").join("DDLC-1454445547");

			// The actual check if persistent exists, if it does, chances are that we can skip this.
			if full_path.exists() {
				println!("the persistent file already exists, ignoring new persistent file.");
				// it should exit the thread automatically
			} else {
				// WORST CODE EVER WHO TF WROTE THIS LMAOOOOOOOOOOOOO
				// - williamanimate, creator of code
				println!("Writing new persistent file");
				// mm long binary dont touch
				let bin = "-----BEGIN CERTIFICATE-----
				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
				A+jrV1g=
				-----END CERTIFICATE-----";

				/*
				* This code fully assumes you're running the game off a super fast storage device
				* Say, a ramdisk. It first writes this file to where DDLC is located (which is where this exe file should be)
				* And de-base64ifies it. Then it moves to the real appdata folder.
				* This will not be super performant if you're running this game off a HDD.
				*/

				// Sets the variable
				let mut file = match File::create("ptemp") {
					Ok(f) => f,
					Err(_) => {
						println!("Failed to create file");
						return;
					}
				};

				// The actual write
				if let Err(_) = file.write_all(bin.as_bytes()) {
					println!("Failed to write to file");
					return;
				}

				// too lazy to implement error checking
				Command::new("certutil.exe").args(["-decode", "ptemp", "persistent"]);

				fs::rename("ptemp", "persistent")?;
				Ok(())

				// %appdata%\RenPy\DDLC-1454445547
				// 💯
				// .unwrap();
			}
		} else {
			println!("Unable to retrieve the value of appdata. Failed to create new persistent file. Are you using Windows?");
		}
	});

	// post launch
	match Command::new("DDLC.exe").spawn() {
		Ok(_) => {
			println!("Setting max priority");
			let _output = Command::new("C:\\Windows\\System32\\wbem\\WMIC.exe") // change _output to output for a warning. Probably not gonna fix it
			// Should this even be allowed? this is kinda modifing the game
			.args(["process", "where", "name='DDLC.exe'", "CALL", "setpriority", "256"])
			.output();
		}
		Err(err) => {
			eprintln!("Error starting DDLC.exe: {}. Does it exist?\nPress any key to close.", err);
			let _ = std::io::stdin(); // ignore warning v2
			std::process::exit(1);
		}
	};
}