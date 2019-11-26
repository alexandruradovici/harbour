use std::io;

use users;

use super::command;

command! ("id", "Print information about USER or the current user", execute, 
	(short = "n", help = "Print name instead of a number")
	show_names: bool,

	(short = "G", help = "Print supplementary group IDs")
	show_groups: bool,

	(short = "u", help = "Print user ID")
	show_user_id: bool,

	(short = "g", help = "Print group ID")
	show_group_id: bool,

	(short = "r", help = "Print real user ID instead of effective ID")
	show_real_id: bool,

	(help="The user name, leave empty for the current user")
	username: Option<String>
);

pub fn execute (mut options: Options) -> Result<(), io::Error>
{
	let username = if let Some(username) = options.username {
			username
		}
		else
		{
			let user = if options.show_real_id {
				users::get_current_username ()
			}
			else
			{
				users::get_effective_username ()
			};
			match user {
				Some (username_os_str) => match username_os_str.to_str() {
					Some (username) => username.to_string (),
					None => String::from ("")
				},
				None => String::from ("")
			}
		};
	let mut errno = 0;
	if let Some (user) = users::get_user_by_name (&username) {
		if options.show_user_id {
			if !options.show_names {
				println! ("{}", user.uid ())
			}
			else
			{
				println! ("{}", &username);
			}
		}
		else 
		if options.show_group_id {
			if !options.show_names {
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
		else
		{

			let groupname = match users::get_group_by_gid (user.primary_group_id()) {
				Some (group) => match group.name().to_str () {
					Some (groupname) => groupname.to_string (),
					None => String::from ("")
				},
				None => String::from ("")
			};

			print! ("uid={}({}) gid={}({}) ", user.uid(), &username, user.primary_group_id(), groupname);
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
	}
	else
	{
		eprintln! ("id: {}: no such user", &username);
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