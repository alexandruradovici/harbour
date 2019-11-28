use super::command;

use std::io;
use std::{thread, time};

command! ("sleep", "Suspend execution for an interval of time", execute, 
	(help = "the interval to suspend execution (default in seconds), optional suffixes are (s)econds, (m)inutes, (h)ours, (d)ays")
	interval: String
);

fn parse_interval_str (interval_s:&str) -> Result<f32, io::Error>
{
	if let Ok (interval) = interval_s.parse::<f32> () {
		Ok (interval)
	}
	else
	{
		eprintln! ("interval must be a number");
		Err (io::Error::from_raw_os_error (1))
	}
}

pub fn execute (options: Options) -> Result<(), io::Error>
{
	let mut errno = 0;

	let seconds = match options.interval.chars().nth (options.interval.len()-1).unwrap_or ('e') {
		's' => parse_interval_str (&options.interval[..options.interval.len()-1])?,
		'm' => parse_interval_str (&options.interval[..options.interval.len()-1])? * 60.0,
		'h' => parse_interval_str (&options.interval[..options.interval.len()-1])? * 60.0 * 60.0,
		'd' => parse_interval_str (&options.interval[..options.interval.len()-1])? * 24.0 * 60.0 * 60.0,
		'e' => {
			eprintln! ("sleep: unknown interval suffix");
			Err (io::Error::from_raw_os_error(1))?
		}
		_ => parse_interval_str (&options.interval)?
	};
	
	if seconds >= 0.0 {
		// TODO update to from_secs_f32 when rust is updated to 1.38
		thread::sleep (time::Duration::from_millis ((seconds*1000.0) as u64));
	}
	else
	{
		eprintln! ("sleep: interval cannot be negative");
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