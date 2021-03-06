use common::{read_all_lines, DoubletSum, IterExt};
use lazy_static::lazy_static;
use std::ops::RangeInclusive;

static ECL: [&'static str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
lazy_static! {
	static ref FS: [(&'static str, Box<dyn Fn(&str) -> bool + Send + Sync>); 7] = [
		("byr", Box::new(|x: &str| vyr(x, 1920..=2002))),
		("iyr", Box::new(|x: &str| vyr(x, 2010..=2020))),
		("eyr", Box::new(|x: &str| vyr(x, 2020..=2030))),
		(
			"hgt",
			Box::new(|x: &str| {
				let (n, u) = x.split_at(x.len().saturating_sub(2));
				match (n.parse::<u32>(), u) {
					(Ok(n), "cm") => n >= 150 && n <= 193,
					(Ok(n), "in") => n >= 59 && n <= 76,
					_ => false,
				}
			}),
		),
		(
			"hcl",
			Box::new(|x: &str| {
				let (a, b) = x.split_at(1);
				a == "#" && b.len() == 6 && b.chars().all(|c| c.is_ascii_hexdigit())
			})
		),
		("ecl", Box::new(|x: &str| ECL.contains(&x))),
		(
			"pid",
			Box::new(|x: &str| x.len() == 9 && x.chars().all(|c| c.is_numeric()))
		),
	];
}

fn vyr(x: &str, r: RangeInclusive<u32>) -> bool {
	x.parse::<u32>().map(|y| r.contains(&y)).unwrap_or(false)
}

fn pc(x: String) -> (usize, usize) {
	x.split(" ")
		.filter_map(|x| {
			let mut s = x.split(":");
			let (k, v) = s.next_doublet()?;
			FS.iter().find(|(f, _)| f == &k).map(|(_, f)| f(v))
		})
		.map(|x| (1, x))
		.doublet_sum()
}

fn main() {
	let (a, b) = read_all_lines("04")
		.peekable()
		.batching(|it| {
			it.peek()?;
			let v = it
				.take_while(|x| !x.is_empty())
				.map(pc)
				.doublet_sum::<usize, usize>();
			Some(v)
		})
		.map(|(a, b)| (a == FS.len(), b == FS.len()))
		.doublet_sum::<usize, usize>();
	println!("{}", a);
	println!("{}", b);
}
