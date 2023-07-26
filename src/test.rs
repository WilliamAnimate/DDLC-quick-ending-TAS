use std::path;

fn main() {
	println!("{}", Path::new("/etc/hosts").exists());
}