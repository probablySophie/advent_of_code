use std::time::{Duration, Instant};

// #[allow(unused)]
// const INPUT: &str = include_str!("../../input/25.txt");
#[allow(unused)]
const EXAMPLE_INPUT_1: &str = "";
#[allow(unused)]
const EXAMPLE_INPUT_2: &str = "";

// For if we need to change it up!
type ResultType = i32;

//https://adventofcode.com/2024/day/25
pub fn go(print_results: bool) -> (Duration, Duration, Duration)
{
	if print_results {println!("Day 25");}
	
	let time_before = Instant::now();
	// ~ ~ ~ ~ ~ PRE CALCULATION ~ ~ ~ ~ ~
	
	// TODO: WARN: Remember to un-comment today's line in ../main.rs
	// TODO: Do any pre-calculation here
	// let parsed = parse_input(EXAMPLE_INPUT_1, "Example 1", print_results);

	// ~ ~ ~ ~ ~ END OF PRE CALCULATION ~ ~ ~ ~ ~
	let pre_calc_time = time_before.elapsed();
	if print_results { util::print_precalc(pre_calc_time) };

	// Part 1
	TimedRun!(time_before, part_one_result, part_one(), part_one_time);

	if print_results
	{
		util::print_result("Part 1", part_one_time, "Part 1 description", &part_one_result);
	}

	// Part 2
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

fn parse_input<'a>(input: &'a str, name: &str, print_results: bool)
{
	if print_results { println!("Parsing: {name}") };
	
}
