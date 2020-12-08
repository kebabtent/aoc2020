use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn read_all_lines(day: &str) -> impl Iterator<Item = String> {
	BufReader::new(File::open(format!("input/day{}.txt", day)).unwrap())
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

macro_rules! impl_bool_add {
	( $( $x:ty ),* ) => {
		$(
			impl Add<bool> for $x {
				type Output = $x;
				fn add(self, rhs: bool) -> Self::Output {
					self + rhs as $x
				}
			}
		)*
	};
}

impl_bool_add!(u32, u64, usize);

pub trait DoubletSum<A, B>: Iterator<Item = (A, B)> + Sized {
	fn doublet_sum<SA, SB>(self) -> (SA, SB)
	where
		SA: Add<A, Output = SA> + Default,
		SB: Add<B, Output = SB> + Default,
	{
		self.fold((SA::default(), SB::default()), |(a, b), (c, d)| {
			(a.add(c), b.add(d))
		})
	}
}

impl<T, A, B> DoubletSum<A, B> for T where T: Iterator<Item = (A, B)> {}

pub trait IterExt: Iterator + Sized {
	fn batching<B, F>(self, f: F) -> Batching<Self, F>
	where
		F: FnMut(&mut Self) -> Option<B>,
	{
		Batching { f, iter: self }
	}
}

impl<T> IterExt for T where T: Iterator {}

pub struct Batching<I, F> {
	f: F,
	iter: I,
}

impl<B, F, I> Iterator for Batching<I, F>
where
	I: Iterator,
	F: FnMut(&mut I) -> Option<B>,
{
	type Item = B;
	fn next(&mut self) -> Option<B> {
		(self.f)(&mut self.iter)
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(0, None)
	}
}
