use std::io;

mod sh_ast;
mod sh_rules;

use sh_ast::*;

use super::command;

command! ("sh", "The shell", execute, 
	
);

pub fn execute (options: Options) -> Result<(), io::Error>
{
	let mut errno = 0;

	let mut redirects: Redirects = Redirects::new ();

    let command = sh_rules::CommandParser::new().parse(&mut redirects, "p1>3 | p2 | s ; E= s3>$12>4 s2");

	match command {
        Ok (s) => println! ("{:#?}", s),
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