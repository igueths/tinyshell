use std::io;
use std::io::{stdin,stdout};
use std::io::Write;
use std::process::Command;
use std::env;
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
match command {
	"cd" => {
		let path = args.peekable().peek().map_or("/", |p| *p);
		match env::set_current_dir(&path) {
			Ok(_) => continue,
			Err(_) => match io::ErrorKind::NotFound {
				_ => println!("No such file or directory"),
			}
		}
	},
	command => {
		let mut exec = Command::new(command).args(args).spawn()?;
		exec.wait()?;
	}
}
	}
}