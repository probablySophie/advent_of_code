use std::time::{Duration, Instant};

use util::{Direction, MapLoc, MapFunction, Point, VecMap};

#[allow(unused)]
const INPUT: &str = include_str!("../../input/16.txt");
#[allow(unused)]
const EXAMPLE_INPUT_1: &str = include_str!("../../input/16_example_1.txt");
#[allow(unused)]
const EXAMPLE_INPUT_2: &str = include_str!("../../input/16_example_2.txt");

// For if we need to change it up!
type ResultType = i32;

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
		.1
}

/// Work out how many tiles (including the start and goal) are on a best possible path (there can be mutiple best possible paths)
fn part_two(maze: &VecMap<char>, corners: &[Point], corner_map: &CornerMap) -> ResultType
{
	let start = maze.find('S').unwrap();
	let  goal = maze.find('E').unwrap();
	let (goal_i, max_score) = corner_map_unwrap(corner_map, goal).unwrap();

	let mut to_check = vec![(start, 0)];
	let mut path_corners: Vec<(usize, usize)> = Vec::new();
	let mut max_steps = 0;

	while let Some((position, step)) = to_check.pop()
	{
		let Some((i, score)) = corner_map_unwrap(corner_map, position)
		else { continue };
		// println!("From {position:2?}");
		path_corners.push((i, step));
		for direction in util::DIRECTIONS
		{
			let Some(pos) = corners[i].get_from_direction(direction)
			else { continue };

			let Some((i2, score2)) = corner_map_unwrap(corner_map, pos)
			else { continue };
			// println!("\tAt {pos:2?} there is {score2}");
			if score2 > score && score2 < max_score
			{
				to_check.push((pos, step+1));
			}
			else if pos == goal
			{
				path_corners.push((i2, step));
				max_steps = step;
			}
		}
	}

	// print_score_map(corner_map);
	// panic!();

	let mut maze_2 = maze.clone();
	for (corner_i, _) in &path_corners
	{		
		maze_2.set(corners[*corner_i].position, 'X');
	}
	fancy_print(&maze_2);
	
	// // Prune bad direction
	prune(corners, corner_map, &mut path_corners, max_steps, goal);
	
	for (corner_i, _) in &path_corners
	{		
		maze_2.set(corners[*corner_i].position, 'O');
	}
	fancy_print(&maze_2);

	let mut score = 0;
	let mut to_check = vec![ goal_i ];
	while let Some(i) = to_check.pop()
	{
		score += 1;
		for (k,(j, _)) in path_corners.iter().enumerate()
		{ if *j == i { path_corners.remove(k); break } };
		
		let pos_1 = corners[i].position;

		let mut overcount = -1;
		for direction in util::DIRECTIONS
		{
			let Some(pos_2) = corners[i].get_from_direction(direction)
			else { continue };

			let Some((i2, _)) = corner_map_unwrap(corner_map, pos_2)
			else { continue };

			for (k, (j, _)) in path_corners.iter().enumerate()
			{
				if *j == i2
				{
					overcount += 1;
					path_corners.remove(k);
					let diff = pos_1.0.abs_diff(pos_2.0) + pos_1.1.abs_diff(pos_2.1) - 1;
					score += diff;
					println!("{pos_1:2?} to {pos_2:2?} is {diff:5?} total: {score:5?}");
					to_check.push( i2 );
					break;
				}
			}
		}
		if overcount > 0
		{
			score -= std::convert::TryInto::<usize>::try_into(overcount).unwrap();
		}
	}
	
	(score).try_into().unwrap()
}



fn prune(corners: &[Point], corner_map: &CornerMap, path_corners: &mut Vec<(usize, usize)>, max_steps: usize, goal: MapLoc)
{
	let mut current_steps = max_steps;

	// This breaks if: 
	// * we have a run where noone is removed
	// * current_steps hits -1
	loop
	{
		let mut all_good = true;
		let required_steps = max_steps - current_steps;

		for pc_i in (0..path_corners.len()).rev()
		{
			let (corner_i, step_num) = path_corners[pc_i];
			
			if step_num < current_steps { continue }
			let position = corners[corner_i].position;

			let mut check_at = vec![( position, 0 )];
			let mut found_goal = false;
			while let Some((check_position, step_num)) = check_at.pop()
			{
				let Some((i2, score1)) = corner_map_unwrap(corner_map, check_position)
				else { continue };
				
				if check_position == goal
				{
					found_goal = true; 
					break
				}
				for direction in util::DIRECTIONS
				{
					let Some(pos2) = corners[i2].get_from_direction(direction)
					else { continue };
					let Some((_, score2)) = corner_map_unwrap(corner_map, pos2)
					else { continue };

					if score2 < score1 { continue }

					if step_num <= required_steps
					{
						check_at.push((pos2, step_num + 1));
					}
				}
			}
			if !found_goal
			{
				all_good = false;
				path_corners.remove(pc_i);
			}
		}
		match current_steps.checked_sub(1)
		{
			None => { break },
			Some(v) => current_steps = v,
		}
		if all_good { break }
	}
	for i in (0..path_corners.len()).rev()
	{
		for j in 0..path_corners.len()
		{
			if i == j { continue }
			if path_corners[i].0 == path_corners[j].0
			{
				path_corners.remove(i);
				break
			}
		}
	}
}

type CornerMap = VecMap<(Option<usize>, ResultType)>;
fn find_corners(maze: &VecMap<char>) -> (Vec<Point>, CornerMap)
{
	let start = maze.find('S').expect("No start?");
	let goal  = maze.find('E').expect("No end?");
	
	let mut corners: Vec<Point> = Vec::new();
	let mut corner_map: CornerMap = maze.new_same_size((None, 0));
	
	corners.push( Point::new(start) );
	assert!(corner_map.set(start, (Some(corners.len()-1), 0)), "Failed to set start location?");
	corners.push( Point::new( goal) );
	assert!(corner_map.set(goal , (Some(corners.len()-1), 0)), "Failed to send end location?");

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
				=> { }, /* It's a corner!! */

				_ => { continue },
			}
			corners.push( Point::new((x, y)) );
			corner_map[y][x] = (Some(corners.len()-1), 0);
		}
	}
	for corner in &mut corners
	{
		for direction in util::DIRECTIONS
		{
			let mut position = corner.position;
			
			while let Some(new_pos) = maze.step(position, direction)
			{
				if maze[new_pos.1][new_pos.0] == '#' { break }
				position = new_pos;
				if new_pos == goal
				|| corner_map[new_pos.1][new_pos.0].0.is_some()
				{ break }
			}
			if position == corner.position { continue } // We went nowhere
			if corner_map[position.1][position.0].0.is_none() {continue};
			// Else
			corner.set_with_direction(direction, position);
		}
	}

	let mut next_to_goal = Vec::new();
	let mut corner_list = vec![ (start, Direction::Right) ];
	while let Some((position, direction)) = corner_list.pop()
	{
		let Some((i, score)) = corner_map_unwrap(&corner_map, position)
		else { continue };

		for new_direction in util::DIRECTIONS
		{
			// Don't go backwards
			if new_direction == direction.opposite() { continue }

			let Some(new_position) = corners[i].get_from_direction(new_direction)
			else { continue };
			let Some((i2, score2)) = corner_map_unwrap(&corner_map, new_position)
			else { continue };

			let possible_score = score + calculate_score(position, new_position, direction, new_direction);
			if score2 == 0 || score2 > possible_score
			{
				corner_map.set(new_position, (Some(i2), possible_score));
				corner_list.push((new_position, new_direction));
			}
			if new_position == goal
			{
				next_to_goal.push( ( i, possible_score ) );
			}
		}
	}
	let (_, goal_score) = corner_map_unwrap(&corner_map, goal).unwrap();
	for ( i, possible_score ) in next_to_goal
	{
		// Then we're a bad position :/
		if goal_score < possible_score
		{
			corner_map.set(corners[i].position, ( Some(i), goal_score + 1000 ));
		}
	}

	(corners, corner_map)
}

fn corner_map_unwrap(corner_map: &CornerMap, position: MapLoc) -> Option<(usize, ResultType)>
{
	let (usize_wrapped, score) = corner_map.at(position)?;
	let usize_unwrapped = usize_wrapped?;

	Some((usize_unwrapped, score))
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

#[allow(unused)]
fn print_score_map(corner_map: &CornerMap)
{
	for line in corner_map
	{
		for item in line
		{
			if item.1 == 0 { print!("      ") }
			else { print!("{:5?} ", item.1) }
		}
		println!();
	}
}

fn calculate_score(pos_1: MapLoc, pos_2: MapLoc, old_direction: Direction, new_direction: Direction) -> ResultType
{
	({
		if old_direction == new_direction { 0 }
		else if old_direction == new_direction.opposite() { 2000 }
		else { 1000 }
	} + ResultType::try_from(pos_1.0.abs_diff(pos_2.0) + pos_1.1.abs_diff(pos_2.1)).unwrap())
}


#[test]
fn part_one_test()
{
	let inputs = [(INPUT, 94444), (EXAMPLE_INPUT_1, 7036), (EXAMPLE_INPUT_2, 11048)];

	for input in inputs
	{
		let maze = util::read_char_map(input.0);
		let (_corners, corner_map) = find_corners(&maze);
		let score = part_one(&maze, &corner_map);
		assert_eq!(score, input.1, "Part 1 output was {score}, it should be {}", input.1);
	}
}
// #[test]
// fn part_two_test()
// {
// 	let inputs = [/*(INPUT, 12),*/ (EXAMPLE_INPUT_1, 45), (EXAMPLE_INPUT_2, 64)];

// 	for (i, input) in inputs.iter().enumerate()
// 	{
// 		println!("Input #{}", i+1);
// 		let maze = util::read_char_map(input.0);
// 		let (corners, corner_map) = find_corners(&maze);
		
// 		let score = part_two(&maze, &corners, &corner_map);
// 		assert_eq!(score, input.1, "Part 2 output was {score}, it should be {}", input.1);
// 	}
// }
