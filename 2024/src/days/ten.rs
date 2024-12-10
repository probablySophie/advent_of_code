use std::time::{Duration, Instant};

use util::{Direction, MapLoc};

#[allow(unused)]
const INPUT: &str = include_str!("../../input/10.txt");
#[allow(unused)]
const EXAMPLE_INPUT_1: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
#[allow(unused)]
const EXAMPLE_INPUT_2: &str = "";

// For if we need to change it up!
type ResultType = i32;

//https://adventofcode.com/2024/day/10
pub fn go(print_results: bool) -> (Duration, Duration, Duration)
{
	if print_results {println!("Day 10");}
	
	let time_before = Instant::now();

	// let map = EXAMPLE_INPUT_1
	let map = INPUT
		.lines().map(
		|line| {
			line.chars().map(
				|c| 
				if c == '.' { 99 }
				else {c.to_digit(10).expect("Failed to convert to digit")}
			).collect::<Vec<u32>>()
		}
	).collect::<Vec<Vec<u32>>>();
	
	let pre_calc_time = time_before.elapsed();

	TimedRun!(time_before, part_one_result, part_one(&map), part_one_time);

	if print_results
	{
		util::print_result("Part 1", part_one_time, "Total trailhead score", &part_one_result);
	}
	
	TimedRun!(time_before, part_two_result, part_two(&map), part_two_time);
	
	if print_results
	{
		println!();
		util::print_result("Part 2", part_two_time, "Part 2 description", &part_two_result);
	}

	// Return how long it took!
	(pre_calc_time, part_one_time, part_two_time)
}

// Hiking paths:
// * start at 0 & end at 9
// * increase by a height of exactly 1 per step
// * no diagonal movements

// Trailheads
// * Are 0
// * Have 1 or more paths starting from them
// * Have a score based on how many paths start from them

/// Calculate all the trailhead scores & add them together
fn part_one(map: &[Vec<u32>]) -> ResultType
{
	let paths = find_paths(map, false);
	paths.len().try_into().unwrap()
}


fn part_two(map: &[Vec<u32>]) -> ResultType
{
	let paths = find_paths(map, true);
	paths.len().try_into().unwrap()
}

/// Find all trail paths
/// `multiple_routes` is whether or not to allow two different paths from point A to point B
fn find_paths(map: &[Vec<u32>], multiple_routes: bool) -> Vec<(MapLoc, MapLoc)>
{
	let mut paths = Vec::new();

	for (y, line) in map.iter().enumerate()
	{
		for (x, num) in line.iter().enumerate()
		{
			if *num != 0 { continue }
			paths.append(&mut find_path(map, (x, y), multiple_routes));
		}
	}

	paths
}

fn find_path(map: &[Vec<u32>], position: MapLoc, multiple_routes: bool) -> Vec<(MapLoc, MapLoc)>
{
	let mut points_to_check = vec![position];
	let mut paths = Vec::new();

	while ! points_to_check.is_empty()
	{
		let point = points_to_check.remove(0);
		let looking_for = map[point.1][point.0] + 1;

		for direction in [Direction::Up, Direction::Left, Direction::Right, Direction::Down]
		{
			if let Some(checked_position) = check_direction(map, point, direction, looking_for)
			{
				if map[checked_position.1][checked_position.0] == 9
				{
					if multiple_routes
					|| ! util::find_in(&paths, &(position, checked_position))
					{
						paths.push((position, checked_position));
					}
				}
				else
				{
					points_to_check.push(checked_position);	
				}
			}
		}
	}
	paths
}

fn check_direction(map: &[Vec<u32>], position: MapLoc, direction: Direction, looking_for: u32) -> Option<MapLoc>
{
	let mut rel_pos = position;
	match direction
	{
		Direction::Up =>
		{
			let v = position.1.checked_sub(1)?;
			rel_pos.1 = v;
		},
		Direction::Down =>
		{
			let v = position.1.checked_add(1)?;
			rel_pos.1 = v;
		},
		Direction::Left =>
		{
			let v = position.0.checked_sub(1)?;
			rel_pos.0 = v;
		},
		Direction::Right =>
		{
			let v = position.0.checked_add(1)?;
			rel_pos.0 = v;
		},
	}
	if check_at(map, rel_pos, looking_for)
	{
		return Some(rel_pos)
	}
	None
}

fn check_at(map: &[Vec<u32>], position: MapLoc, looking_for: u32) -> bool
{
	// Make sure we're in bounds
	if position.0 >= map[0].len()
	|| position.1 >= map[1].len()
	{ return false }
	
	if map[position.1][position.0] == looking_for
	{
		return true
	}
	false
}
