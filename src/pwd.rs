use std::io;
use std::env;

use super::command;

command! ("pwd", "Print the full filename of the current working directory", execute, );

fn execute (_options:Options) -> Result<(), io::Error>
{
	let current_dir = env::current_dir ()?;
	println! ("{}", current_dir.display());
	Ok (())
}