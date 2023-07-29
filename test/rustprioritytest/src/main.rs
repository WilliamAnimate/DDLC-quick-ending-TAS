use std::process::Command;

fn main() {
	let process_name = "DDLC.exe";

	// Find the process ID of the target process
	let output = Command::new("tasklist")
		.output()
		.expect("Failed to execute tasklist command");

	let tasklist_output = String::from_utf8_lossy(&output.stdout);
	let process_id = find_process_id(&tasklist_output, process_name)
		.expect("Process not found");

	// Set I/O priority to Maximum (4)
	let io_priority_cmd = format!("wmic process where ProcessId={} set PriorityClass=4", process_id);
	Command::new("cmd")
		.args(&["/C", &io_priority_cmd])
		.output()
		.expect("Failed to set I/O priority");

	// Set CPU priority to High (above normal)
	let cpu_priority_cmd = format!("wmic process where ProcessId={} CALL setpriority realtime", process_id);
	Command::new("cmd")
		.args(&["/C", &cpu_priority_cmd])
		.output()
		.expect("Failed to set CPU priority");
}

fn find_process_id(tasklist_output: &str, process_name: &str) -> Option<u32> {
	for line in tasklist_output.lines() {
		let columns: Vec<&str> = line.split_whitespace().collect();
		if columns.len() >= 2 && columns[0].ends_with(".exe") {
			let pid = columns[1].parse::<u32>();
			if let Ok(pid) = pid {
				let name = columns[0];
				if name == process_name {
					return Some(pid);
				}
			}
		}
	}
	None
}