use std::time::{Duration, Instant};

use util::{Direction, MapFunction, MapLoc};

#[allow(unused)]
const INPUT: &str = include_str!("../../input/6.txt");
#[allow(unused)]
const EXAMPLE_INPUT_1: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
#[allow(unused)]
const EXAMPLE_INPUT_2: &str = "";

//https://adventofcode.com/2024/day/6
pub fn go(print_results: bool) -> (Duration, Duration, Duration)
{
	if print_results {println!("Day 6");}
	
	let time_before = Instant::now();
	
	// let map = make_map(INPUT);
	let map = make_map(EXAMPLE_INPUT_1);

	if print_results
	{
		println!("Map Size: {}x{}", map[0].len(), map.len());

	}
	
	let pre_calc_time = time_before.elapsed();

	TimedRun!(time_before, part_one_result, part_one(&mut map.clone()), part_one_time);

	if print_results
	{
		util::print_result("Part 1", part_one_time, "Total unique positions", &part_one_result);
	}
	
	TimedRun!(time_before, part_two_result, part_two(&mut map.clone()), part_two_time);
	
	if print_results
	{
		println!();
		util::print_result("Part 2", part_two_time, "Number of unique loop-causing positions", &part_two_result);
	}

	// Return how long it took!
	(pre_calc_time, part_one_time, part_two_time)	
}

type LevelMap = Vec<Vec<char>>;

fn part_one(map: &mut LevelMap) -> i32
{
	let Some(mut guard_location) = get_guard_loc(map)
	else { return -1 };
	let mut guard_direction = Direction::Up;

	'mainloop: loop
	{
		// Record where we've been!
		map[guard_location.1][guard_location.0] = 'X';
		
		// Take a step
		let Some(new_pos) = map.step(guard_location, guard_direction)
		// Else we've left the map left/top
		else { break 'mainloop; };
		// And check if we've exited on the right/bottom
		if new_pos.0 > (map[0].len() - 1) || new_pos.1 > (map.len() - 1)
		{ break 'mainloop; }

		// Did we bump into something
		if map[new_pos.1][new_pos.0] == '#'
		{
			guard_direction = guard_direction.turn_right();
		}
		else
		{
			// Update the guard's location
			guard_location = new_pos;
		}
	}
	// print_map(map);
	count_points(map)
}

fn part_two(map: &mut LevelMap) -> i32
{
	let (points, point_map) = Point::build_point_map(map);
	// TODO: use this point map
	// Work out everywhere that adding a single new obstical would cause a loop
	// But it can't be the guard's starting space

	// For the loop checking, instead of iterating through each position on the map
	// Go from point to point - should be considerably faster!

	let mut checked_obstacles: Vec<Vec<Option<bool>>> = vec![ vec![ None; map[0].len() ]; map.len() ];
	let mut obstacle_counter = 0;
	
	let Some(starting_location) = get_guard_loc(map)
	else { return -1 };
	
	let mut guard_direction = Direction::Up;
	let mut guard_location = starting_location;
	'mainloop: loop
	{
		// Take a step
		let Some(new_pos) = map.step(guard_location, guard_direction)
		// Else we've left the map left/top
		else { break 'mainloop; };
		// And check if we've left right/bottom
		if new_pos.0 > (map[0].len() - 1) || new_pos.1 > (map.len() - 1)
		{ break 'mainloop; }

		// Did we bump into something
		if map[new_pos.1][new_pos.0] == '#'
		{
			guard_direction = guard_direction.turn_right();
			continue;
		}
		// Now check if adding an obstical infront of the guard would cause a loop
		if checked_obstacles[new_pos.1][new_pos.0].is_none()
		&& new_pos != starting_location
		{
			let result = is_loop(map, new_pos);
			checked_obstacles[new_pos.1][new_pos.0] = Some( result );
			if result { obstacle_counter += 1; }
		}
		// Update the guard's location
		guard_location = new_pos;
	}

	obstacle_counter
}

fn is_loop(map: &mut LevelMap, obstical_location: (usize, usize)) -> bool
{
	let mut new_map = map.clone();
	new_map[obstical_location.1][obstical_location.0] = 'O';
	
	let Some(mut guard_location) = get_guard_loc(&new_map)
	else { return false };

	let mut guard_direction = Direction::Up;

	let mut num_turns = 0;
	let mut did_180 = false;

	for _ in 0..=10000
	{
		// Else take a step
		let Some(new_pos) = map.step(guard_location, guard_direction)
		else { return false };
		if new_pos.0 >= new_map[0].len()
		|| new_pos.1 >= new_map.len()
		{
			return false
		}
		if new_map[new_pos.1][new_pos.0] == '#'
		|| new_map[new_pos.1][new_pos.0] == 'O'
		{
			guard_direction = guard_direction.turn_right();
			num_turns += 1;
		}
		else
		{
			// did we just turn?
			if num_turns > 0
			{
				// INFO: This check just here catches 152 endless loops!! (from 6.txt)
				match ( num_turns, did_180 )
				{
					// We did a 180
					(2, false) => { did_180 = true },
					// We've done two 180s in a row.  That's a loop babeey
					(2, true)  => { return true },
					// Else we didn't just do a 180
					_ => { did_180 = false },
				}
				// And reset
				num_turns = 0;
			}
			// Mark where we've been & which direction we were going
			new_map[guard_location.1][guard_location.0] = guard_direction.to_char();
			
			guard_location = new_pos;

			// We've already been here before and going this same direction!?
			// INFO: This check catches 1684 loops!! (from 6.txt)
			if new_map[guard_location.1][guard_location.0]
			== guard_direction.to_char()
			{
				return true
			}
		}		
	}
	// If we've taken more that 1000 steps, then it's PROBABLY true
	// println!("\n{obstical_location:?}");
	// print_map(&new_map);
	// INFO: We now have no timeouts!!!!
	true
}

/// Turn an input `&str` into a `Vec<Vec<char>>` :)
fn make_map(input: &str) -> LevelMap
{
	input.lines().map(|line| {
		line.chars().collect::<Vec<char>>()
	}).collect()
}

#[allow(unused)]
fn print_map(map: &LevelMap)
{
	for line in map
	{
		for c in line
		{
			print!("{c}");
		}
		println!();
	}
}

/// Get the current location of the guard on the map
/// Returns None if the guard isn't on the map
fn get_guard_loc(map: &LevelMap) -> Option<(usize, usize)>
{
	for (y, line) in map.iter().enumerate()
	{
		for (x, c) in line.iter().enumerate()
		{
			match *c
			{
				'^'|'v'|'<'|'>' => { return Some((x, y)) }
				_ => {}
			}
		}
	}
	None
}

fn count_points(map: &LevelMap) -> i32
{
	let mut count = 0;
	for line in map
	{
		for c in line
		{
			if *c == 'X'
			{
				count += 1;
			}
		}
	}
	count
}

#[derive(Default)]
struct Point
{
	pub location: MapLoc,
	pub from_below: Option<MapLoc>,
	pub from_above: Option<MapLoc>,
	pub from_left: Option<MapLoc>,
	pub from_right: Option<MapLoc>,
}
impl Point
{
	#[allow(clippy::needless_range_loop)]
	pub fn build_point_map(map: &LevelMap) -> (Vec<Point>, Vec<Vec<Option<usize>>>)
	{
		let mut points = Vec::new();
		let mut point_map: Vec<Vec<Option<usize>>> = vec![ vec![ None; map[0].len() ]; map.len() ];
		// Build empty points
		for y in 0..map.len()
		{
			for x in 0..map[0].len()
			{
				if map.at((x, y)) == Some('#')
				{
					points.push ( Point {location: (x, y), ..Default::default() });
					point_map[y][x] = Some(points.len()-1);
				}
			}
		}
		// Fill in the links
		for i in 0..points.len()
		{
			for i_2 in 0..points.len()
			{
				if i == i_2 { continue }; // if they're the same guy, skip
				let l1 = points[i].location;
				let l2 = points[i_2].location;

				if l1.0.abs_diff(l2.0) != 1 // if they can't be relative friends, skip
				&& l1.1.abs_diff(l2.1) != 1
				{ continue };

				// let mut map_copy = map.clone();
				// map_copy.set(l1, '@');
				// map_copy.set(l2, '&');
				// util::print_map(&map_copy);

				// println!("{:?} {:?} {:?} {:?}", l1.0.abs_diff(l2.0), l1.1.abs_diff(l2.1), l1.0.cmp(&l2.0), l1.1.cmp(&l2.1));

				// Work out what direction they are from each-other
				match (l1.0.abs_diff(l2.0), l1.1.abs_diff(l2.1), l1.0.cmp(&l2.0), l1.1.cmp(&l2.1))
				{
					// From bottom - good
					(_, 1, std::cmp::Ordering::Less, std::cmp::Ordering::Less) =>
					{
						set_if_less_or_none(&mut points[i], l2, Direction::Down);
					},
					// From top - good
					(_, 1, std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) =>
					{
						set_if_less_or_none(&mut points[i], l2, Direction::Up);
					},
					// From left - good
					(1, _, std::cmp::Ordering::Greater, std::cmp::Ordering::Less) =>
					{
						set_if_less_or_none(&mut points[i], l2, Direction::Left);
					},
					// From right - good
					(1, _, std::cmp::Ordering::Less, std::cmp::Ordering::Greater) =>
					{
				 		set_if_less_or_none(&mut points[i], l2, Direction::Right);
					},
					_ => {} // Bad
				 }
			}
		}
		(points, point_map)
	}
}

fn set_if_less_or_none(point: &mut Point, location: MapLoc, direction: Direction)
{
	match direction
	{
		Direction::Up =>
		{
			if point.from_above.is_none()
			{
				point.from_above = Some(location);
				return
			}
			// Else
			if point.location.1.abs_diff(location.1) 
				> point.location.1.abs_diff(point.from_above.unwrap().1)
			{
				point.from_above = Some(location);
			}
		},
		Direction::Down =>
		{
			if point.from_below.is_none()
			{
				point.from_below = Some(location);
				return
			}
			// Else
			if point.location.1.abs_diff(location.1) 
				> point.location.1.abs_diff(point.from_below.unwrap().1)
			{
				point.from_below = Some(location);
			}
		},
		Direction::Left =>
		{
			if point.from_left.is_none()
			{
				point.from_left = Some(location);
				return
			}
			// Else
			if point.location.0.abs_diff(location.0) 
				> point.location.0.abs_diff(point.from_left.unwrap().0)
			{
				point.from_left = Some(location);
			}
		},
		Direction::Right =>
		{
			if point.from_right.is_none()
			{
				point.from_right = Some(location);
				return
			}
			// Else
			if point.location.0.abs_diff(location.0) 
				> point.location.0.abs_diff(point.from_right.unwrap().0)
			{
				point.from_right = Some(location);
			}
		},
	}
}
