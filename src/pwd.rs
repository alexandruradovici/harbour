use std::io;
use std::env;

use super::Command;

pub fn register () -> (Command)
{
	Command {
		command: "pwd", 
		description: "Print the full filename of the current working directory",
		run: &run
	}
}

fn run (_args: &[String]) -> Result<(), io::Error>
{
	let current_dir = env::current_dir ()?;
	println! ("{}", current_dir.display());
	Ok (())
}