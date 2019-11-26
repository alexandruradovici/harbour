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
use tabular::{Table, Row};

use super::command;

use users;

const LONG_FORMAT:&str = "{:<}{:<}  {:>} {:<}  {:<}  {:>} {:>} ";
const INODE_FORMAT:&str = "{:>} ";
const FORMAT:&str = "{:<}";

struct File
{
	filename: String,
	metadata: fs::Metadata,
	full_path: PathBuf
}

fn sort_by_filename (files: & mut Vec<File>)
{
	files.sort_by (|a, b| a.filename.partial_cmp (&b.filename).unwrap());
}

fn sort_by_size (files: & mut Vec<File>)
{
	files.sort_by (|a, b| a.metadata.len().partial_cmp (&b.metadata.len()).unwrap());
}

// fn arguments (args: &[String]) -> Result<Options, io::Error>
// {
// 	let mut options: Options = Options {
// 		show_hidden_files: false,
// 		show_folder_and_parent: false,
// 		escape: false,
// 		show_long_listing: false,
// 		path: Vec::new (),
// 		show_inode: false,
// 		sort_by_filename: true,
// 		sort_by_size: false,
// 		reverse_order: false,
// 		show_file_type: false,
// 		show_numeric_uid_and_gid: false
// 	};

// 	let mut is_option = true;

// 	for arg in args {
// 		if is_option && (arg.starts_with ("-") || arg.starts_with ("--")) {
// 			if arg == "-a" || arg == "--all" {
// 				options.show_hidden_files = true;
// 				options.show_folder_and_parent = true;
// 			}
// 			else
// 			if arg == "-A" || arg == "--almost-all" {
// 				options.show_hidden_files = true;
// 			}
// 			else
// 			if arg == "-b" || arg == "--escape" {
// 				options.escape = true;
// 			}
// 			else
// 			if arg == "-l" {
// 				options.show_long_listing = true;
// 			}
// 			else
// 			if arg == "-i" || arg == "--inode" {
// 				options.show_inode = true;
// 			}
// 			else
// 			if arg == "-f"  {
// 				options.sort_by_filename = false;
// 			}
// 			else
// 			if arg == "-s" || arg == "--size"  {
// 				options.sort_by_filename = false;
// 				options.sort_by_size = false;
// 			}
// 			else
// 			if arg == "-r" || arg == "--reverse"  {
// 				options.reverse_order = true;
// 			}
// 			else
// 			if arg == "-F" || arg == "-p" || arg == "--classify"  {
// 				options.show_file_type = true;
// 			}
// 			else
// 			if arg == "-n" || arg == "--numeric-uid-gid"  {
// 				options.show_numeric_uid_and_gid = true;
// 			}
// 		}
// 		else {
// 			is_option = false;			
// 			options.path.push (PathBuf::from (arg));
// 		}
// 	}
// 	Ok (options)
// }

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
			metadata: fs::metadata (env::current_dir()?)?,
			full_path: env::current_dir ()?.to_path_buf ()
		});
		if let Some (parent) = env::current_dir()?.parent() {
			files.push (File {
				filename: String::from (".."),
				metadata: fs::metadata (parent)?,
				full_path: parent.to_path_buf ()
			});
		}
		else
		{
			files.push (File {
				filename: String::from ("."),
				metadata: fs::metadata (env::current_dir()?)?,
				full_path: env::current_dir ()?.to_path_buf ()
			});
		}
	}

	if path.is_dir () {
		for file in fs::read_dir (path)? {
			let file = file?;
			if let Some (filename) = file.path().file_name() {
				if let Some (filename_str) = filename.to_str() {
					let mut full_path = path.to_path_buf();
					full_path.push (filename_str);
					files.push (File {
						filename: filename_str.to_string (),
						metadata: file.metadata()?,
						full_path: full_path
					});
				}
			}
		}
	}
	else if path.is_file () {
		if let Some (filename) = path.file_name() {
			if let Some (filename_str) = filename.to_str() {
				let mut full_path = path.to_path_buf();
				files.push (File {
					filename: filename_str.to_string (),
					metadata: fs::symlink_metadata(&full_path)?,
					full_path: full_path
				});
			}
		}
	}
	else
	{

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

	let mut format = FORMAT.to_string();
	if options.show_long_listing {
		format = format! ("{}{}", LONG_FORMAT, FORMAT);
	}
	if options.show_inode {
		format = format! ("{}{}", INODE_FORMAT, format);
	}

	let mut table = Table::new (&format);

	for mut f in files {
		let mut row = Row::new ();
		
		if options.show_inode {
			row.add_cell (f.metadata.ino());
		}

		if options.show_file_type {
			let mut ftype = ' ';
			let file_type = f.metadata.file_type ();
			if file_type.is_dir () {
				ftype = '/';
			}
			else if file_type.is_fifo () {
				ftype = '|';
			}
			else if file_type.is_socket () {
				ftype = '=';
			}
			else if f.metadata.permissions().mode() & 0o111 != 0 {
				ftype = '*';
			}
			else if file_type.is_symlink () && !options.show_long_listing {
				ftype = '@';
			}

			if ftype != ' ' {
				f.filename = format! ("{}{}", &f.filename, ftype);
			}
		}

		if options.show_long_listing {
			if let Err(error) = print_long_listing (&mut row, &mut f, &options) {
				eprintln! ("ls: {}", error);
			}
			else
			{
				row.add_cell (filename (&f.filename, &options));
			}
		}
		else {
			row.add_cell (filename (&f.filename, &options));
		}
		table.add_row (row);
	}
	println! ("{}", table);
	Ok (())
}

fn print_long_listing (row:&mut Row, f: &mut File, options: &Options) -> Result<(), io::Error>
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
		let link = fs::read_link (&f.full_path)?;
		if let Some (link_str) = link.to_str () {
			f.filename = format! ("{} -> {}", &f.filename, link_str.to_string());
		}
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

	let username = if !options.show_numeric_uid_and_gid {
		match users::get_user_by_uid (uid) {
			Some (user) => match user.name ().to_str() {
				Some (username) => username.to_string(),
				None => uid.to_string()
			},
			None => uid.to_string ()
		}
	}
	else {
		uid.to_string ()
	};

	let gid = f.metadata.gid();

	let group = if !options.show_numeric_uid_and_gid {
		match users::get_group_by_gid (gid) {
			Some (group) => match group.name ().to_str() {
				Some (groupname) => groupname.to_string(),
				None => gid.to_string()
			},
			None => gid.to_string ()
		}
	}
	else {
		gid.to_string ()
	};

	let dt:DateTime<Local> = DateTime::from (f.metadata.modified()?);

	let mtime = dt.format ("%d %b %H:%M");
	
	// println! ("{}{}  {}  {}  {}  {}  {}  {}", ftype, mode_string, f.metadata.nlink(), username, group, f.metadata.len(), mtime, filename(&f.filename, &options));
	row.add_cell (ftype).add_cell (mode_string).add_cell (f.metadata.nlink()).add_cell (username).add_cell (group).add_cell (f.metadata.len()).add_cell (mtime);
	Ok (())
}

command! ("ls", "List directory(s) contents", execute, 
	(short = "a", long = "all", help = "Don't hide entries starting with .")
	show_hidden_files: bool,

	(short = "A", long = "almost-all", help = "Don't list . and ..")
	show_folder_and_parent: bool,

	(short = "b", long = "escape", help = "Escape names")
	escape: bool,

	(short = "l", help = "Long listing format")
	show_long_listing: bool,

	(short = "i", long = "inode", help = "List inode numbers")
	show_inode: bool,

	(short = "f", help = "Sort by file name")
	sort_by_filename: bool,

	(short = "s", long = "size", help = "Sort by file size")
	sort_by_size: bool,

	(short = "r", long = "reverse", help = "Sort in reverse order")
	reverse_order: bool,

	(short = "F", long = "classify", help = "Append indicator (one of */=@|) to entries")
	show_file_type: bool,

	(short = "p", help = "Append indicator (one of */=@|) to entries")
	show_file_type_dir: bool,

	(short = "n", long = "numeric-uid-gid", help = "List numeric UIDs and GIDs instead of names")
	show_numeric_uid_and_gid: bool,

	(name = "file", parse(from_str), help = "Files or directories to show")
	path: Vec<PathBuf>
);

fn execute (mut options:Options) -> Result <(), io::Error>
{
	let mut errno = 0;

	if options.path.len() == 0 {
		options.path.push (env::current_dir ()?);
	}
	
	for path in options.path.iter() {
		if options.path.len () > 1 && path.is_dir () {
			println! ("{}:", path.display());
		}
		if let Err (error) = list_folder (&path, &options) {
			eprintln! ("ls: {}: {}", path.display (), error);
			if errno == 0 {
				errno = match error.raw_os_error () {
					Some (e) => e,
					None => 1
				}
			}
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