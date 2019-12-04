use std::path::PathBuf;
use std::path::Path;
use std::fs;
use std::io;

use super::command;

command! ("rmdir", "Remove directory(s) if they are empty", execute, 
	(short = "p", long="parents", help = "Include empty parents")
	remove_parent: bool,

	(help = "directories to remove")
	path: Vec<PathBuf>
);

fn remove_directory (directory: &Path) -> i32
{
	let mut errno = 0;
	if let Err (err) = fs::remove_dir (&directory) {
		errno = errno | match err.raw_os_error () {
			Some (e) => e,
			None => 1
		};
		eprintln! ("rmdir: {}: {}", directory.to_str().unwrap_or (""), err);
	}
	errno
}

pub fn execute (options: Options) -> Result<(), io::Error>
{
	let mut errno = 0;

	if options.path.len () > 0 {
		for directory in options.path {
			if options.remove_parent {
				for parent in directory.ancestors () {
					errno = errno | remove_directory (parent);	
					if errno != 0 {
						break;
					}
				}
			} 
			else
			{
				errno = errno | remove_directory (&directory);
			}
		}
	}
	else
	{
		eprintln! ("rmdir: no directory");
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