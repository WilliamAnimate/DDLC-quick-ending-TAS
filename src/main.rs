use std::{fs::{rename, File, remove_file}, process::Command, thread::{spawn, sleep}, io::Write};

fn main() {
	spawn(|| {
		match Command::new("DDLC.exe").spawn() {
			Ok(_) => {
				println!("Launched DDLC");
				// realtime forced because if you set it to high, it'll refuse.
				// not that it'll allow us to set it to realtime, only high.
				match Command::new("C:\\Windows\\System32\\wbem\\WMIC.exe").args(["process", "where", "name='DDLC.exe'", "CALL", "setpriority", "realtime"]).spawn() {
					Ok(_) => {}
					Err(err) => eprintln!("ERROR: unable to set priority for DDLC.exe: {}", err)
				}
			}
			Err(err) => {
				eprintln!("FATAL: Error starting DDLC: {}", err);
			}
		}
	});

	// Create firstrun, bypassing the early content warning
	// This will work as long as you have a DDLC save file in your appdata directory.
	spawn(|| {
		match remove_file("game/firstrun") {
			Ok(_) => {}
			Err(err) => eprintln!("Firstrun doesn't exist, did you delete it?: {}", err),
		}
		// explaination on why deletion:
		// turns out, firstrun exists, it just puts the '1' value into it.
		// so we delete it and write 1 to it.
		match File::create("game/firstrun") {
			Ok(mut file) => {
				match file.write_all(b"1") {
					Ok(_) => {}
					Err(err) => eprintln!("ERROR: Failed to create firstrun (2): {}", err),
				}
			}
			Err(err) => eprintln!("ERROR: Failed to create firstrun (1): {}", err),
		}
	});

	// You can edit "sayori.chr" to "monika.chr" for the other ending where Sayori finds out she's in a game.
	match rename("characters/sayori.chr", "characters/s") {
		Ok(_) => {}
		Err(err) => {
			eprintln!("FATAL: Can't delete character file: {}", err);
		}
	}
	loop{sleep(std::time::Duration::from_secs(1));} // Required, because if this (main) thread exits, all the others die too.
	// We have the sleep thing so we don't end up using 12% CPU.
}