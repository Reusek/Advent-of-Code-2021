use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::convert::{ TryInto };
use std::ops::{ AddAssign };

#[derive(Debug)]
struct Turn {
	x: i32,
	y: i32,
}

impl AddAssign for Turn {
	fn add_assign(&mut self, other: Self) {
		*self = Self {
			x: self.x + other.x,
			y: self.y + other.y,
		};
	}
}

impl TryInto<Turn> for String {
	type Error = ();
	fn try_into(self) -> Result<Turn, Self::Error> {
		let mut x: i32 = 0;
		let mut y: i32 = 0;
		
		let ds = self.split(" ").collect::<Vec<&str>>();
		if ds.len() != 2 {
			return Err(());
		}

		let a = match ds[1].to_string().parse::<i32>() {
			Ok(d) => d,
			Err(_) => return Err(())
		};

		match ds[0] {
			"forward" => {
				return Ok(Turn {x: a, y: 0})
			},
			"up" => {
				return Ok(Turn {x: 0, y: a * -1})
			},
			"down" =>  {
				return Ok(Turn {x: 0, y: a})
			},
			_ => return Err(())
		}
	}
}


fn main() -> io::Result<()> {
	let args: Vec<String> = std::env::args().collect();
	let file_path = &args[1];
	println!("Path to input file: {}", file_path);

	let file = File::open(file_path)?;
	let reader = BufReader::new(file);

	let mut pos = Turn {x: 0, y: 0};

	for line in reader.lines() {
		// let turn: Result<Turn, _> = line?.try_into();
		let turn: Turn = match line?.try_into() {
			Ok(t) => t,
			Err(_) => {continue;}
		};

		pos += turn;
	}

	let result = pos.x * pos.y;

	println!("{}", result);

	Ok(())
}

