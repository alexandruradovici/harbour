# Harbour Shell
Standard Linux shell commands written in Rust and inspired by BusyBox.

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

## Available Commands

These commands are implemented so far

Name | Description
------- | -----------------
*id* | Print information about USER or the current user
*ls* | List directory(s) contents
*mkdir* | Create directory
*ps* | Report process status
*pwd* | Print the full filename of the current working directory
*which* | Locate a command

### id
Print information about USER or the current user

```
USAGE:
    id [FLAGS] [username]

FLAGS:
    -h, --help       Prints help information
    -g               Print group ID
    -G               Print supplementary group IDs
    -n               Print name instead of a number
    -r               Print real user ID instead of effective ID
    -u               Print user ID
    -V, --version    Prints version information

ARGS:
    <username>    The user name, leave empty for the current user
```

### ls
List directory(s) contents

```
USAGE:
    ls [FLAGS] [file]...

FLAGS:
    -b, --escape             Escape names
    -h, --help               Prints help information
    -r, --reverse            Sort in reverse order
    -F, --classify           Append indicator (one of */=@|) to entries
    -p                       Append indicator (one of */=@|) to entries
    -A, --almost-all         Don't list . and ..
    -a, --all                Don't hide entries starting with .
    -i, --inode              List inode numbers
    -l                       Long listing format
    -n, --numeric-uid-gid    List numeric UIDs and GIDs instead of names
    -f                       Sort by file name
    -s, --size               Sort by file size
    -V, --version            Prints version information

ARGS:
    <file>...    Files or directories to show
```

### mkdir
Create directory

```
USAGE:
    mkdir [FLAGS] [path]...

FLAGS:
    -p               No error if exists; make parent directories as needed
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <path>...    directories to create
```

### ps
Report process status

```
USAGE:
    ps [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o <column,...>        Columns for display
```

### pwd
Print the full filename of the current working directory

```
USAGE:
    pwd

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```

### which
Locate a command

```
USAGE:
    which [FLAGS] [command]...

FLAGS:
    -h, --help       Prints help information
    -s               Slient mode, only return the error code
    -a, --all        Show all commands
    -V, --version    Prints version information

ARGS:
    <command>...    Commands to locate
```