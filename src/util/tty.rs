use std::io;
use std::fs;

pub struct TTYDriver
{
	
}

pub fn read_drivers () -> Result<Vec<TTYDriver>, io::Error>
{
	let drivers_str = fs::read_to_string ("/proc/tty/drivers")?;
	let mut drivers:Vec<TTYDriver> = Vec::new ();
	println! ("{}", drivers_str);
	Ok(drivers)
}