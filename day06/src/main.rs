use common::{read_all_lines, DoubletSum};
use itertools::Itertools;

fn main() {
	let (a, b) = read_all_lines("06")
		.peekable()
		.batching(|it| {
			it.peek()?;
			let (a, b) = it.take_while(|l| !l.is_empty()).fold((0, !0), |(a, b), x| {
				let m = x
					.chars()
					.filter_map(|c| c.to_digit(36))
					.fold(0u64, |a, c| a | 1 << c);
				(a | m, b & m)
			});
			Some((a.count_ones(), b.count_ones()))
		})
		.doublet_sum::<u32, u32>();
	println!("{}", a);
	println!("{}", b);
}
