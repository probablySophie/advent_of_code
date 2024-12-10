use std::time::{Duration, Instant};

#[allow(unused)]
// const INPUT: &str = include_str!("../../input/~DAY_NUM~.txt");
#[allow(unused)]
const EXAMPLE_INPUT_1: &str = "";
#[allow(unused)]
const EXAMPLE_INPUT_2: &str = "";

// For if we need to change it up!
type ResultType = i32;

//https://adventofcode.com/2024/day/~DAY_NUM~
pub fn go(print_results: bool) -> (Duration, Duration, Duration)
{
	println!("Day ~DAY_NUM~");
	
	let time_before = Instant::now();
	// TODO: Do any pre-calculation here

	let pre_calc_time = time_before.elapsed();

	TimedRun!(time_before, part_one_result, part_one(), part_one_time);

	if print_results
	{
		util::print_result("Part 1", part_one_time, "Part 1 description", &part_one_result);
	}
	
	TimedRun!(time_before, part_two_result, part_two(), part_two_time);
	
	if print_results
	{
		println!();
		util::print_result("Part 2", part_two_time, "Part 2 description", &part_two_result);
	}

	// Return how long it took!
	(pre_calc_time, part_one_time, part_two_time)	
}

fn part_one() -> ResultType
{
	0
}

fn part_two() -> ResultType
{
	0
}
