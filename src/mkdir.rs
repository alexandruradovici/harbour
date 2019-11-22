use std::fs;
use std::io;
use std::path::PathBuf;

use super::Command;

struct Options {
	create_parent: bool,
	path: Vec<PathBuf>
}

fn arguments (args: &[String]) -> Result<Options, io::Error> 
{
	let mut options: Options = Options {
		create_parent: false,
		path: Vec::new ()
	};

	let mut is_option = true;

	for arg in args {
		if is_option && (arg.starts_with ("-")) {
			if arg == "-p" {
				options.create_parent = true;
			}
		}
		else {
			is_option = false;			
			options.path.push (PathBuf::from (arg));
		}
	}
	Ok (options)
}

pub fn register () -> Command
{
	Command {
		command: "mkdir",
		description: "Create directory",
		run: &run
	}
}

pub fn run (args: &[String]) -> Result<(), io::Error>
{
	let options = arguments (args)?;
	let mut errno = 0;

	for directory in options.path {
		if let Err(err) = if options.create_parent {
				fs::create_dir_all (&directory)
			}
			else
			{
				fs::create_dir (&directory)
			}
		{
			errno = errno | match err.raw_os_error () {
				Some (e) => e,
				None => 1
			};
			eprintln! ("mkdir: {}: {}", directory.to_str().unwrap_or (""), err);
		}
	}

	if errno != 0 {
		Err (io::Error::from_raw_os_error (errno))
	}
	else
	{
		Ok (())
	}
}