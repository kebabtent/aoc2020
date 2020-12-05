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
