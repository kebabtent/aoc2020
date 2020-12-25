use common::read_lines;
use std::collections::HashMap;

fn r(m: usize) -> usize {
	let mut h = HashMap::with_capacity(2048);
	let mut s = 0;
	let mut a = 0;
	for v in read_lines("15")
		.next()
		.unwrap()
		.split(",")
		.filter_map(|v| v.parse::<usize>().ok())
	{
		h.insert(v, s);
		s += 1;
	}
	for i in s..(m - 1) {
		a = if let Some(v) = h.insert(a, i) {
			i - v
		} else {
			0
		};
	}
	a
}

fn main() {
	println!("{}", r(2020));
	println!("{}", r(30_000_000));
}
