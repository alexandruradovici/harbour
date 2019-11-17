use std::io;
use std::env;

pub fn run (_args: &[String]) -> Result<(), io::Error>
{
	let current_dir = env::current_dir ()?;
	println! ("{}", current_dir.display());
	Ok (())
}