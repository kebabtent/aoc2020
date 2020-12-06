use common::{read_all_lines, DoubleSum};
use itertools::Itertools;

fn main() {
	let (a, b) = read_all_lines("06")
		.batching(|it| {
			let mut it = it.peekable();
			it.peek()?;
			let (a, b) = it.take_while(|l| !l.is_empty()).fold((0, !0), |(a, b), x| {
				let m = x
					.chars()
					.filter_map(|c| c.to_digit(36))
					.map(|c| c - 10)
					.fold(0u32, |a, c| a | 1 << c);
				(a | m, b & m)
			});
			Some((a.count_ones(), b.count_ones()))
		})
		.double_sum::<u32, u32>();
	println!("{}", a);
	println!("{}", b);
}
