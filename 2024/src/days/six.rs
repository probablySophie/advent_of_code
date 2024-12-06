use colored::Colorize;

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

// https://adventofcode.com/2024/day/6

pub fn go()
{
	println!("Day 6");
	let map = make_map(INPUT);
	// let map = make_map(EXAMPLE_INPUT_1);

	println!("Map Size: {}x{}", map[0].len(), map.len());

	println!("\t{}\n\tTotal unique positions: {}",
		"Part 1".bold(),
		part_one(&mut map.clone())
	);
	
	println!("\t{}\n\tNumber of loop-causing positions: {}",
		"Part 2".bold(),
		part_two(&mut map.clone())
	);
}

type LevelMap = Vec<Vec<char>>;

fn part_one(map: &mut LevelMap) -> i32
{
	let Some(mut guard_location) = get_guard_loc(map)
	else { return -1 };
	let mut guard_direction = Direction::Up;

	'mainloop: loop
	{
		map[guard_location.1][guard_location.0] = 'X';
		
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
		// Update the guard's location
		guard_location = new_pos;
	}
	// print_map(map);
	count_points(map)
}

fn part_two(map: &mut LevelMap) -> i32
{
	// Work out everywhere that adding a single new obstical would cause a loop
	// But not the guard's starting space

	// WARN: This solution is painfully inefficient

	let mut loop_spots = Vec::new();
	
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
		if new_pos != starting_location && is_loop(map, new_pos) {
			let mut found = false;
			for ls in &loop_spots { if ls == &new_pos { found = true } };
			if !found
			{
				loop_spots.push( new_pos );
			}
		}
		// Update the guard's location
		guard_location = new_pos;
	}
	
	loop_spots.len().try_into().unwrap()
}

fn is_loop(map: &mut LevelMap, obstical_location: (usize, usize)) -> bool
{
	let mut new_map = map.clone();
	new_map[obstical_location.1][obstical_location.0] = 'O';
	
	let Some(starting_location) = get_guard_loc(&new_map)
	else { return false };

	let mut guard_direction = Direction::Up;
	let mut guard_location = starting_location;

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
		}
		else
		{
			guard_location = new_pos;
			
			new_map[guard_location.1][guard_location.0] = 
				match (new_map[guard_location.1][guard_location.0], guard_direction)
				{
					('.'|'|', Direction::Up   | Direction::Down  ) => '|',
					('.'|'-', Direction::Left | Direction::Right ) => '-',
					('-', Direction::Up   | Direction::Down  ) | 
					('|', Direction::Left | Direction::Right ) | 
					('+', _) => '+',
					('^', _) => '^',
					_ => '?'
				};
		}
		
		// Are we back at the starting point facing the correct direction?
		if guard_location.0 == starting_location.0
		&& guard_location.1 == starting_location.1
		&& guard_direction == Direction::Up
		{
			// println!("\n{obstical_location:?}");
			// print_map(&new_map);
			return true
		}
	}
	// If we've taken more that 1000 steps, then it's PROBABLY true
	// println!("\n{obstical_location:?}");
	// print_map(&new_map);
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
	pub fn turn_right(&self) -> Self
	{
		match self
		{
		    Direction::Up => Direction::Right,
		    Direction::Down => Direction::Left,
		    Direction::Left => Direction::Up,
		    Direction::Right => Direction::Down,
		}
	}
	pub fn guard_from_dir(&self) -> char
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

fn should_turn(map: &LevelMap, direction: &Direction) -> bool
{
	let Some(guard_pos) = get_guard_loc(map)
	else { return false };

	let Some(check_pos) = step(guard_pos, direction)
	else { return false };
	
	// Are we checking outside of the map?
	if check_pos.0 > map[0].len() - 1
	|| check_pos.1 > map.len() - 1
	{
		return false // yes
	}

	// Is the space occupied
	if map[ check_pos.1 ][ check_pos.0 ] == '.'
	|| map[ check_pos.1 ][ check_pos.0 ] == 'X'
	{
		return false // no
	}

	// The space is occipied
	true
}
