use std::time::{Duration, Instant};

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
	println!("Day 6");
	
	let time_before = Instant::now();
	
	let map = make_map(INPUT);
	// let map = make_map(EXAMPLE_INPUT_1);

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
		let Some(new_pos) = step(guard_location, &guard_direction)
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
	// Work out everywhere that adding a single new obstical would cause a loop
	// But it can't be the guard's starting space

	// TODO: Make a point cloud - points are obstacles & know where the next obstacle is relative to them
	// points = Vec<Point>
	// point_map = Vec<Vec<Option<usize>>> - points to the Points
	// valid_obstacles = Vec<Vec<Option<bool>>> - if we've tried here before

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
		let Some(new_pos) = step(guard_location, &guard_direction)
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
		let Some(new_pos) = step(guard_location, &guard_direction)
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
			new_map[guard_location.1][guard_location.0] = guard_direction.guard_from_dir();
			
			guard_location = new_pos;

			// We've already been here before and going this same direction!?
			// INFO: This check catches 1684 loops!! (from 6.txt)
			if new_map[guard_location.1][guard_location.0]
			== guard_direction.guard_from_dir()
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


/// The direction the guard is currently facing
#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction
{
	Up,
	Down,
	Left,
	Right
}
impl Direction
{
	pub fn turn_right(self) -> Self
	{
		match self
		{
		    Direction::Up => Direction::Right,
		    Direction::Down => Direction::Left,
		    Direction::Left => Direction::Up,
		    Direction::Right => Direction::Down,
		}
	}
	pub fn guard_from_dir(self) -> char
	{
		match self
		{
		    Direction::Up => '^',
		    Direction::Down => 'v',
		    Direction::Left => '<',
		    Direction::Right => '>',
		}	
	}
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

fn step(guard_pos: (usize, usize), direction: &Direction) -> Option<(usize, usize)>
{
	let check_pos = match direction
	{
	    Direction::Up => 
	    	(Some(guard_pos.0), guard_pos.1.checked_sub(1)),
	    Direction::Down => 
	    	(Some(guard_pos.0), guard_pos.1.checked_add(1)),
	    Direction::Left => 
	    	(guard_pos.0.checked_sub(1), Some(guard_pos.1)),
	    Direction::Right =>
	    	(guard_pos.0.checked_add(1), Some(guard_pos.1)),
	};

	if check_pos.0.is_none() || check_pos.1.is_none()
	{
		return None
	}
	// Else
	Some(( check_pos.0.unwrap(), check_pos.1.unwrap() ))
}
