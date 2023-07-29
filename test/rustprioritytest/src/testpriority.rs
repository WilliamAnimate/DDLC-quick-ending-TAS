extern crate winapi;

use std::ptr;
use winapi::um::processthreadsapi::{GetCurrentProcess, SetPriorityClass};
use winapi::um::winnt::{PROCESS_ALL_ACCESS, PROCESS_SET_INFORMATION};
use winapi::um::handleapi::CloseHandle;
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32};
use winapi::shared::minwindef::{DWORD, FALSE, MAX_PATH};

fn main() {
	// Get the process ID of "DDLC.exe"
	let process_id = get_process_id("DDLC.exe");
	if let Some(pid) = process_id {
		// Open the process with all access rights
		let process_handle = unsafe { winapi::um::processthreadsapi::OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid) };
		if process_handle != ptr::null_mut() {
			// Set the I/O priority to high
			unsafe {
				let result = winapi::um::processthreadsapi::SetPriorityClass(process_handle, winapi::um::winbase::HIGH_PRIORITY_CLASS);
				if result == 0 {
					println!("Failed to set I/O priority to high");
				}
			}
			// Set the CPU priority to high
			unsafe {
				let result = winapi::um::processthreadsapi::SetPriorityClass(process_handle, winapi::um::winbase::REALTIME_PRIORITY_CLASS);
				if result == 0 {
					println!("Failed to set CPU priority to high");
				}
			}
			// Close the process handle
			unsafe {
				winapi::um::handleapi::CloseHandle(process_handle);
			}
		} else {
			println!("Failed to open process");
		}
	} else {
		println!("Process not found");
	}
}

fn get_process_id(process_name: &str) -> Option<DWORD> {
	let snapshot = unsafe { CreateToolhelp32Snapshot(winapi::um::tlhelp32::TH32CS_SNAPPROCESS, 0) };
	if snapshot != winapi::um::handleapi::INVALID_HANDLE_VALUE {
		let mut process_entry: PROCESSENTRY32 = PROCESSENTRY32 {
			dwSize: std::mem::size_of::<PROCESSENTRY32>() as DWORD,
			cntUsage: 0,
			th32ProcessID: 0,
			th32DefaultHeapID: 0,
			th32ModuleID: 0,
			cntThreads: 0,
			th32ParentProcessID: 0,
			pcPriClassBase: 0,
			dwFlags: 0,
			szExeFile: [0; MAX_PATH],
		};
		if unsafe { Process32First(snapshot, &mut process_entry) } == TRUE {
			while unsafe { Process32Next(snapshot, &mut process_entry) } == TRUE {
				let exe_file = String::from_utf8_lossy(&process_entry.szExeFile);
				if exe_file.to_lowercase() == process_name.to_lowercase() {
					return Some(process_entry.th32ProcessID);
				}
			}
		}
		unsafe {
			CloseHandle(snapshot);
		}
	}
	None
}
