use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;

use super::Command;

struct Options
{

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
	gid: u32
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
		gid: 0
	};
	// command
	match fs::read_to_string (format!("/proc/{}/cmdline", pid)) {
		Ok (command) => {
			process.command = command;
			if (process.command.len() == 0) {
				process.kernel_thread = true;
			}
		},
		Err (e) => process.kernel_thread = true
	};
	// read the stat
	if let Ok (stat) = fs::read_to_string (format!("/proc/{}/stat", pid)) {
		let stat_list:Vec<&str> = stat.split (" ").collect ();
		// println! ("{:?}", stat_list);
		// name
		if !process.kernel_thread {
			process.name = stat_list[1][1..stat_list[1].len()-1].to_string();
		}
		else
		{
			process.name = format! ("[{}]", &stat_list[1][1..stat_list[1].len()-1]);
			process.command = process.name.clone ();
		}
		// state
		if let Some (state) = stat_list[2].chars().nth (0) {
			process.state = state;
		}
		// ppid
		if let Ok (ppid) = stat_list[3].parse () {
			process.ppid = ppid;
		}
	}
	// uid and gid
	if let Ok (metadata) = fs::metadata (format! ("/proc/{}", pid)) {
		process.uid = metadata.uid ();
		process.gid = metadata.gid ();
	}
	println! ("{:?}", process);
	process
}

fn read_processes (_options: &Options) -> Result<Vec<Process>, io::Error>
{
	let mut processes:Vec<Process> = Vec::new ();
    for process in fs::read_dir ("/proc")? {
		let process = process?;
		let path = process.path ();
		if path.is_dir () {
			if let Some (pid_str) = path.file_name().expect("Unable to read procfs filesystem").to_str() {
				if let Ok (pid) = pid_str.parse::<u64> () {
					// println! ("{}", pid_str);
					processes.push (create_process (pid));
				}
			}
		}
	}
	// println! ("{:?}", processes);
	Ok (processes)
}

fn arguments (args: &[String]) -> Result<Options, io::Error> 
{
	let mut options: Options = Options {
		
	};

	// let mut is_option = true;

	for arg in args {
		// if is_option && (arg.starts_with ("-")) {
			
		// }
		// else {
			
		// }
	}
	Ok (options)
}

pub fn register () -> Command
{
	Command {
		command: "ps",
		description: "Report process status",
		run: &run
	}
}

pub fn run (args: &[String]) -> Result<(), io::Error>
{
	let options = arguments (args)?;
	let mut errno = 0;

	read_processes (&options);

	if errno != 0 {
		Err (io::Error::from_raw_os_error (errno))
	}
	else
	{
		Ok (())
	}
}