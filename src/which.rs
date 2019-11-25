use std::io;
use std::env;
use std::fs;

use structopt::StructOpt;

use std::os::unix::fs::PermissionsExt;


/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "which", about="Locate a command")]
struct Options {
    #[structopt(short = "a", long = "--all")]
	show_all: bool,
	
    #[structopt(short = "-s")]
    no_display: bool,

    #[structopt(name="command", parse(from_str))]
    commands: Vec<String>
}

use super::Command;

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
	let options = Options::from_iter (args.iter());
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