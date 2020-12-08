use common::read_lines;
use numext_fixed_uint::U1024;
use std::iter;
use O::*;

struct S {
	i: usize,
	a: usize,
}

impl S {
	fn n() -> Self {
		Self { i: 0, a: 0 }
	}

	fn u(mut self, di: i16, da: i16) -> Self {
		self.i = (self.i as i16 + di) as usize;
		self.a = (self.a as i16 + da) as usize;
		self
	}
}

#[derive(Copy, Clone, PartialEq)]
enum O {
	N(i16),
	A(i16),
	J(i16),
	K,
}

impl O {
	fn p(i: &str) -> Option<Self> {
		let j = i[4..].parse().ok()?;
		let o = match &i[..3] {
			"nop" => N(j),
			"acc" => A(j),
			"jmp" => J(j),
			_ => return None,
		};
		Some(o)
	}

	fn a(&self, s: S) -> S {
		match *self {
			N(_) => s.u(1, 0),
			A(x) => s.u(1, x),
			J(x) => s.u(x, 0),
			K => s,
		}
	}

	fn f(&mut self) {
		*self = match *self {
			N(x) => J(x),
			J(x) => N(x),
			x => x,
		};
	}
}

fn r(o: &Vec<O>) -> S {
	let mut s = S::n();
	let mut b = U1024::zero();
	loop {
		b.set_bit(s.i, true);
		s = o[s.i].a(s);
		if b.bit(s.i).unwrap() {
			break;
		}
	}
	s
}

fn main() {
	let mut o = read_lines("08")
		.filter_map(|l| O::p(&l))
		.chain(iter::once(K))
		.collect::<Vec<_>>();

	let a = r(&o).a;

	let mut t = 0;
	let b;
	loop {
		o[t].f();
		let s = r(&o);
		if o[s.i] == K {
			b = s.a;
			break;
		}
		o[t].f();
		t += 1;
	}

	println!("{}", a);
	println!("{}", b);
}
