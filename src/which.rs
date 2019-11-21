use std::io;
use std::env;
use std::fs;

use std::os::unix::fs::PermissionsExt;

use super::Command;

struct Options <'a> {
	show_all: bool,
	no_display: bool,
	commands: Vec<&'a str>,
}

fn arguments (args: &[String]) -> Result<Options, io::Error> 
{
	let mut options: Options = Options {
		show_all: false,
		no_display: false,
		commands: Vec::new ()
	};

	let mut is_option = true;

	for arg in args {
		if is_option && (arg.starts_with ("-")) {
			if arg == "-a" {
				options.show_all = true;
			}
			else
			if arg == "-s" {
				options.no_display = true;
			}
		}
		else {
			is_option = false;			
			options.commands.push (&arg);
		}
	}
	Ok (options)
}

fn get_paths () -> Vec<String>
{
	let mut paths:Vec<String> = Vec::new ();
	match env::var ("PATH") {
		Ok (path) => {
			for p in path.split (":") {
				paths.push (p.to_string())
			}
		}
		Err (_e) => {}
	};
	paths
}

pub fn register () -> Command
{
	Command {
		command: "which",
		description: "Locate a command",
		run: &run
	}
}

pub fn run (args: &[String]) -> Result<(), io::Error>
{
	let options = arguments (args)?;
	let mut errno = 0;

	let paths = get_paths ();
	
	for command in options.commands.iter() {
		let mut err = 1;
		for path in paths.iter () {
			let full_filename = format! ("{}/{}", path, command);
			if let Ok (metadata) = fs::metadata (&full_filename) {
				if metadata.is_file () && metadata.permissions().mode() & 0o111 != 0
				{
					err = 0;
					if !options.no_display
					{
						println! ("{}", full_filename);
					}
					if !options.show_all 
					{
						break;
					}
				}
			}
		}
		errno = errno | err;
	}

	if errno != 0 {
		Err (io::Error::from_raw_os_error (errno))
	}
	else
	{
		Ok (())
	}
}