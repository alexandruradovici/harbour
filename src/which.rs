use std::io;
use std::env;
use std::fs;

use std::os::unix::fs::PermissionsExt;

use super::command;


command! ("which", "Locate a command", execute, 
	(short = "a", long = "all", help = "Show all commands")
	show_all: bool,
	
    (short = "s", help = "Slient mode, only return the error code")
    no_display: bool,

    (name="command", parse(from_str), help = "Commands to locate")
    commands: Vec<String>
);

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

pub fn execute (options:Options) -> Result<(), io::Error>
{
	let mut errno = 0;

	let paths = get_paths ();
	
	if options.commands.len () > 0 {
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
	}
	else
	{
		errno = 1;
	}

	if errno != 0 {
		Err (io::Error::from_raw_os_error (errno))
	}
	else
	{
		Ok (())
	}
}