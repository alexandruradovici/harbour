use std::env;
use std::path::Path;
use std::io;
use std::process::exit;
use std::collections::HashMap;
use tabular::{Table, Row};

mod command;
mod util;
mod pwd;
mod ls;
mod which;
mod id;
mod ps;
mod mkdir;
mod sleep;
mod rmdir;
mod docks;

use command::Command;

fn get_command (arg: &str) -> &str
{
    let path = Path::new (arg);
    if let Some(command_os_str) = path.file_stem () {
        if let Some(command) = command_os_str.to_str () {
            return command;
        }
    }
    ""
}

fn command_not_found (command: &str)
{
    println! ("{}: command not found", command);
}

fn run_command (command: &str, args:&[String]) -> Result<(), std::io::Error>
{
    let mut commands: HashMap <String, Command> = HashMap::new ();

    
    // add commands
    register! (commands, pwd);
    register! (commands, ls);
    register! (commands, which);
    register! (commands, id);
    register! (commands, docks);
    register! (commands, ps);
    register! (commands, mkdir);
    register! (commands, sleep);
    register! (commands, rmdir);

    if command == "" {
        let mut table = Table::new ("   {:>}  {:<}");
        let mut sorted_commands:Vec<&Command> = commands.values().collect();
        sorted_commands.sort_by (|c1, c2| c1.command.partial_cmp (&c2.command).unwrap());
        for command in sorted_commands.iter() {
            table.add_row (Row::new ().with_cell (command.command).with_cell (command.description));
        }
        println! ("{}", table);
    }
    else
    {
        match commands.get (command) {
            Some (cmd) => (cmd.run) (&args)?,
            _ => {
                command_not_found (&command);
                return Err (io::Error::from_raw_os_error (127));
            }
        }
    }
    Ok (())
}

fn main() {
    let args:Vec<String> = env::args().collect ();
    let mut command = get_command (&args[0]);
    let mut first_argument = 0;
    if command == "harbour" {
        if args.len () > 1 {
            command = get_command (&args[1]);
            first_argument = 1;
        }
        else
        {
            command = "";
        }
    }
    if let Err(error) = run_command (&command, &args[first_argument..]) {
        if let Some(error_number) = error.raw_os_error () {
            exit (error_number);
        }
    }
}
