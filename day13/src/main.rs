use common::read_lines;

fn main() {
	let mut l = read_lines("13");
	let d = l.next().unwrap().parse::<u32>().unwrap();
	let l = l.next().unwrap();
	let a = l
		.split(",")
		.filter_map(|v| v.parse::<u32>().ok())
		.min_by_key(|&v| v - d % v)
		.map(|v| v * (v - d % v))
		.unwrap();
	let (b, _) = l
		.split(",")
		.enumerate()
		.filter_map(|(i, v)| Some((i as u64, v.parse::<u64>().ok()?)))
		.fold((0, 1), |(mut b, mut p), (x, y)| {
			while (b + x) % y != 0 {
				b += p;
			}
			p *= y;
			(b, p)
		});
	println!("{}", a);
	println!("{}", b);
}
