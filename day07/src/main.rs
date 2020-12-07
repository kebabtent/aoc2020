use common::read_lines;
use numext_fixed_uint::U1024;
use std::collections::HashMap;

fn ai(mi: &Vec<Vec<usize>>, i: usize, b: &mut U1024) {
	for &j in &mi[i] {
		*b |= U1024::one() << j;
		ai(mi, j, b);
	}
}

fn a(mi: &Vec<Vec<usize>>, i: usize) -> u32 {
	let mut b = U1024::zero();
	ai(mi, i, &mut b);
	b.count_ones()
}

fn b(m: &Vec<Vec<(usize, u32)>>, i: usize) -> u32 {
	let mut c = 0;
	for &(j, p) in &m[i] {
		c += p * (b(&m, j) + 1);
	}
	c
}

fn main() {
	let is = read_lines("07")
		.map(|l| l.split(" bags contain ").next().unwrap().to_owned())
		.enumerate()
		.map(|(v, k)| (k, v))
		.collect::<HashMap<_, _>>();

	let mut m = Vec::<Vec<_>>::with_capacity(is.len());
	m.resize_with(is.len(), Default::default);
	let mut mi = Vec::<Vec<_>>::with_capacity(is.len());
	mi.resize_with(is.len(), Default::default);

	for l in read_lines("07") {
		let mut l = l.split(" bags contain ");
		let v = *is.get(l.next().unwrap()).unwrap();
		let ks = l.next().unwrap();
		if ks.starts_with("no ") {
			continue;
		}
		for k in ks.split(", ") {
			let mut x = k.match_indices(" ");
			let i = x.next().unwrap().0;
			let j = x.last().unwrap().0;
			let (c, k) = k.split_at(j).0.split_at(i + 1);
			let c = c.trim_end().parse::<u32>().unwrap();
			let k = *is.get(k).unwrap();
			m[v].push((k, c));
			mi[k].push(v);
		}
	}

	let i = *is.get("shiny gold").unwrap();
	println!("{}", a(&mi, i));
	println!("{}", b(&m, i));
}
