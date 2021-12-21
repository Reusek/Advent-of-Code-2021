use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::convert::{ TryInto };
use std::ops::{ AddAssign, Mul };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Inst {
	Forward,
	Up,
	Down,
	Nothing
}


#[derive(Debug, Clone, Copy)]
struct Turn {
	horizontal: i32,
	depth: i32,
	aim: i32,
	inst: Inst,
}

impl Mul for Turn {
	type Output = Turn;
	fn mul(self, rhs: Self) -> Self::Output {
		let a = if rhs.aim == 0 { 1 } else { rhs.aim };
		Turn {
			horizontal: self.horizontal,
			depth: self.depth,
			aim: self.aim * a,
			inst: self.inst
		}
	}
}

impl AddAssign for Turn {
	fn add_assign(&mut self, rhs: Self) {
		*self = Self {
			horizontal: self.horizontal + rhs.horizontal,
			depth: self.depth + rhs.depth,
			aim: self.aim + rhs.aim,
			inst: self.inst.clone()
		};
	}
}

impl TryInto<Turn> for String {
	type Error = ();
	fn try_into(self) -> Result<Turn, Self::Error> {
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
				return Ok(Turn {horizontal: a, depth: a, aim: 0, inst: Inst::Forward})
			},
			"up" => {
				return Ok(Turn {horizontal: 0, depth: 0, aim: a * -1, inst: Inst::Up})
			},
			"down" =>  {
				return Ok(Turn {horizontal: 0, depth: 0, aim: a, inst: Inst::Down})
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

	let mut pos = Turn { horizontal: 0, depth: 0, aim: 0, inst: Inst::Nothing};

	for line in reader.lines() {
		let turn: Turn = match line?.try_into() {
			Ok(t) => t,
			Err(_) => {continue;}
		};

		if turn.inst == Inst::Forward {
			if pos.aim != 0 {
				pos.depth += turn.depth * pos.aim;
			}
			pos.horizontal += turn.horizontal;
		} else {
			pos += turn;
		}
	}

	let result = pos.horizontal * pos.depth;

	println!("{}", result);

	Ok(())
}

