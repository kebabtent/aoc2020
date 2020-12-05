use common::read_lines;

fn s(x: &str) -> u16 {
	let x = x
		.replace("F", "0")
		.replace("B", "1")
		.replace("L", "0")
		.replace("R", "1");
	u16::from_str_radix(&x, 2).unwrap()
}

fn main() {
	let mut ss: Vec<_> = read_lines("05").map(|l| s(&l)).collect();
	ss.sort_unstable();
	let i = ss
		.iter()
		.zip(ss.iter().skip(1))
		.find(|(&a, &b)| a + 1 != b)
		.map(|(&a, _)| a + 1)
		.unwrap();
	println!("{}", ss.last().unwrap());
	println!("{}", i);
}
