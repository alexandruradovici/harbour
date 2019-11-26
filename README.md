# harbour
Standard UNIX shell commands written in Rust and inspired by BusyBox.

## Design 
Harbour is composed out of *commands*. Each command is a rust module residing the the main crate.

## Command Format

### Module

Create a new module with the command's name.

You can use the *command!* macro. This talkes the following arguments:
  * the name of the command (used as *harbour name* or just *name*)
  * the description of the command (displayed when *harbour* is used without any parameter)
  * the function that will be executed when the command is used (sually called *execute*)
  * the list of [structopt](https://docs.rs/structopt/0.3.5/structopt/) attributes

The *command!* macro will generate:
  * the required *Options* struct
  * the *register* function (used by *register!* in main.rs)
  * the *run* function (used be *register!* in main.rs)

````rust
use super::command;

command! ("name", "Description of the command", execute, 
	(short = "o", long = "option", help = "Option description")
	option: bool,

	(short = "a", long = "another-option", help = "Another option description")
	another_option: bool,

	(help="optional element")
	element: Option<String>,

    (help="mandatory elements")
	mandatory_element: String,

    (help="elements")
    elements: Vec<String>
);

pub fn execute (mut options: Options) -> Result<(), io::Error>
{
	let mut errno = 0;
	
    // ... the command's code is here

	if errno != 0 {
		Err (io::Error::from_raw_os_error (errno))
	}
	else
	{
		Ok (())
	}
}
````

### Register the command

In the main.rs module, add in the *run_command* your new command using the *register!* macro. It takes the following parameters:
  * your command's module (the one where you used the *command!* macro)
  * the *commands* variable Hashmap

````rust
mod new_command;

/// ...

fn run_command (command: &str, args:&[String]) -> Result<(), std::io::Error>
{
    let mut commands: HashMap <String, Command> = HashMap::new ();
    
    /// ... register other commands

    register! (new_command, commands);
    
    /// ... the rest of the run_command function
}
````