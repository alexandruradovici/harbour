use std::io;

mod sh_ast;
mod sh_rules;

use sh_ast::*;

use super::command;

command! ("sh", "The shell", execute, 
	(short="c", help="run command")
	command: Vec<String>
);

pub fn execute (options: Options) -> Result<(), io::Error>
{
	let mut errno = 0;

	if options.command.len() > 0 {
		let command_str = options.command.join (" ");
		parse_command (&options, &command_str);
	}

	if errno != 0 {
		Err (io::Error::from_raw_os_error (errno))
	}
	else
	{
		Ok (())
	}
}

fn parse_command (options: &Options, command_str:&str) -> Result<(), io::Error>
{
	let command = sh_rules::CommandParser::new().parse(command_str);
	match command {
		Ok (s) => println! ("{:#?}", s),
		Err (e) => eprintln! ("{}", e)
	};
	Ok (())
}