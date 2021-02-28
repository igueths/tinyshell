use std::io;
use std::io::{stdin,stdout};
use std::process::Command;
use std::io::Write;
fn main() -> io::Result<()> {
	loop {
		println!(">");
		stdout().flush()?;
		let mut input = String::new();
		 stdin().read_line(&mut input)?;
		 // Let's make sure to handle any arguments that
		 //may come after the command.
let mut input_parts = input.trim().split_whitespace();
let command = input_parts.next().unwrap();
let args = input_parts;		
let mut exec = Command::new(command).args(args).spawn()?;
		exec.wait()?;
	}
}