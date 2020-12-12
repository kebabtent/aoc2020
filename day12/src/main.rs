use common::read_lines;
use std::mem;
use std::ops::{AddAssign, Mul};

#[derive(Clone, Copy)]
struct P {
	x: i32,
	y: i32,
}

impl P {
	fn n(x: i32, y: i32) -> Self {
		Self { x, y }
	}

	fn z() -> Self {
		Self::n(0, 0)
	}

	fn m(&self) -> i32 {
		self.x.abs() + self.y.abs()
	}

	fn r(&mut self, v: u32) -> Self {
		for _ in 0..v {
			self.x *= -1;
			mem::swap(&mut self.x, &mut self.y);
		}
		*self
	}
}

impl AddAssign for P {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl Mul<i32> for P {
	type Output = P;
	fn mul(mut self, rhs: i32) -> P {
		self.x *= rhs;
		self.y *= rhs;
		self
	}
}

fn main() {
	let o = (0..4).map(|i| P::n(0, 1).r(i)).collect::<Vec<_>>();
	let (_, p, s, _) = read_lines("12").fold(
		(1, P::z(), P::z(), P::n(10, 1)),
		|(mut d, mut p, mut s, mut w), l| {
			let v = l[1..].parse::<i32>().unwrap();
			let l = l.chars().next().unwrap();
			let m = match l {
				'N' => Some(0),
				'E' => Some(1),
				'S' => Some(2),
				'W' => Some(3),
				'F' => Some(d),
				_ => None,
			};
			if let Some(m) = m {
				let z = o[m as usize] * v;
				p += z;
				if l == 'F' {
					s += w * v;
				} else {
					w += z;
				}
			} else {
				let v = v / 90;
				let r = if l == 'L' { 4 - v } else { v };
				d = (d + r) % 4;
				w.r(r as u32);
			}

			(d, p, s, w)
		},
	);
	println!("{}", p.m());
	println!("{}", s.m());
}
