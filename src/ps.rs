use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;

use tabular::{Table, Row};

use users;

use super::util::system;

use super::command;

#[derive(Debug)]
struct TTY {
	nr: u32,
	major: u8,
	minor: u32,
	name: String
}

#[derive(Debug)]
struct Process
{
    pid:u64,
    ppid:u64,
	name: String,
	state: char,
	kernel_thread: bool,
    command: String,
	uid: u32,
	gid: u32,
	tty: TTY,
	time: u32
}

fn read_process_name<'a> (statline:&'a str) -> Option<&'a str>
{
	// TODO read full process name from /proc/(pid)/status
	if let (Some(index1), Some(index2)) = (statline.find ('('), statline.find (')')) {
		Some (&statline[index1+1..index2])
	}
	else
	{
		None
	}
}

fn read_self_tty_nr () -> u32
{
	if let Ok (stat) = fs::read_to_string ("/proc/self/stat") {
		if let Some(pindex) = stat.find (')') {
			let stat_list:Vec<&str> = (&stat[pindex+2..]).split (" ").collect ();
			// tty
			if let Ok (tty_nr) = stat_list[4].parse::<u32> () {
				return tty_nr;
			} 
		} 
	}
	0
}

fn read_process_tty (pid:u64) -> Option<String> {
	// TODO read it from tty_nr
	if let Ok(link) = fs::read_link (format! ("/proc/{}/fd/0", pid)) {
		if let Some (link_str) = link.to_str () {
			if &link_str[0..5] == "/dev/" {
				return Some ((&link_str[5..]).to_string ());
			}
			else
			{
				return Some(link_str.to_string()); 
			}
		}
	}
	None
}

fn create_process (pid:u64) -> Process {
	let mut process = Process {
		pid,
		ppid: 0,
		name: String::from (""),
		state: '?',
		kernel_thread: false,
		command: String::from (""),
		uid: 0,
		gid: 0,
		tty: TTY {
			nr: 0,
			major: 0,
			minor: 0,
			name: String::from ("?")
		},
		time: 0
	};
	// command
	match fs::read_to_string (format!("/proc/{}/cmdline", pid)) {
		Ok (command) => {
			process.command = command;
			if process.command.len() == 0 {
				process.kernel_thread = true;
			}
		},
		Err (_e) => process.kernel_thread = true
	};
	// read the stat
	if let Ok (stat) = fs::read_to_string (format!("/proc/{}/stat", pid)) {
		if let Some(pindex) = stat.find (')') {
			let stat_list:Vec<&str> = (&stat[pindex+2..]).split (" ").collect ();
			// name
			if !process.kernel_thread {
				process.name = match read_process_name (&stat) {
					Some(name) => name.to_string(),
					None => String::from ("")
				};
			}
			else
			{
				process.name = format! ("[{}]", match read_process_name (&stat) {
					Some(name) => name.to_string(),
					None => String::from ("")
				});
				process.command = process.name.clone ();
			}
			// state
			if let Some (state) = stat_list[0].chars().nth (0) {
				process.state = state;
			}
			// ppid
			if let Ok (ppid) = stat_list[1].parse () {
				process.ppid = ppid;
			}
			// tty
			if let Ok (tty_nr) = stat_list[4].parse::<u32> () {
				process.tty.nr = tty_nr;
				process.tty.major = ((tty_nr >> 8) & 0xFF) as u8;
				process.tty.minor = (tty_nr >> 20) + (tty_nr & 0xFF);
				// println! ("pid {}, tty_nr {}, {} {}", pid, tty_nr, major, minor);
			}
			// time
			if let (Ok (utime), Ok (stime)) = (stat_list[10].parse::<u32> (), stat_list[11].parse::<u32> ()) {
				process.time = (utime + stime) / system::sysconf (system::_SC_CLK_TCK).unwrap_or (1) as u32;
			}
		} 
	}
	// uid and gid
	if let Ok (metadata) = fs::metadata (format! ("/proc/{}", pid)) {
		process.uid = metadata.uid ();
		process.gid = metadata.gid ();
	}
	// tty
	if let Some(tty) = read_process_tty (pid) {
		process.tty.name = tty;
	};
	process
}

fn read_processes (options: &Options) -> Result<Vec<Process>, io::Error>
{
	let mut processes:Vec<Process> = Vec::new ();
	let self_uid = if let Some(username) = &options.username {
			if let Some (user) = users::get_user_by_name (username)
			{
				user.uid()
			}
			else
			{
				eprintln! ("ps: username {}  does not exist", username);
				Err(io::Error::from_raw_os_error (1))?
			}
		}
		else
		{
			users::get_effective_uid ()
		};
	let self_tty_nr = read_self_tty_nr ();
    for process in fs::read_dir ("/proc")? {
		let process = process?;
		let path = process.path ();
		if path.is_dir () {
			if let Some (pid_str) = path.file_name().expect("Unable to read procfs filesystem").to_str() {
				if let Ok (pid) = pid_str.parse::<u64> () {
					// println! ("{}", pid_str);
					let process = create_process (pid);
					if options.show_all || options.username != None || (process.uid == self_uid && process.tty.nr == self_tty_nr) {
						processes.push (process);
					}
				}
			}
		}
	}
	// println! ("{:?}", processes);
	Ok (processes)
}

command! ("ps", "Report process status", execute,
	(name= "column,...", short = "o", help = "Columns for display")
	columns:Option<String>,

	(short="a", help = "Show all processes")
	show_all: bool,

	(short = "u", help = "Show processes only for a user")
	username:Option<String>
);

pub fn execute (options:Options) -> Result<(), io::Error>
{
	let mut errno = 0;

	let columns:Vec<&str> = if let Some (columns) = &options.columns {
		columns.split(",").collect ()
	}
	else
	{
		"pid,tty,time,cmd".split (",").collect ()
	};

	let processes = read_processes (&options)?;

	let mut format = String::from ("");

	let mut row = Row::new ();
	
	for column in columns.iter() {
		format = format! ("{} {}", format, match column.as_ref () {
				"pid" => {
					row.add_cell ("PID");
					"{:>}"
				},
				"cmd" => {
					row.add_cell ("COMMAND");
					"{:<}"
				},
				"uid" => {
					row.add_cell ("UID");
					"{:>}"
				},
				// "tty" => {
				// 	row.add_cell ("TTY");
				// 	"{:<}"
				// },
				/*
				the default is UPPERCASE and {:<} format
				*/
				_ => {
					row.add_cell (column.to_uppercase ());
					"{:<}"
				}
			}
		);
	}

	let mut table = Table::new (&format);
	table.add_row (row);

	if processes.len () > 0 {
		for process in processes {
			let mut row = Row::new ();

			for column in columns.iter () {
				match column.as_ref () {
					"pid" => row.add_cell (process.pid),
					"cmd" => row.add_cell (&process.name),
					"uid" => row.add_cell (process.uid),
					"tty" => row.add_cell (&process.tty.name),
					"time" => row.add_cell (format! ("{:0>2}:{:0>2}:{:0>2}", process.time / 3600, (process.time % 3600) / 60, process.time % 60)),
					_ => row.add_cell ("?")
				};
			}
			
			table.add_row (row);
		}
	}
	else
	{
		errno = 1;
	}

	println! ("{}", table);

	if errno != 0 {
		Err (io::Error::from_raw_os_error (errno))
	}
	else
	{
		Ok (())
	}
}