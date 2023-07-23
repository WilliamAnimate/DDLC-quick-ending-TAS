use std::fs;
use std::process::Command;

fn main() {
	// you can edit "sayori.chr" to "monika.chr" for the other quick ending.
	// TODO: can we make this async thread? DDLC is run on python, so its bound to be slow asf.
	// doing the async thread may require cargo.
	let file_path = format!("{}/{}", "characters", "sayori.chr"); // bye bye

	// delete
	match fs::remove_file(&file_path) {
		Ok(_) => {
			println!("bye bye {}", file_path);
			// launch the game.
			match Command::new("DDLC.exe").spawn() {
				Ok(_) => {
					eprintln!("Launched DDLC.")
				}
				Err(err) => {
					eprintln!("Can't launch DDLC: {}", err);
				}
			}
		} Err(err) => {
			// what
			eprintln!("Can't delete the file at {}: {}", file_path, err);
		}
	}
}
