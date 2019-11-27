use std::io;

#[macro_export]
macro_rules! command {
    ( $command:literal, $description: literal, $run:ident, $(
            ( $($attributes:meta),* ) $option: ident: $type: ty),* ) => {
        use crate::Command;
        use structopt::StructOpt;

        #[derive(StructOpt/*, Debug*/)]
        #[structopt(name = $command, about=$description)]
        pub struct Options
        {
            $( #[structopt ($($attributes),*)] $option: $type),*
        }

        pub fn register () -> Command
        {
            Command {
                command: $command,
                description: $description,
                run: &run
            }
        }

        pub fn run (args: &[String]) -> Result<(), io::Error>
        {
            let options = Options::from_iter (args.iter());
            // println! ("{:?}", options);
            return $run (options)
        }
    }
}

#[macro_export]
macro_rules! register {
    ( $commands: ident, $command: ident ) => {
        let command_register = $command::register ();
        $commands.insert (command_register.command.to_string(), command_register);
    }
}

pub struct Command
{
    pub command: &'static str,
    pub description: &'static str,
    pub run: &'static dyn Fn(&[String]) -> Result<(), io::Error>
}
