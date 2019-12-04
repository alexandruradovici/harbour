use std::io;

mod docks_rules;

use super::command;

command! ("docks", "The shell", execute, 
	
);

pub fn execute (options: Options) -> Result<(), io::Error>
{
	let mut errno = 0;

    let command = docks_rules::CommandParser::new().parse("S= 2 ls1$(p p2 | E=1 s s2) 5435243|ls3");

	match command {
        Ok (s) => println! ("{}", s),
        Err (e) => eprintln! ("{}", e)
    };

	if errno != 0 {
		Err (io::Error::from_raw_os_error (errno))
	}
	else
	{
		Ok (())
	}
}