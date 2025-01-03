use std::{cmp::Ordering, time::{Duration, Instant}};

use util::{pathfinding::a_star, MapFunction, MapLoc, Pair, PairFunctions, VecMap};

#[allow(unused)]
const INPUT: &str = include_str!("../../input/20.txt");
#[allow(unused)]
const EXAMPLE_INPUT_1: &str = include_str!("../../input/20_example_1.txt");

// For if we need to change it up!
type ResultType = i32;

//https://adventofcode.com/2024/day/20
pub fn go(print_results: bool) -> (Duration, Duration, Duration)
{
	if print_results {println!("Day 20");}
	
	let time_before = Instant::now();
	// ~ ~ ~ ~ ~ PRE CALCULATION ~ ~ ~ ~ ~
	
	// TODO: Do any pre-calculation here
	// let (track, score_map, path) = parse_input(EXAMPLE_INPUT_1, "Example 1", print_results);
	let (track, score_map, path) = parse_input(INPUT, "Real Input", print_results);

	// ~ ~ ~ ~ ~ END OF PRE CALCULATION ~ ~ ~ ~ ~
	let pre_calc_time = time_before.elapsed();
	if print_results { util::print_precalc(pre_calc_time) };

	// Part 1
	TimedRun!(time_before, part_one_result, part_one(&track, &score_map, &path), part_one_time);

	if print_results
	{
		util::print_result("Part 1", part_one_time, "The number of cheats that save 100 picoseconds is", &part_one_result);
	}

	// Part 2
	TimedRun!(time_before, part_two_result, part_two(&track, &score_map, &path), part_two_time);
	
	if print_results
	{
		println!();
		util::print_result("Part 2", part_two_time, "And with the longer cheats", &part_two_result);
	}

	// Return how long it took!
	(pre_calc_time, part_one_time, part_two_time)	
}

fn part_one(track: &VecMap<char>, score_map: &VecMap<Option<ResultType>>, path: &[MapLoc]) -> ResultType
{
	// In the maze, you are allowed to 'cheat'
	// Cheating is moving 2 spaces without collision

	let mut saves_100_picoseconds = 0;

	// println!("Distance: {distance}");

	for spot in path
	{
		let current_score = score_map.at_unchecked(*spot)
			.expect("There's supposed to be a score here???");
		for direction in util::DIRECTIONS
		{
			let Some(step_1) = track.step(*spot, direction)
			else { continue };

			// Make sure we're actually cheating
			if track.at_unchecked(step_1) != '#' { continue }
			
			let Some(step_2) = track.step(step_1, direction)
			else { continue };

			// Make sure we aren't still in a wall
			if track.at_unchecked(step_2) == '#' { continue }

			if let Some(score) = score_map.at_unchecked(step_2)
			{
				if score >= current_score { continue }
				// Else
				if current_score - score - 2 >= 100
				{
					saves_100_picoseconds += 1;
				}
			}
		}
	}
	saves_100_picoseconds
}

fn part_two(track: &VecMap<char>, score_map: &VecMap<Option<ResultType>>, path: &[MapLoc]) -> ResultType
{
	// In the maze, you are allowed to 'cheat'

	let mut saves_100_picoseconds = 0;

	// println!("Distance: {distance}");

	// let mut a = [ 0; 100 ];

	for i1 in 0..path.len()
	{
		let current_score = score_map.at_unchecked(path[i1])
			.expect("There's supposed to be a score here???");

		for i2 in 0..path.len()
		{
			if i1 == i2 { continue }
			let distance = path[i1].distance(path[i2]);
			if distance > 20 { continue }

			let point_2_score = score_map.at_unchecked(path[i2])
				.expect("There's also supposed to be a score here");

			if point_2_score < current_score { continue }

			let score_diff: usize = (point_2_score as usize) - (current_score as usize) - distance;
			if score_diff < 100 { continue }
			saves_100_picoseconds += 1;
			// a[ score_diff ] += 1;
		}
	}

	// for i in 50..a.len()
	// {
	// 	if a[i] == 0 { continue }
	// 	println!("{i:2?}:  {}", a[i]);
	// }

	
	assert!(saves_100_picoseconds > 927_166, "We know the answer is larger than 927166");
	assert!(saves_100_picoseconds < 1_056_116, "We know the answer is less than 1056116");
	saves_100_picoseconds
}

fn parse_input<'a>(input: &'a str, name: &str, print_results: bool) ->
(
	VecMap<char>, 
	VecMap<Option<ResultType>>,
	Vec<MapLoc>
)
{
	if print_results { println!("Parsing: {name}") };

	let track = util::read_char_map(input);

	let mut score_map = track.new_same_size::<Option<ResultType>>(None);
	
	let start = track.find('S').unwrap();
	let  goal = track.find('E').unwrap();

	let ( _ , path) = a_star::get_best_path(&track, &['#'], start, goal)
		.expect("No path to goal?");

	// Make the score map :)
	for (i, spot) in path.iter().enumerate()
	{ 
		score_map.set(*spot, 
			Some(i.try_into().unwrap())
		);
	};
	
	(track, score_map, path)
}
