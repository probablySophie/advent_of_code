use std::{borrow::BorrowMut, cmp::Ordering, time::{Duration, Instant}};

use util::{Direction, MapFunction, MapLoc, PairFunctions, Point, VecMap};

#[allow(unused)]
const INPUT: &str = include_str!("../../input/16.txt");
#[allow(unused)]
const EXAMPLE_INPUT_1: &str = include_str!("../../input/16_example_1.txt");
#[allow(unused)]
const EXAMPLE_INPUT_2: &str = include_str!("../../input/16_example_2.txt");

// For if we need to change it up!
type ResultType = i32;
type ConnectedPoint = util::ConnectedPoint<ResultType>;

//https://adventofcode.com/2024/day/16
pub fn go(print_results: bool) -> (Duration, Duration, Duration)
{
	if print_results {println!("Day 16");}
	
	let time_before = Instant::now();
	// ~ ~ ~ ~ ~ PRE CALCULATION ~ ~ ~ ~ ~

	// let use_me = INPUT;
	// let use_me = EXAMPLE_INPUT_1;
	let use_me = EXAMPLE_INPUT_2;

	let maze = util::read_char_map(use_me);
	let (corners, corner_map) = find_corners(&maze);


	// ~ ~ ~ ~ ~ END OF PRE CALCULATION ~ ~ ~ ~ ~
	let pre_calc_time = time_before.elapsed();
	if print_results { util::print_precalc(pre_calc_time) };

	// Part 1
	TimedRun!(time_before, part_one_result, part_one(&maze, &corner_map), part_one_time);

	if print_results
	{
		util::print_result("Part 1", part_one_time, "The lowest possible score is", &part_one_result);
	}

	// Part 2
	TimedRun!(time_before, part_two_result, part_two(&maze, &corners, &corner_map), part_two_time);
	
	if print_results
	{
		println!();
		util::print_result("Part 2", part_two_time, "Num tiles on ALL best paths", &part_two_result);
	}

	// Return how long it took!
	(pre_calc_time, part_one_time, part_two_time)	
}


/// Work out the minimum possible movement score for a given maze!
fn part_one(maze: &VecMap<char>, corner_map: &CornerMap) -> ResultType
{
	let goal = maze.find('E').expect("No goal on map?");
	
	corner_map.at(goal)
		.expect("No goal position on corner_map?")
		.expect("No point at the goal position on the corner map?")
		.1
}

fn part_two(maze: &VecMap<char>, corners: &[Point], corner_map: &CornerMap) -> ResultType
{

	0
}

type CornerMap = VecMap<Option<(usize, ResultType)>>;

fn find_corners(maze: &VecMap<char>) -> (Vec<Point>, CornerMap)
{
	let start = maze.find('S').expect("No start?");
	let goal  = maze.find('E').expect("No end?");
	
	let mut corner_map: CornerMap = maze.new_same_size(None);

	let mut corners = vec![];
	
	corners.push( Point::new(start) );
	corners.push( Point::new(goal ) );

	// Find where all of the corners are
	for (y, line) in maze.iter().enumerate()
	{
		for (x, ch) in line.iter().enumerate()
		{
			if *ch == '#'
			|| (x, y) == start
			|| (x, y) == goal
			{ continue } // Skip walls
			match (maze[y-1][x] != '#', maze[y+1][x] != '#', maze[y][x+1] != '#', maze[y][x-1] != '#')
			{
				(_    , true , true , true ) |
				(true , _    , true , true ) |
				(true , true , _    , true ) |
				(true , true , true , _    ) |
				(true , false, true , false) |
				(false, true , false, true ) |
				(false, true , true , false) |
				(true , false, false, true )
				=> /* It's a corner!! */
				{
					corners.push( Point::new((x, y)) );
				},
				_ => { continue },
			}
		}
	}	
	attach_cardinal_points(&mut corners, maze);
	
	for (i, corner) in corners.iter().enumerate()
	{
		corner_map.set(corner.position, Some((i, -1)) );
	}
	corner_map.set(start, Some((0, 0)));
	// From corners_0, travel from the start to the end & update the points if need be

	let mut open_set = vec![ (0, Direction::Right) ];

	while let Some((i, direction)) = open_set.pop()
	{
		let pos1 = corners[i].position;
		if pos1 == goal { continue }
		
		let (_, score) = corner_map.at_unchecked(pos1).unwrap();

		for new_direction in util::DIRECTIONS
		{
			let Some(pos2) = corners[i].get_from_direction(new_direction)
			else { continue };

			let new_score = score + calculate_score(pos1, pos2, direction, new_direction);

			let Some((i2, score2)) = corner_map.at_unchecked(pos2)
			else { panic!("aaa") };

			if score2 == -1
			|| score2 > new_score
			{
				corner_map.set(pos2, Some((i2, new_score)));
				open_set.push( ( i2, new_direction ) );
			}
		}
	}
	
	(corners, corner_map)
}

fn attach_cardinal_points(points: &mut [Point], maze: &VecMap<char>)
{	
	// Fill in the cardinal directions
	for i1 in 0..points.len()
	{
		let pos1 = points[i1].position;
		for i2 in 0..points.len()
		{
			// If they're the same point
			if i1 == i2 { continue } // Continue
			let pos2 = points[i2].position;
			// If they're not even on the same line
			if pos1.0 != pos2.0
			&& pos1.1 != pos2.1
			{
				continue // Continue
			}
			// How far apart are they
			let distance = pos1.0.abs_diff(pos2.0) + pos1.1.abs_diff(pos2.1);

			match ( pos1.0.cmp(&pos2.0), pos1.1.cmp(&pos2.1) )
			{
				// They're to our right
			    (Ordering::Less, Ordering::Equal) =>
			    if points[i1].right.is_some_and( |pos| pos.distance(pos2) > distance )
			    || points[i1].right.is_none()
			    {
			    	points[i1].right = Some(pos2);
			    },
			    // They're below us
			    (Ordering::Equal, Ordering::Less) =>
			    if points[i1].down.is_some_and( |pos| pos.distance(pos2) > distance )
			    || points[i1].down.is_none()
			    {
			    	points[i1].down = Some(pos2);
			    },
			    // They're above us
			    (Ordering::Equal, Ordering::Greater) =>
			    if points[i1].up.is_some_and( |pos| pos.distance(pos2) > distance )
			    || points[i1].up.is_none()
			    {
			    	points[i1].up = Some(pos2);
			    },
			    // They're to our left
			    (Ordering::Greater, Ordering::Equal) =>
			    if points[i1].left.is_some_and( |pos| pos.distance(pos2) > distance )
			    || points[i1].left.is_none()
			    {
			    	points[i1].left = Some(pos2);
			    },
			    _ => {},
			}
		}
	}

	for point in points
	{
		let pos1 = point.position;
		'directionLoop: for direction in util::DIRECTIONS
		{
			let Some(pos2) = point.get_from_direction(direction)
			else { continue };

			let mut position = pos1;
			while let Some(temp_pos) = maze.step(position, direction)
			{
				if maze.at(temp_pos) == Some('#')
				{
					break;
				}
				position = temp_pos;
				
				if position == pos2 // We made it to where we want to be
				{
					continue 'directionLoop;
				}
			}
			// if we're here.  That's bad
			point.clear_direction(direction);
		}
	}
}

#[allow(unused)]
fn fancy_print(maze: &VecMap<char>) -> ResultType
{
	let mut score = 0;
	for (y, line) in maze.iter().enumerate()
	{
		for (x, spot) in line.iter().enumerate()
		{
			if *spot == '#'
			{
				print!(" ");
			}
			else if *spot == '.'
			{
				match (maze[y-1][x] != '#', maze[y+1][x] != '#', maze[y][x+1] != '#', maze[y][x-1] != '#')
				{
					(true , true , true , true ) => print!("╋"),
					(true , true , false, false) => print!("┃"),
					(false, false, true , true ) => print!("━"),
					(true , false, true , true ) => print!("┻"),
					(false, true , true , true ) => print!("┳"),
					(true , true , false, true ) => print!("┫"),
					(true , true , true , false) => print!("┣"),
					(false, true , true , false) => print!("┏"),
					(false, true , false, true ) => print!("┓"),
					(true , false, true , false) => print!("┗"),
					(true , false, false, true ) => print!("┛"),
					_ => print!(" "),
				}
			}
			else
			{
				print!("{spot}");
			}
		}
		println!(" {y}");
	}
	score
	
}

fn calculate_score(pos_1: MapLoc, pos_2: MapLoc, old_direction: Direction, new_direction: Direction) -> ResultType
{
	({
		if old_direction == new_direction { 0 }
		else if old_direction == new_direction.opposite() { 2000 }
		else { 1000 }
	} + ResultType::try_from(pos_1.0.abs_diff(pos_2.0) + pos_1.1.abs_diff(pos_2.1)).unwrap())
}


// #[test]
// fn part_one_test()
// {
// 	let inputs = [(INPUT, 94444), (EXAMPLE_INPUT_1, 7036), (EXAMPLE_INPUT_2, 11048)];

// 	for input in inputs
// 	{
// 		let maze = util::read_char_map(input.0);
// 		let corners = find_corners(&maze);
// 		let corner_map = make_corner_map(, , )
		
// 		let score = part_one(&maze, &corner_map);
// 		assert_eq!(score, input.1, "Part 1 output was {score}, it should be {}", input.1);
// 	}
// }
// // #[test]
// // fn part_two_test()
// // {
// // 	let inputs = [/*(INPUT, 12),*/ (EXAMPLE_INPUT_1, 45), (EXAMPLE_INPUT_2, 64)];

// // 	for (i, input) in inputs.iter().enumerate()
// // 	{
// // 		println!("Input #{}", i+1);
// // 		let maze = util::read_char_map(input.0);
// // 		let (corners, corner_map) = find_corners(&maze);
		
// // 		let score = part_two(&maze, &corners, &corner_map);
// // 		assert_eq!(score, input.1, "Part 2 output was {score}, it should be {}", input.1);
// // 	}
// // }
