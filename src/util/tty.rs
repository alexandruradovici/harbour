use std::io;
use std::fs;

#[derive(Debug)]
pub struct TTYDriver
{
	pub driver: String,
	pub file_entry: String,
	pub major: u8,
	pub minor_min: u32,
	pub minor_max: u32,
	pub driver_type: String
}

pub fn read_drivers () -> Result<Vec<TTYDriver>, io::Error>
{
	let drivers_str = fs::read_to_string ("/proc/tty/drivers")?;
	let mut drivers:Vec<TTYDriver> = Vec::new ();
	println! ("{}", drivers_str);
	for driver_str in drivers_str.split("\n") {
		if driver_str.len () > 0 {
			let driver_parts_str:Vec<&str> = driver_str.split_whitespace ().collect();
			let minor_str:Vec<&str> = driver_parts_str[3].split ("-").collect ();
			let tty_driver = TTYDriver {
				driver: driver_parts_str[0].to_string (),
				file_entry: if let Ok (inf) = fs::metadata(driver_parts_str[1]) {
					if inf.is_dir () { format! ("{}/", driver_parts_str[1]) } else { driver_parts_str[1].to_string () }	
				}
				else {
					driver_parts_str[1].to_string ()
				},
				major: driver_parts_str[2].parse::<u8> ().unwrap_or (0),
				minor_min: minor_str[0].parse::<u32>().unwrap_or (0),
				minor_max: if minor_str.len () > 1 { minor_str[1].parse::<u32>().unwrap_or(0) } else { 0 },
				driver_type: driver_parts_str[4].to_string ()
			};
			println! ("driver -> {:?}", tty_driver);
			drivers.push (tty_driver);
		}
	}
	Ok(drivers)
}
