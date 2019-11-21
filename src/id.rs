use std::io;
use super::Command;

use users;

struct Options {
	show_all: bool,
	show_names: bool,
	show_groups: bool,
	show_user_id: bool,
	show_group_id: bool,
	show_numbers: bool,
	username: String
}

fn arguments (args: &[String]) -> Result<Options, io::Error> 
{
	let mut options: Options = Options {
		show_all: true,
		show_names: false,
		show_groups: false,
		show_user_id: false,
		show_group_id: false,
		show_numbers: false,
		username: match users::get_effective_username () {
			Some (username_os_str) => match username_os_str.to_str() {
				Some (username) => username.to_string (),
				None => String::from ("")
			},
			None => String::from ("")
		}
	};

	let mut is_option = true;
	let mut is_username = false;

	for arg in args {
		if is_option && (arg.starts_with ("-")) {
			if arg == "-u" {
				options.show_user_id = true;
				options.show_all = false;
				// TODO show error
			}
			else
			if arg == "-g" {
				options.show_group_id = true;
				options.show_all = false;
				// TODO show error
			}
			else
			if arg == "-r" {
				options.username = match users::get_current_username () {
					Some (username_os_str) => match username_os_str.to_str() {
						Some (username) => username.to_string (),
						None => String::from ("")
					},
					None => String::from ("")
				};
			}
			else
			if arg == "-G" {
				options.show_groups = true;
				options.show_all = false;
				// TODO show error
			}
			else
			if arg == "-n" {
				options.show_numbers = true;
				options.show_names = true;
			}
		}
		else {
			is_option = false;			
			if !is_username {
				options.username = arg.to_string();
				is_username = false;
			}
		}
	}
	Ok (options)
}

pub fn register () -> Command
{
	Command {
		command: "id",
		description: "Print information about USER or the current user",
		run: &run
	}
}

pub fn run (args: &[String]) -> Result<(), io::Error>
{
	let options = arguments (args)?;
	let mut errno = 0;

	if let Some (user) = users::get_user_by_name (&options.username) {
		if options.show_all {

			let groupname = match users::get_group_by_gid (user.primary_group_id()) {
				Some (group) => match group.name().to_str () {
					Some (groupname) => groupname.to_string (),
					None => String::from ("")
				},
				None => String::from ("")
			};

			print! ("uid={}({}) gid={}({}) ", user.uid(), &options.username, user.primary_group_id(), groupname);
			if let Some (groups) = user.groups () {
				let mut first_element = true;
				for group in groups.iter () {
					if group.gid () != user.primary_group_id () {
						let groupname = match group.name ().to_str () {
							Some (groupname) => groupname,
							None => &""
						};
						if !first_element {
							print! (",");
						}
						else
						{
							first_element = false;
						}
						print! ("{}({})", group.gid(), groupname);
					}
				}
			}
			println! ();
		}
		else
		if options.show_user_id {
			if options.show_numbers {
				println! ("{}", user.uid ())
			}
			else
			{
				println! ("{}", &options.username);
			}
		}
		else 
		if options.show_group_id {
			if options.show_numbers {
				println! ("{}", user.primary_group_id ())
			}
			else
			{
				let groupname = match users::get_group_by_gid (user.primary_group_id()) {
					Some (group) => match group.name().to_str () {
						Some (groupname) => groupname.to_string (),
						None => String::from ("")
					},
					None => String::from ("")
				};
				println! ("{}", groupname);
			}
		}
		else
		if options.show_groups {
			if let Some (groups) = user.groups () {
				for group in groups.iter () {
					if options.show_names {
						match group.name ().to_str () {
							Some (groupname) => print! ("{} ",groupname),
							None => ()
						};
					}
					else
					{
						print! ("{} ", group.gid ());
					}
				}
			}
			println! ();
		}
	}
	else
	{
		eprintln! ("id: {}: no such user", options.username);
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