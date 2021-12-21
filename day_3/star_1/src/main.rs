use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::vec;

fn bin_to_dec(s: &str) -> u32 {
	let mut o = 0u32;
	for (i, c) in s.chars().enumerate() {
		if c == '1' {
			o += 1 << i;
		}
	}
	o
}

fn main() -> io::Result<()> {
	let args: Vec<String> = std::env::args().collect();
	let file_path = &args[1];
	println!("Path to input file: {}", file_path);

	let file = match File::open(file_path) {
		Ok(f) => f,
		Err(_) => {
			println!("Failed to open file!");
			return Ok(());
		}
	};
	let reader = BufReader::new(file);

	let mut line_count = 0u32;

	let mut common: Vec<u32> = vec![];

	for line in reader.lines() {
		let l = match line {
			Ok(l) => l,
			Err(_) => { continue; }
		};
		// let mut i = 0;
		for (i, c) in l.trim().chars().enumerate() {
			// println!("{} - {}", i, common.len());
			if common.len() <= i {
				common.push(0);
			}
			if c == '1' {
				common[i] += 1;
			}
			// i += 1;
		}
		line_count += 1;
	}

	let mut a = String::new();

	for x in &common {
		if *x >= line_count / 2 {
			a.push('1');
		} else {
			a.push('0');
		}
	}

	a = a.chars().rev().collect::<String>();

	let gamma_rate = bin_to_dec(a.as_str());

	let er_tmp = !bin_to_dec(a.as_str());
	let mut mask = 0u32;
	for x in 0..common.len() {
		mask += 1 << x;
	}
	let epsilon_rate = mask & er_tmp;

	let power_consumption = gamma_rate * epsilon_rate;

	println!("Gamma rate: {}", gamma_rate);
	println!("Epsilon rate: {}", epsilon_rate);
	println!("Power consumption: {}", power_consumption);

	Ok(())
}

