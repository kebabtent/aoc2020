use common::{read_lines, Buffer, IterExt};
use std::collections::VecDeque;

struct S {
	t: u64,
	i: VecDeque<(u64, u64, u64)>,
}

impl S {
	fn n(t: u64) -> Self {
		S {
			t,
			i: VecDeque::new(),
		}
	}

	fn u(&mut self, v: u64) -> Option<u64> {
		self.i.push_back((0, 0, 0));
		for (a, l, h) in &mut self.i {
			let f = *a == 0;
			*a += v;
			if f || v < *l {
				*l = v;
			}
			if f || v > *h {
				*h = v;
			}
			if *a > v && *a == self.t {
				return Some(*l + *h);
			}
		}

		while self.i.get(0).map(|&(a, _, _)| a >= self.t).unwrap_or(false) {
			self.i.pop_front();
		}

		None
	}
}

fn f(b: &mut Buffer<u64>, v: u64) -> Option<u64> {
	use itertools::Itertools;

	let mut r = false;
	if b.is_full() {
		r = !b
			.iter()
			.permutations(2)
			.map(|p| p.iter().map(|&&v| v).sum::<u64>())
			.any(|s| s == v);
	}
	b.push(v);
	if r {
		Some(v)
	} else {
		None
	}
}

fn main() {
	let a = read_lines("09")
		.filter_map(|l| l.parse::<u64>().ok())
		.fold_while(Buffer::new(25), |b, v| f(b, v))
		.unwrap();
	let b = read_lines("09")
		.filter_map(|l| l.parse::<u64>().ok())
		.fold_while(S::n(a), |s, v| s.u(v))
		.unwrap();
	println!("{}", a);
	println!("{}", b);
}
