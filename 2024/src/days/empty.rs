#[allow(unused)]
const INPUT: &str = include_str!("../../input/~DAY_NUM~.txt");
#[allow(unused)]
const EXAMPLE_INPUT_1: &str = "";
#[allow(unused)]
const EXAMPLE_INPUT_2: &str = "";

// For if we need to change it up!
type ResultType = i32;

//https://adventofcode.com/2024/day/~DAY_NUM~
pub fn go()
{
	println!("Day ~DAY_NUM~");

	let time_before = std::time::Instant::now();
	let part_one_result = part_one();
	let time_elapsed = time_before.elapsed();
	
	util::print_result("Part 1", time_elapsed, "Part 1 description", &part_one_result);

	println!();
	
	let time_before = std::time::Instant::now();
	let part_two_result = part_two();
	let time_elapsed = time_before.elapsed();
	
	util::print_result("Part 2", time_elapsed, "Part 2 description", &part_two_result);	
}

fn part_one() -> ResultType
{
	0
}

fn part_two() -> ResultType
{
	0
}
