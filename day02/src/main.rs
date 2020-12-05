use common::{read_lines, DoubleSum};
use std::ops::RangeInclusive;

enum State {
	Def,
	Rng(RangeInclusive<usize>),
	Ltr(RangeInclusive<usize>, char),
	Ok(bool, bool),
	Err,
}

impl State {
	fn update(self, x: &str) -> Self {
		use State::*;
		match self {
			Def => {
				let mut p = x.split("-").map(|y| y.parse::<usize>().unwrap());
				match (p.next(), p.next()) {
					(Some(a), Some(b)) => Rng(a..=b),
					_ => Err,
				}
			}
			Rng(r) => match x.chars().next() {
				Some(l) => Ltr(r, l),
				None => Err,
			},
			Ltr(r, l) => {
				let v1 = r.contains(&x.matches(l).count());
				let (a, b) = r.into_inner();
				let mut c = x.chars().skip(a - 1);
				let v2 = match (c.next(), c.skip(b - a - 1).next()) {
					(Some(a), Some(b)) => (a == l) ^ (b == l),
					_ => false,
				};
				Ok(v1, v2)
			}
			s => s,
		}
	}

	fn inner(self) -> Option<(u32, u32)> {
		match self {
			State::Ok(a, b) => Some((a as u32, b as u32)),
			_ => None,
		}
	}
}

fn main() {
	let (a, b): (u32, u32) = read_lines("02")
		.map(|l| l.split(" ").fold(State::Def, |s, x| s.update(x)))
		.filter_map(|s| s.inner())
		.double_sum();
	println!("{}", a);
	println!("{}", b);
}
