use std::time::{Duration, Instant};

use util::{MapFunction, MapLoc, PairFunctions, VecMap};

#[allow(unused)]
const INPUT: &str = include_str!("../../input/18.txt");
#[allow(unused)]
const EXAMPLE_INPUT_1: &str = include_str!("../../input/18_example_1.txt");

// For if we need to change it up!
type ResultType = i32;

//https://adventofcode.com/2024/day/18
pub fn go(print_results: bool) -> (Duration, Duration, Duration)
{
	if print_results {println!("Day 18");}
	
	let time_before = Instant::now();
	// ~ ~ ~ ~ ~ PRE CALCULATION ~ ~ ~ ~ ~

	// let coords = parse_input(EXAMPLE_INPUT_1); let size = 6; let i = 12;
	let coords = parse_input(INPUT); let size = 70; let i = 1024;
	
	// ~ ~ ~ ~ ~ END OF PRE CALCULATION ~ ~ ~ ~ ~
	let pre_calc_time = time_before.elapsed();
	if print_results { util::print_precalc(pre_calc_time) };

	// Part 1
	TimedRun!(time_before, part_one_result, part_one(&coords, size, i), part_one_time);

	if print_results
	{
		util::print_result("Part 1", part_one_time, "Steps required to reach the exit", &part_one_result);
	}

	// Part 2
	TimedRun!(time_before, part_two_result, part_two(&coords, size), part_two_time);
	
	if print_results
	{
		println!();
		util::print_result("Part 2", part_two_time, "The specfic block that makes exiting impossible is", &part_two_result);
	}

	// Return how long it took!
	(pre_calc_time, part_one_time, part_two_time)	
}

fn part_one(coords: &[MapLoc], size: usize, iterations: usize) -> ResultType
{
	let mut grid: VecMap<char> = vec![ vec!['.'; size+1]; size+1 ];
	let goal: MapLoc = (grid.len() - 1, grid.len() - 1);


	(0..iterations).for_each(|i|
		{
			grid.set(coords[i], '#');
		}
	);

	let steps = util::pathfinding::a_star::get_shortest_distance(&grid, &['#'], (0, 0), goal);

	steps.unwrap().try_into().unwrap()
}

fn part_two(coords: &[MapLoc], size: usize) -> String
{
	let mut grid: VecMap<char> = vec![ vec!['.'; size+1]; size+1 ];
	let goal: MapLoc = (grid.len() - 1, grid.len() - 1);
	
	let mut i = 0;
	// Do 1000 steps, check if we're good.  Do another 1000.
	i = go_forwards(&mut grid, coords, i, goal, 1000);
	// Now we know that coords[i] is bad

	// Now go backwards by 100 - check if good, do another 100
	i = go_backwards(&mut grid, coords, i, goal, 100);
	// Now we know that coords[i] is good

	// And then forwards by 10
	i = go_forwards(&mut grid, coords, i, goal, 10);
	// And again, coords[i] is bad

	// And then backwards by 1
	i = go_backwards(&mut grid, coords, i, goal, 1);
	// So now, coords[i] is the LAST GOOD BLOCK TO FALL

	// SOOO i+1 is the block that makes leaving impossible!
	let just_fell = coords[i+1];
	
	just_fell.x().to_string() + "," + &just_fell.y().to_string()
}


fn go_forwards(grid: &mut VecMap<char>, coords: &[MapLoc], i: usize, goal: MapLoc, step_size: usize) -> usize
{
	let mut i = i;
	while util::pathfinding::a_star::get_shortest_distance(grid, &['#'], (0, 0), goal)
			.is_some()
	{
		for _ in 0..step_size
		{
			i += 1;
			grid.set(coords[i], '#');
		}
	}
	i
}
fn go_backwards(grid: &mut VecMap<char>, coords: &[MapLoc], i: usize, goal: MapLoc, step_size: usize) -> usize
{	
	let mut i = i;
	while util::pathfinding::a_star::get_shortest_distance(grid, &['#'], (0, 0), goal)
			.is_none()
	{
		for _ in 0..step_size
		{
			// Remove that spot
			grid.set(coords[i], '.');
			i -= 1;
		}
	}
	i
}

fn parse_input(input: &str) -> Vec<MapLoc>
{
	input.lines().map(
		|line|
		{
			let line = line.trim();
			let comma = line.find(',').expect("No comma in line?");
			(
				line[0..comma].parse().expect("Failed to parse"),
				line[comma+1..].parse().expect("Failed to parse")
			)
		}
	).collect()
}

#[test]
fn part_one_example_one()
{
	let coords = parse_input(EXAMPLE_INPUT_1); let size = 6; let i = 12;
	let score = part_one(&coords, size, i);
	let required = 22;

	assert_eq!(score, required, "Part One's output was {score}.  It should be {required}.");
}

#[test]
fn part_two_example_one()
{
	let coords = parse_input(EXAMPLE_INPUT_1); let size = 6;
	let result = part_two(&coords, size);
	let required = "6,1";

	assert_eq!(result, required, "Part One's output was {result}.  It should be {required}.");
}
