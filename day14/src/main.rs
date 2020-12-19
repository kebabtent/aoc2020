use common::read_lines;
use std::collections::HashMap;

fn main() {
	let mut o = HashMap::with_capacity(1 << 9);
	let mut t = HashMap::with_capacity(1 << 17);
	let mut p = 0;
	let mut n = 0;
	let mut m = String::new();
	let mut r = String::with_capacity(36);
	let mut s = String::with_capacity(36);
	for l in read_lines("14") {
		if l.starts_with("mask") {
			p = u64::from_str_radix(&l[7..].replace("X", "0"), 2).unwrap();
			n = u64::from_str_radix(&l[7..].replace("X", "1"), 2).unwrap();
			m = l[7..].to_owned();
		} else {
			let mut l = l[4..].split("] = ");
			let x = l.next().unwrap().parse::<u64>().unwrap();
			let y = l.next().unwrap().parse::<u64>().unwrap();
			*o.entry(x).or_insert(0) = (y | p) & n;
			let c = 1 << m.matches("X").count();
			for i in 0..c {
				r.clear();
				s.clear();
				let mut q = m.split("X").enumerate().peekable();
				while let Some((j, b)) = q.next() {
					r += b;
					s += &b.replace("0", "1");
					if q.peek().is_some() {
						if i & (1 << j) > 0 {
							r += "1";
							s += "1";
						} else {
							r += "0";
							s += "0";
						}
					}
				}
				let u = u64::from_str_radix(&r, 2).unwrap();
				let v = u64::from_str_radix(&s, 2).unwrap();
				*t.entry((x | u) & v).or_insert(0) = y;
			}
		}
	}
	let a = o.values().sum::<u64>();
	let b = t.values().sum::<u64>();
	println!("{}", a);
	println!("{}", b);
}
