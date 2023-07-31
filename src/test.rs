use std::{io::Write, path::Path, fs::File};

fn main() {
	if Path::new("game/firstrun").exists() {
		println!("No need to firstrun: it exists.");
		// exits automatically
	} else {
		// Jank CMD implementation
		// match Command::new("cmd.exe").args(["/c", "echo 1 >> game/firstrun"]).spawn() {
		// 	Ok(_) => {}
		// 	Err(err) => eprintln!("ERROR: unable to create firstrun: {}", err),
		// }

		match File::create("game/firstrun") {
			Ok(mut file) => {
				// For some reason, DDLC reads the '1' value from the firstrun file
				// So we can't just create it and call it a day... sad.
				match file.write_all(b"1") {
					Ok(_) => {}
					Err(err) => eprintln!("ERROR: Failed to create firstrun (2): {}", err),
				}
			}
			Err(err) => eprintln!("ERROR: Failed to create firstrun (1): {}", err),
		}
	}
}