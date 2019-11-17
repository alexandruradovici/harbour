use std::io;
use std::fs;
use std::env;
use snailquote::escape;
use std::path::PathBuf;
use std::path::Path;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::PermissionsExt;
use chrono::prelude::*;

use users;

use super::Command;

struct Options
{
	show_hidden_files: bool,
	show_folder_and_parent: bool,
	escape: bool,
	show_long_listing: bool,
	path: Vec<PathBuf>,
	show_inode: bool,
	sort_by_filename: bool,
	sort_by_size: bool,
	reverse_order: bool
}

struct File
{
	filename: String,
	metadata: fs::Metadata
}

fn sort_by_filename (files: & mut Vec<File>)
{
	files.sort_by (|a, b| a.filename.partial_cmp (&b.filename).unwrap());
}

fn sort_by_size (files: & mut Vec<File>)
{
	files.sort_by (|a, b| a.metadata.len().partial_cmp (&b.metadata.len()).unwrap());
}

fn arguments (args: &[String]) -> Result<Options, io::Error>
{
	let mut options: Options = Options {
		show_hidden_files: false,
		show_folder_and_parent: false,
		escape: false,
		show_long_listing: false,
		path: Vec::new (),
		show_inode: false,
		sort_by_filename: true,
		sort_by_size: false,
		reverse_order: false
	};

	let mut is_option = true;

	for arg in args {
		if is_option && (arg.starts_with ("-") || arg.starts_with ("--")) {
			if arg == "-a" || arg == "--all" {
				options.show_hidden_files = true;
				options.show_folder_and_parent = true;
			}
			else
			if arg == "-A" || arg == "--almost-all" {
				options.show_hidden_files = true;
			}
			else
			if arg == "-b" || arg == "--escape" {
				options.escape = true;
			}
			else
			if arg == "-l" {
				options.show_long_listing = true;
			}
			else
			if arg == "-i" || arg == "--inode" {
				options.show_inode = true;
			}
			else
			if arg == "-f"  {
				options.sort_by_filename = false;
			}
			else
			if arg == "-s" || arg == "--size"  {
				options.sort_by_filename = false;
				options.sort_by_size = false;
			}
			else
			if arg == "-r" || arg == "--reverse"  {
				options.reverse_order = true;
			}
		}
		else {
			is_option = false;			
			options.path.push (PathBuf::from (arg));
		}
	}
	if options.path.len() == 0 {
		options.path.push (env::current_dir ()?);
	}
	Ok (options)
}

fn filename (filename: &str, options: &Options) -> String
{
	if options.escape {
		return escape (filename).to_string();
	}
	filename.to_string ()
}

fn list_folder (path: &Path, options: &Options) -> Result<(), io::Error>
{
	let mut files:Vec<File> = Vec::new ();

	// add special dirs
	if options.show_folder_and_parent
	{
		files.push (File {
			filename: String::from ("."),
			metadata: fs::metadata (env::current_dir()?)?
		});
		if let Some (parent) = env::current_dir()?.parent() {
			files.push (File {
				filename: String::from (".."),
				metadata: fs::metadata (parent)?
			});
		}
		else
		{
			files.push (File {
				filename: String::from ("."),
				metadata: fs::metadata (env::current_dir()?)?
			});
		}
	}

	for file in fs::read_dir (path)? {
		let file = file?;
		if let Some (filename) = file.path().file_name() {
			if let Some (filename_str) = filename.to_str() {
				files.push (File {
					filename: filename_str.to_string (),
					metadata: file.metadata()?
				});
			}
		}
	}

	// files.sort ();
	if options.sort_by_filename {
		sort_by_filename (&mut files);
	}
	else
	if options.sort_by_size {
		sort_by_size (&mut files);
	}
	if options.reverse_order {
		files.reverse ();
	}
	for f in files {
		if options.show_long_listing {
			if let Err(error) = print_long_listing (&f, &options) {
				eprintln! ("ls: {}", error);
			}
		}
		else {
			println! ("{} ", filename (&f.filename, &options));
		}
	}
	Ok (())
}

fn print_long_listing (f: &File, options: &Options) -> Result<(), io::Error>
{
	let mut ftype = '-';
	let file_type = f.metadata.file_type ();
	if file_type.is_dir () {
		ftype = 'd';
	}
	else if file_type.is_char_device () {
		ftype = 'c';
	}
	else if file_type.is_block_device () {
		ftype = 'b';
	}
	else if file_type.is_fifo () {
		ftype = 'f';
	}
	else if file_type.is_socket () {
		ftype = 's';
	}
	else if file_type.is_symlink () {
		ftype = 'l';
	}

	let permissions = f.metadata.permissions ();

	let mode = permissions.mode ();

	let mut str_mode: [char; 9] = ['-'; 9];

	if mode & 0o400 != 0 {
		str_mode [0] = 'r';
	}

	if mode & 0o200 != 0 {
		str_mode [1] = 'w';
	}

	if mode & 0o100 != 0 {
		str_mode [2] = 'x';
	}

	if mode & 0o040 != 0 {
		str_mode [3] = 'r';
	}

	if mode & 0o040 != 0 {
		str_mode [4] = 'w';
	}

	if mode & 0o010 != 0 {
		str_mode [5] = 'x';
	}

	if mode & 0o004 != 0 {
		str_mode [6] = 'r';
	}

	if mode & 0o002 != 0 {
		str_mode [7] = 'w';
	}

	if mode & 0o001 != 0 {
		str_mode [8] = 'x';
	}

	let mode_string:String = str_mode.into_iter().collect();

	let uid = f.metadata.uid();
	let username = match users::get_user_by_uid (uid) {
		Some (user) => match user.name ().to_str() {
			Some (username) => username.to_string(),
			None => uid.to_string()
		},
		None => uid.to_string ()
	};

	let gid = f.metadata.gid();
	let group = match users::get_group_by_gid (gid) {
		Some (group) => match group.name ().to_str() {
			Some (groupname) => groupname.to_string(),
			None => gid.to_string()
		},
		None => gid.to_string ()
	};

	let dt:DateTime<Local> = DateTime::from (f.metadata.modified()?);

	let mtime = dt.format ("%d %b %H:%M");

	println! ("{}{}  {}  {}  {}  {}  {}  {}", ftype, mode_string, f.metadata.nlink(), username, group, f.metadata.len(), mtime, filename(&f.filename, &options));
	Ok (())
}

pub fn register () -> Command
{
	Command {
		command: "ls",
		description: "List directory contents",
		run: &run
	}
}

pub fn run (args: &[String]) -> Result<(), io::Error>
{
	let options = arguments (args)?;
	
	for path in options.path.iter() {
		if options.path.len () > 1 {
			println! ("{}:", path.display());
		}
		if let Err (error) = list_folder (&path, &options) {
			eprintln! ("ls: {}: {}", path.display (), error);
		}
	}

	Ok (())
}