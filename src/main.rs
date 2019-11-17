use std::env;
use std::path::Path;
use std::io;
use std::process::exit;

mod pwd;
mod ls;

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
    match command {
        "pwd" => pwd::run (&args)?,
        "ls" => ls::run (&args)?,
        _ => {
            command_not_found (&command);
            return Err (io::Error::from_raw_os_error (127));
        }
    }
    Ok (())
}

fn main() {
    let args:Vec<String> = env::args().collect ();
    let command = get_command (&args[0]);
    if let Err(error) = run_command (&command, &args[1..]) {
        if let Some(error_number) = error.raw_os_error () {
            exit (error_number);
        }
    }
}
