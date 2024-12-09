use regex::Regex;

const INPUT: &str = include_str!("../../input/3.txt");
//const SAMPLE_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
// const SAMPLE_INPUT_2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

// https://adventofcode.com/2024/day/3
pub fn go()
{
	println!("Day 3");
	
	let time_before = std::time::Instant::now();
	let part_one_result = part_one();
	
	util::print_result("Part 1", time_before.elapsed(), "The multiplication total", &part_one_result);

	println!();
	
	let time_before = std::time::Instant::now();
	let part_two_result = part_two();
	
	util::print_result("Part 2", time_before.elapsed(), "The revised multiplication total", &part_two_result);	
}

fn parse_mul(mul: &str) -> i32
{
	let comma = mul.find(',').expect("No comma?");
	
	let one: i32 = mul[4 .. comma].to_string().parse().expect("Failed to parse first number");
	let two: i32 = mul[ (comma+1) .. (mul.len()-1) ].to_string().parse().expect("Failed to convert ti i32");

	one * two
}

fn part_one() -> i32
{
	let mul_regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").expect("Failed to create regex");

	// https://docs.rs/regex/latest/regex/struct.Regex.html#method.find_iter
	let matches: Vec<_> = mul_regex.find_iter(INPUT).map(|m| m.as_str()).collect();

	let mut total = 0;
	for m in &matches
	{
		total += parse_mul(m);
	}
	total
}

fn part_two() -> i32
{
	let mul_regex = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))").expect("Failed to create regex");

	// https://docs.rs/regex/latest/regex/struct.Regex.html#method.find_iter
	let matches: Vec<_> = mul_regex.find_iter(INPUT).map(|m| m.as_str()).collect();

	let mut total = 0;
	let mut enabled = true;
	for m in &matches
	{
		match *m
		{
			"do()" => enabled = true,
			"don't()" => enabled = false,
			_ => {
				if enabled
				{
					total += parse_mul(m);
				}
			}
		}
	}
	total
}
