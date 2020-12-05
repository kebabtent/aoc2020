use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn read_all_lines(day: &str) -> impl Iterator<Item = String> {
	BufReader::new(File::open(format!("day{}.txt", day)).unwrap())
		.lines()
		.map(|l| l.unwrap())
}

pub fn read_lines(day: &str) -> impl Iterator<Item = String> {
	read_all_lines(day).filter(|l| !l.is_empty())
}

pub fn try_read_lines_as<T: FromStr>(day: &str) -> impl Iterator<Item = Result<T, String>> {
	read_lines(day).map(|l| T::from_str(&l).map_err(|_| l))
}

pub fn read_lines_as<T: FromStr>(day: &str) -> impl Iterator<Item = T> {
	try_read_lines_as(day).map(|l| l.unwrap())
}

pub trait Add<Rhs = Self> {
	type Output;
	fn add(self, rhs: Rhs) -> Self::Output;
}

impl<T> Add<T> for T
where
	T: std::ops::Add<T>,
{
	type Output = T::Output;
	fn add(self, rhs: T) -> Self::Output {
		self + rhs
	}
}

impl Add<bool> for u32 {
	type Output = u32;
	fn add(self, rhs: bool) -> Self::Output {
		self + rhs as u32
	}
}

pub trait DoubleSum<A, B>: Iterator<Item = (A, B)> + Sized {
	fn double_sum<SA, SB>(self) -> (SA, SB)
	where
		SA: Add<A, Output = SA> + Default,
		SB: Add<B, Output = SB> + Default,
	{
		self.fold((SA::default(), SB::default()), |(a, b), (c, d)| {
			(a.add(c), b.add(d))
		})
	}
}

impl<T, A, B> DoubleSum<A, B> for T where T: Iterator<Item = (A, B)> {}
