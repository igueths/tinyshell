use std::io::{stdin,stdout};
use std::process::Command;
use std::io::Write;
fn main() {
	loop {
		println!(">");
		stdout().flush();
		let mut input = String::new();
		 stdin().read_line(&mut input).unwrap();
		let command = input.trim();
		let mut exec = Command::new(command).spawn().unwrap();
		exec.wait();
	}
}