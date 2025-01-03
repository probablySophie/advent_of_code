use std::time::{Duration, Instant};

use util::{find_in, ConnectedPoint, MapLoc};

#[allow(unused)]
const INPUT: &str = include_str!("../../input/23.txt");
#[allow(unused)]
const EXAMPLE_INPUT_1: &str = include_str!("../../input/23_example_1.txt");
#[allow(unused)]
const EXAMPLE_INPUT_2: &str = "";

// For if we need to change it up!
type ResultType = i32;

//https://adventofcode.com/2024/day/23
pub fn go(print_results: bool) -> (Duration, Duration, Duration)
{
	if print_results {println!("Day 23");}
	
	let time_before = Instant::now();
	// ~ ~ ~ ~ ~ PRE CALCULATION ~ ~ ~ ~ ~

	let computers = parse_input(EXAMPLE_INPUT_1, "Example 1", print_results);
	
	// TODO: Do any pre-calculation here
	// let parsed = parse_input(EXAMPLE_INPUT_1, "Example 1", print_results);

	// ~ ~ ~ ~ ~ END OF PRE CALCULATION ~ ~ ~ ~ ~
	let pre_calc_time = time_before.elapsed();
	if print_results { util::print_precalc(pre_calc_time) };

	// Part 1
	TimedRun!(time_before, part_one_result, part_one(&computers), part_one_time);

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

fn part_one(computers: &[ConnectedPoint<ResultType>]) -> ResultType
{
	let t_num = "abcdefghijklmnopqrstuvwxyz".find('t').unwrap();
	// Make a set of all of the loops
	// For each set of 3, if any have the x value of t_num, inc our count
	let mut possible_rings = 0;



	possible_rings
}

fn part_two() -> ResultType
{
	0
}

fn parse_input(input: &str, name: &str, print_results: bool) -> Vec<ConnectedPoint<ResultType>>
{
	if print_results { println!("Parsing: {name}") };

	let mut computer_points = Vec::new();
	let mut computer_connections = Vec::new();

	for line in input.lines()
	{
		let Some(dash) = line.find('-')
		else { continue }; // Probably an empty line
		
		// print!("{line}  {} {}", &line[0..dash], &line[dash+1..]);
		
		let computer_1 = name_to_usize_pair(&line[0..dash]);
		let computer_2 = name_to_usize_pair(&line[dash+1..]);

		// print!("  {computer_1:2?} {computer_2:2?}");
		if ! find_in(&computer_points, &computer_1)
		{
			computer_points.push(computer_1);
		}
		if ! find_in(&computer_points, &computer_2)
		{
			computer_points.push(computer_2);
		}
		computer_connections.push( ( computer_1, computer_2, 1 ) );
		// println!();
	}
	let points = ConnectedPoint::new_points(computer_points, computer_connections, 0);
	// for point in &points
	// {
	// 	println!("{point:?}");
	// }
	if print_results { println!("Created {} computers", points.len()) };

	points
}

fn name_to_usize_pair(computer_name: &str) -> MapLoc
{
	assert_eq!(computer_name.len(), 2, "The given computer name must be 2 chars long");
	let chars: Vec<char> = computer_name.chars().collect();

	let letters = "abcdefghijklmnopqrstuvwxyz";

	let x = letters.find(chars[0]).unwrap();
	let y = letters.find(chars[1]).unwrap();

	(x, y)
}
fn usize_pair_to_name(usize_pair: MapLoc) -> String
{
	let letters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
	let char_1 = letters[usize_pair.0];
	let char_2 = letters[usize_pair.1];

	[char_1, char_2].to_vec().iter().collect::<String>()
}
