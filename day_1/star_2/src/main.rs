use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
	let args: Vec<String> = std::env::args().collect();
	let file_path = &args[1];
	println!("Path to input file: {}", file_path);

	let file = File::open(file_path)?;
	let reader = BufReader::new(file);

	let mut count: u32 = 0;
	let mut hcount: u32 = 0;
	let mut lcount: u32;
	let mut previous: [Option<u32>; 3] = [None; 3];

	'linel: for line in reader.lines() {
		let linev = match line?.parse::<u32>() {
			Ok(v) => {
				v
			},
			Err(_) => {
				use std::io::{ Error, ErrorKind };
				return Err(Error::new(ErrorKind::Other, "uh oh!"))
			}
		};

		lcount = hcount;

		hcount = 0;
		for d in previous {
			match d {
				Some(d) => {
					hcount += d
				},
				None => {
					previous.rotate_right(1);
					previous[0] = Some(linev);
					continue 'linel;
				}
			}
		}

		if hcount > lcount {
			count += 1;
		}

		previous.rotate_right(1);
		previous[0] = Some(linev);
	}

	println!("{}", count);

	Ok(())
}

