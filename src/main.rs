// mod test;
use std::{fs::{rename/*, File*/}, process::Command, thread::{spawn, sleep}, /*path::Path, io::Write*/};

fn main() {
	// Create firstrun, bypassing the thing
	// Note: sometimes, the game will still ask you about the left off area and stuff..?
	spawn(|| {
		// TODO: forget about cmd
		// check if it exists
		// match File::create("game/firstrun") {
		// 	// Ok(mut file) => println!("Whoa"),
		// 	Ok(_) => {}
		// 	Err(err) => eprintln!("ERROR: Failed to create firstrun (1): {}", err),
		// }
		// For some reason, DDLC reads the '1' value from the firstrun file
		// So we can't just create it and call it a day... sad.
		// FIXME: sometimes, it doesn't write this? im confused.

		// match file.write_all(b"1") {
		// 	Ok(_) => {}
		// 	Err(err) => eprintln!("ERROR: Failed to create firstrun (2): {}", err),
		// }

		match Command::new("cmd.exe").args(["/c echo 1 >> game/firstrun"]).spawn() {
			Ok(_) => {println!("test it worked")}
			Err(err) => eprintln!("ERROR: unable to create firstrun: {}", err),
		}
	});

	spawn(|| {
		match Command::new("DDLC.exe").spawn() {
			Ok(_) => {
				// println!("Launched DDLC");
				// realtime forced because if you set it to high, it'll refuse.
				// not that realtime will even be applied.
				match Command::new("C:\\Windows\\System32\\wbem\\WMIC.exe").args(["process", "where", "name='DDLC.exe'", "CALL", "setpriority", "realtime"]).spawn() {
					Ok(_) => {}
					// Ok(_) => println!("hm"),
					Err(err) => eprintln!("ERROR: unable to set priority for DDLC.exe: {}", err)
				}
			}
			Err(err) => {
				eprintln!("FATAL: Error starting DDLC: {}", err);
			}
		}
	});

	// You can edit "sayori.chr" to "monika.chr" for the other ending where Sayori finds out she's in a game.
	match rename("characters/sayori.chr", "characters/s") {
		Ok(_) => {}
		Err(err) => {
			eprintln!("FATAL: Can't delete file: {}", err);
		}
	}
	println!("Sucessfully Launched DDLC.");
	loop{sleep(std::time::Duration::from_secs(1));} // Required, because if this (main) thread exits, all the others die too.
	// We have the sleep thing so we don't end up using 12% CPU.
}