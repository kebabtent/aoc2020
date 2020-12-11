use common::{read_lines, IterExt};
use std::iter;

fn main() {
	let mut r = iter::once(0)
		.chain(read_lines("10").filter_map(|l| l.parse::<u8>().ok()))
		.collect::<Vec<_>>();
	r.sort_unstable();
	r.push(r.last().copied().unwrap() + 3);

	let a = r
		.iter()
		.zip(r.iter().skip(1))
		.map(|(&a, &b)| b - a - 1)
		.fold(0u32, |a, x| a + (1 << 8 * x));
	let b = r
		.iter()
		.zip(r.iter().skip(1))
		.map(|(a, b)| b - a)
		.peekable()
		.batching(|it| {
			it.peek()?;
			let n = it.take_while(|&v| v < 3).count().saturating_sub(1);
			Some(n * (n + 1) / 2 + 1)
		})
		.product::<usize>();
	println!("{}", a);
	println!("{}", b);
}
