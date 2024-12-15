use std::time::{Duration, Instant};

use util::MapLoc;

#[allow(unused)]
const INPUT: &str = include_str!("../../input/14.txt");
#[allow(unused)]
const EXAMPLE_INPUT_1: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
#[allow(unused)]
const EXAMPLE_INPUT_2: &str = "";

// For if we need to change it up!
type ResultType = i32;
type Robot = (MapLoc, (ResultType, ResultType));

//https://adventofcode.com/2024/day/14
pub fn go(print_results: bool) -> (Duration, Duration, Duration)
{
	if print_results {println!("Day 14");}
	
	let time_before = Instant::now();
	// ~ ~ ~ ~ ~ PRE CALCULATION ~ ~ ~ ~ ~

	// let robots = parse_input(EXAMPLE_INPUT_1);
	// let world_size = (11, 7);

	let robots = parse_input(INPUT);
	let world_size = (101, 103);
	
	// ~ ~ ~ ~ ~ END OF PRE CALCULATION ~ ~ ~ ~ ~
	let pre_calc_time = time_before.elapsed();

	// Part 1
	TimedRun!(time_before, part_one_result, part_one(&robots, world_size), part_one_time);

	if print_results
	{
		util::print_result("Part 1", part_one_time, "Robots-quadrant score", &part_one_result);
	}

	// Part 2
	TimedRun!(time_before, part_two_result, part_two(&robots, world_size), part_two_time);
	
	if print_results
	{
		println!();
		util::print_result("Part 2", part_two_time, "Part 2 description", &part_two_result);
	}

	// Return how long it took!
	(pre_calc_time, part_one_time, part_two_time)	
}

fn part_one(robots: &[Robot], worldsize: (usize, usize)) -> ResultType
{
	let show_map = false;
	let mut world = vec![ vec![ 0; worldsize.0 ]; worldsize.1 ];

	if show_map
	{
		for robot in robots
		{
			let position = robot.0;
			world[position.1][position.0] += 1;
		}
		println!("Robots Before:");
		print_world(&world);
	}

	let mut top_left = 0; // Top left
	let mut top_right = 0; // Top right
	let mut bottom_left = 0; // Bottom left
	let mut bottom_right = 0; // Bottom right

	for robot in robots
	{
		let new_position = move_robot(robot, worldsize, 100);
		if show_map
		{
			world[robot.0.1][robot.0.0] -= 1;
			world[new_position.1][new_position.0] += 1;
		}

		match (
			new_position.0.cmp(&(worldsize.0/2)),
			new_position.1.cmp(&(worldsize.1/2))
		)
		{
			(std::cmp::Ordering::Less, std::cmp::Ordering::Less) 
			=> top_left += 1,
			(std::cmp::Ordering::Less, std::cmp::Ordering::Greater) 
			=> top_right += 1,
			(std::cmp::Ordering::Greater, std::cmp::Ordering::Less)
			=> bottom_left += 1,
			(std::cmp::Ordering::Greater, std::cmp::Ordering::Greater)
			=> bottom_right += 1,
			_ => {} // Ignore any Equal-s
		}
	}

	if show_map
	{
		println!("Robots After:");
		print_world(&world);
	}

	top_left * top_right * bottom_left * bottom_right
}

fn part_two(robots: &[Robot], worldsize: (usize, usize)) -> ResultType
{
	let mut i = 0;
	let mut robots = robots.to_vec();

	'mainLoop: loop
	{
		let mut world = vec![ vec![ 0; worldsize.0 ]; worldsize.1 ];
				
		i += 1;
		for robot in &mut robots
		{
			robot.0 = move_robot(robot, worldsize, 1);
			world[robot.0.1][robot.0.0] += 1;
		}
		for line in world
		{
			let mut in_a_row = 0;
			for i in line
			{
				if i == 0
				&& in_a_row > 10
				{
					break 'mainLoop
				}
				else if i == 0
				{
					in_a_row = 0;
					continue;
				}
				in_a_row += 1;
			}
		}		
	}
	i
}

fn move_robot(robot: &Robot, worldsize: (usize, usize), num_times: ResultType) -> MapLoc
{
	let change_x = robot.1.0 * num_times;
	let change_y = robot.1.1 * num_times;

	let remainder_x = change_x % ResultType::try_from(worldsize.0)
		.expect("Failed to ResultTypeify");
	let remainder_y = change_y % ResultType::try_from(worldsize.1)
		.expect("Failed to ResultTypeify");

	let new_x = remainder_x + ResultType::try_from(robot.0.0)
		.expect("Failed to ResultSizeify");
	let new_y = remainder_y + ResultType::try_from(robot.0.1)
		.expect("Failed to ResultSizeify");

	let mut x: usize;
	let mut y: usize;

	if new_x < 0 
	{ x = worldsize.0.checked_sub(new_x.unsigned_abs().try_into().unwrap()).expect(":(") }
	else {
		let new_x: usize = new_x.try_into().unwrap();
		if new_x < worldsize.0 { x = new_x }
		else { x = new_x - worldsize.0 }
	}
	if new_y < 0 
	{ y = worldsize.1.checked_sub(new_y.unsigned_abs().try_into().unwrap()).expect(":(") }
	else {
		let new_y: usize = new_y.try_into().unwrap();
		if new_y < worldsize.1 { y = new_y }
		else { y = new_y - worldsize.1 }
	}
	
	(x, y)
}

/// Parse a given input `&str` into a `Vec<(MapLoc, (ResultType, ResultType))>`
fn parse_input(input: &str) -> Vec<Robot>
{
	let mut lines = Vec::new();

	for line in input.lines()
	{
		let split: Vec<&str> = line.split_whitespace().collect();
		if split.len() != 2 { continue }

		let Some(position) = parse_split::<usize>(split[0])
		else { continue };

		let Some(velocity) = parse_split::<ResultType>(split[1])
		else { continue };

		lines.push((position, velocity));
	}
	lines
}

/// Parse a given split for `parse_input()`
fn parse_split<T: std::str::FromStr>(split: &str) -> Option<(T, T)>
{
	let equals = split.find('=')?;
	let comma  = split.find(',')?;

	let Ok(a) = split[equals+1..comma].parse()
	else { return None };
	
	let Ok(b) = split[comma+1..].parse()
	else { return None };

	Some((a, b))
}

fn print_world(world: &[Vec<ResultType>])
{
	println!("╭{}╮", "─".repeat(world[0].len()));
	for line in world
	{
		print!("│");
		for i in line
		{
			if *i != 0 {print!("{i}")}
			else { print!(" ") };
		}
		print!("│");
		println!();
	}
	println!("╰{}╯", "─".repeat(world[0].len()));
}
