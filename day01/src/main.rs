use common::read_lines_as;
use itertools::Itertools;

fn perm(k: usize) -> u32 {
	read_lines_as::<u32>("01")
		.permutations(k)
		.filter(|p| p.iter().sum::<u32>() == 2020)
		.next()
		.unwrap()
		.into_iter()
		.fold(1, |a, v| a * v)
}

fn main() {
	println!("{}", perm(2));
	println!("{}", perm(3));
}
