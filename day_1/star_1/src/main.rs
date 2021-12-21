use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
	let args: Vec<String> = std::env::args().collect();
	let file_path = &args[1];
	println!("Path to input file: {}", file_path);

	let file = File::open(file_path)?;
	let reader = BufReader::new(file);

	let mut count: u32 = 0;
	let mut previous: Option<u32> = None;

	for line in reader.lines() {
		// use std::string::String;

		let linev = match line?.parse::<u32>() {
			Ok(v) => {
				v
			},
			Err(_) => {
				use std::io::{ Error, ErrorKind };
				return Err(Error::new(ErrorKind::Other, "uh oh!"))
			}
		};

		match previous {
			Some(v) => {
				if v < linev {
					count = count + 1;
				}
			},
			None => {}
		}

		previous = Some(linev);
	}

	println!("{}", count);

	Ok(())

	// println!("Hello, world!");
}

