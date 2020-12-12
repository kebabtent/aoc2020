use common::read_lines;
use X::*;

#[derive(Clone, Copy, PartialEq)]
enum X {
	F,
	E,
	O,
}

impl From<char> for X {
	fn from(c: char) -> Self {
		match c {
			'.' => F,
			'L' => E,
			'#' => O,
			_ => unreachable!(),
		}
	}
}

fn n(d: &[(isize, isize)], f: bool, p: &Vec<Vec<X>>, ix: isize, iy: isize) -> usize {
	let mut c = 0;
	let ly = p.len() as isize;
	let lx = p[0].len() as isize;
	for &(dx, dy) in d {
		let mut x = ix;
		let mut y = iy;
		loop {
			x += dx;
			y += dy;
			if x < 0 || x >= lx || y < 0 || y >= ly {
				break;
			}

			let v = p[y as usize][x as usize];
			if f || v != F {
				if v == O {
					c += 1;
				}
				break;
			}
		}
	}
	c
}

fn u(d: &[(isize, isize)], f: bool, mut p: Vec<Vec<X>>) -> usize {
	let mut a;
	let t = if f { 4 } else { 5 };
	loop {
		a = 0;
		let q = p.clone();
		for y in 0..p.len() {
			for x in 0..p[0].len() {
				let n = n(d, f, &q, x as isize, y as isize);
				p[y][x] = match q[y][x] {
					E if n == 0 => O,
					O if n >= t => E,
					v => v,
				};
				if p[y][x] == O {
					a += 1;
				}
			}
		}
		if p == q {
			break;
		}
	}
	a
}

fn main() {
	let mut d = Vec::with_capacity(8);
	for i in -1isize..=1 {
		for j in -1isize..=1 {
			if i == 0 && j == 0 {
				continue;
			}
			d.push((i, j));
		}
	}

	let p: Vec<Vec<X>> = read_lines("11")
		.map(|l| l.chars().map(|c| c.into()).collect())
		.collect();

	let a = u(&d, true, p.clone());
	let b = u(&d, false, p);
	println!("{}", a);
	println!("{}", b);
}
