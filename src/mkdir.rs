use std::fs;
use std::io;
use std::path::PathBuf;

use super::command;

command! ("mkdir", "Create directory", execute, 
	(short = "p", help = "No error if exists; make parent directories as needed")
	create_parent: bool,

	(help = "directories to create")
	path: Vec<PathBuf>
);

pub fn execute (options: Options) -> Result<(), io::Error>
{
	let mut errno = 0;

	if options.path.len () > 0 {
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
	}
	else
	{
		eprintln! ("mkdir: no directory");
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