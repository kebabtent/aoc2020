use common::read_lines;

fn main() {
	let rs = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
	let mut ls = read_lines("03").peekable();
	let l = ls.peek().unwrap().len();
	let (_, t) = ls
		.enumerate()
		.fold(([0; 5], [0; 5]), |(mut a, mut b), (i, x)| {
			let _ = rs
				.iter()
				.zip(a.iter_mut().zip(b.iter_mut()))
				.map(|(&(rr, rd), (a, b))| {
					if i % rd == 0 {
						let t = x.chars().skip(*a).next().unwrap() == '#';
						*b += t as u64;
						*a = (*a + rr) % l;
					}
				})
				.collect::<()>();
			(a, b)
		});

	println!("{}", t[1]);
	println!("{}", t.iter().product::<u64>());
}
