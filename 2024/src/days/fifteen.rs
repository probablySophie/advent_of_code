use std::time::{Duration, Instant};

use util::{find_in, Direction, MapFunction, MapLoc, VecMap};

#[allow(unused)]
const INPUT: &str = include_str!("../../input/15.txt");
#[allow(unused)]
const EXAMPLE_INPUT_1: &str = include_str!("../../input/15_example_1.txt");
#[allow(unused)]
const EXAMPLE_INPUT_2: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
#[allow(unused)]
const EXAMPLE_INPUT_3: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

// For if we need to change it up!
type ResultType = i32;

//https://adventofcode.com/2024/day/15
pub fn go(print_results: bool) -> (Duration, Duration, Duration)
{
	if print_results {println!("Day 15");}
	
	let time_before = Instant::now();
	// ~ ~ ~ ~ ~ PRE CALCULATION ~ ~ ~ ~ ~

	let this_one = INPUT;
	// let this_one = EXAMPLE_INPUT_1;
	// let this_one = EXAMPLE_INPUT_2;
	// let this_one = EXAMPLE_INPUT_3;

	let (map, movements) = parse_input(this_one);
	let double_map = double_map(this_one);
	
	// ~ ~ ~ ~ ~ END OF PRE CALCULATION ~ ~ ~ ~ ~
	let pre_calc_time = time_before.elapsed();

	// Part 1
	TimedRun!(time_before, part_one_result, part_one(&map, &movements), part_one_time);

	if print_results
	{
		util::print_result("Part 1", part_one_time, "The GPS score", &part_one_result);
	}

	// Part 2
	TimedRun!(time_before, part_two_result, part_two(&double_map, &movements), part_two_time);
	
	if print_results
	{
		println!();
		util::print_result("Part 2", part_two_time, "Warehouse 2's GPS score", &part_two_result);
	}

	// Return how long it took!
	(pre_calc_time, part_one_time, part_two_time)	
}

fn part_one(map: &VecMap<char>, movements: &[Direction]) -> ResultType
{
	let score = do_the_thing(map, movements, false);
	
	// (my) real input: 1509074
	if map[0].len() == 50 { assert_eq!(1_509_074, score) };
	// example_1: 10092
	if map[0].len() == 10 { assert_eq!(10092, score) };
	// example_2: 2028
	if map[0].len() ==  8 { assert_eq!(2028, score) };
	score
}

fn part_two(map: &VecMap<char>, movements: &[Direction]) -> ResultType
{
	do_the_thing(map, movements, false)
}

fn do_the_thing(map: &VecMap<char>, movements: &[Direction], show_workings: bool) -> ResultType
{
	let mut map = map.clone();
	let mut position = map.find('@').expect(":(");
	map.set(position, '.'); // Remove the robot from the map
	
	for movement in movements
	{
		let Some(new_pos) = map.step(position, *movement)
		else { continue };
		
		match map.at(new_pos)
		{
			None => { panic!("????") },
			Some('#') => { /* Blocked */ },
			Some('[' | ']' | 'O') => // A crate!
			{
				if push(&mut map, new_pos, *movement)
				{
					position = new_pos;
				}
			},
			_ => // Else - move
			{
				// println!("Moving to {new_pos:?}");
				position = new_pos;
			},
		}
		if show_workings
		{
			println!("At {position:?} moving {movement:?}");
			map.set(position, '@');
			util::print_map(&map);
			map.set(position, '.');
			println!();
		}
	}

	calculate_score(&map)
}

fn calculate_score(map: &VecMap<char>) -> ResultType
{
	let mut score = 0;
	for (y, line) in map.iter().enumerate()
	{
		for (x, ch) in line.iter().enumerate()
		{
			if *ch == 'O' // Part 1
			|| *ch == '[' // Part 2
			{
				score += (100 * y) + x;
			}
		}
	}
	score.try_into().expect("Uh oh")
}

fn double_map(input: &str) -> VecMap<char>
{
	let mut double_map = Vec::new();

	for line in input.lines()
	{
		if ! line.starts_with('#') { continue }
		let mut new_line = Vec::new();
		// Else
		for ch in line.chars()
		{
			match ch
			{
				'#' => new_line.append(&mut vec!['#', '#']),
				'O' => new_line.append(&mut vec!['[', ']']),
				'.' => new_line.append(&mut vec!['.', '.']),
				'@' => new_line.append(&mut vec!['@', '.']),
				_ => {  },
			}
		}
		double_map.push(new_line);
	}
	
	double_map
}

/// Push in a given direction, starting at position
/// Returns the robot's new position
fn push(map: &mut VecMap<char>, position: MapLoc, direction: Direction) -> bool
{
	// A list of the boxes we're pushing
	let mut pushing: Vec<MapLoc> = Vec::new();
	let mut good_push = true;

	// Where we still need to check
	let mut to_check = vec![ position ];

	while let Some(new_position) = to_check.pop()
	{
		// println!("checking {new_position:?}, which is {:?}", map.at(new_position));
		match map.at(new_position)
		{
			None => { panic!("Pushed off the edge of the map??") },
			// If it's a box, then add it to our list of things to push
			Some('O') =>
			{ 
				pushing.push( new_position );
				to_check.push( map.step(new_position, direction).unwrap() );
			},
			Some('[') =>
			{
				pushing.push( new_position );
				to_check.push( map.step(new_position, direction).unwrap() );
				if ! find_in(&pushing, &map.step(new_position, Direction::Right).unwrap())
				{
					to_check.push( map.step(new_position, Direction::Right).unwrap() );
				}
			},
			Some(']') =>
			{
				pushing.push( new_position );
				to_check.push( map.step(new_position, direction).unwrap() );
				if ! find_in(&pushing, &map.step(new_position, Direction::Left).unwrap())
				{
					to_check.push( map.step(new_position, Direction::Left).unwrap() );
				}
			},
			// If we hit a wall, then there's nothing much we can do :(
			Some('#') =>
			{
				good_push = false;
				break
			},
			_ => { },
		}
	}
	if !good_push
	|| pushing.is_empty()
	{
		return false
	}
	// TODO: The actual push

	// Part 1
	if map.at(pushing[0]) == Some('O')
	{
		let new_spot = map.step(*pushing.last().unwrap(), direction).unwrap();
		// println!("emptying {:?}", pushing[0]);
		map.set(pushing[0], '.');
		map.set(new_spot, 'O');
	}
	else // Part 2
	{
		let mut map_clone = map.clone();
		for to_push in &pushing
		{
			map_clone.set(*to_push, '.');
		}
		for to_push in pushing
		{
			map_clone.set(map.step(to_push, direction).unwrap(),
			map.at(to_push).unwrap());
		}

		*map = map_clone;
	}
	true
}

fn parse_input(input: &str) -> (VecMap<char>, Vec<Direction>)
{
	let mut movements = Vec::new();

	let mut map_lines = String::new();

	for line in input.lines()
	{
		// Map
		if line.starts_with('#')
		{
			map_lines += &("\n".to_owned() + line);
		}
		// Else movements
		else if ! line.is_empty()
		{
			for movement in line.chars()
			{
				match movement
				{
					'<' => movements.push(Direction::Left),
					'>' => movements.push(Direction::Right),
					'^' => movements.push(Direction::Up),
					'v' => movements.push(Direction::Down),
					_   => {},
				}
			}
		}
	}

	(util::read_char_map(&map_lines), movements)
}
