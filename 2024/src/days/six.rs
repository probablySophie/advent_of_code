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
	let Some(mut position) = map.find('^')
	else { return -1 };
	let mut direction = Direction::Up;
	let mut unique_positions = 0;

	while let Some(new_position) = map.step(position, direction)
	{
		if map.at(position) != Some('X')
		{
			unique_positions += 1;
			map.set(position, 'X'); // Record where we've been
		}
		if map.at(new_position) == Some('#') // Are we about to hit something?
		{
			direction = direction.turn_right(); // Turn right!
			continue
		}
		position = new_position; // Update the guard's location
	}
	if map.at(position) != Some('X')
	{
		unique_positions += 1;
		map.set(position, 'X'); // Record where we've been
	}
	// print_map(map);
	unique_positions
}

fn part_two(map: &mut LevelMap) -> i32
{
	let (points, point_map) = Point::build_point_map(map);

	let mut checked_obstacles: Vec<Vec<Option<bool>>> = 
		vec![ vec![ None; map[0].len() ]; map.len() ];
		
	let Some(starting_location) = map.find('^') else { return -1 };

	let mut obstacle_counter = 0; // Our score
	let mut direction = Direction::Up;
	let mut position = starting_location;
	while let Some(new_pos) = map.step(position, direction)
	{
		// Did we bump into something
		if map[new_pos.1][new_pos.0] == '#'
		{
			direction = direction.turn_right();
			continue;
		}
		// Now check if adding an obstacle in front of the guard would cause a loop
		if checked_obstacles[new_pos.1][new_pos.0].is_none()
		&& new_pos != starting_location
		{
			let result = point_loop(map, &points, &point_map, new_pos, starting_location);
			checked_obstacles[new_pos.1][new_pos.0] = Some( result ); // Save the result
			if result { obstacle_counter += 1; } // Increase our counter if it was a loop!
		}
		position = new_pos; // Update the guard's location
		// println!("{:?}", points[point_map.at((52, 82)).unwrap().unwrap()]);
		// panic!();
	}
	for line in point_map
	{
		for item in line
		{
			match item
			{
				None => { print!(" ") },
				Some(u) => {
					match (
						points[u].from_below.is_some(),
						points[u].from_above.is_some(),
						points[u].from_left.is_some(),
						points[u].from_right.is_some()
					)
					{
					    (true, true, true, true) => print!("╋"),    // all directions
					    (true, true, true, false) => print!("┫"),   // 
					    (true, true, false, true) => print!("┣"),   //
					    (true, true, false, false) => print!("┃"),  //
					    (true, false, true, true) => print!("┳"),   //
					    (true, false, true, false) => print!("┓"),  //
					    (true, false, false, true) => print!("┏"),  //
					    (true, false, false, false) => print!("┛"), //
					    (false, true, true, true) => print!("┻"),   //
					    (false, true, true, false) => print!("┗"),  //
					    (false, true, false, true) => print!("╻"),  //
					    (false, true, false, false) => print!("╹"), //
					    (false, false, true, true) => print!("━"),  //
					    (false, false, true, false) => print!("╸"), //
					    (false, false, false, true) => print!("╺"), //
					    
					    (false, false, false, false) => print!("."),
					}
				},
			}
		}
		println!();
	}
	if map.len() < 20
	{
		assert_eq!(6, obstacle_counter);
	}
	else
	{
		assert_eq!(1705, obstacle_counter);
	}
	obstacle_counter // return our score
}

fn get_starting_pos(map: &LevelMap, points: &[Point], point_map: &PointMap, obstacle_location: MapLoc, starting_position: MapLoc, direction: Direction) -> (Option<MapLoc>, Direction)
{
	let position =
		obstacle_or_location(
			starting_position,
			Point::best_point_on_line(points, starting_position, direction),
			obstacle_location
	);
	if point_map.at(position).unwrap().is_none()
	{
		// Step backwards
		let pos = map.step(position, direction.opposite()).unwrap();
		// Turn right (we hit something)
		let dir = direction.turn_right();
		return (Point::best_point_on_line(points, pos, dir), dir);
	};
	// Else
	(Some(position), Direction::Up)
}

fn point_loop(map: &mut LevelMap, points: &[Point], point_map: &PointMap, obstacle_location: MapLoc, starting_position: MapLoc) -> bool
{
	let debug_print = false;
	if debug_print {println!("\nObstacle: {obstacle_location:?}");} // TEMP
	let mut map_clone = map.clone(); // TEMP
	map_clone.set(obstacle_location, 'O');

	// Either the first point we hit, or the obstacle itself
	let (mut position, mut direction) = get_starting_pos(map, points, point_map, obstacle_location, starting_position, Direction::Up);

	// TEMP map drawing & print
	if let Some(pos) = position { map_clone.set(pos, '1'); }
	
	let mut next_point: Option<MapLoc> = None;
	let mut hits: Vec<(MapLoc, Direction)> = Vec::new();
	let mut is_loop: bool = false;

	let mut i = 0;
	'mainLoop: while let Some(current_position) = next_point.or(position)
	{
		if debug_print {println!("step {i} at {current_position:?} facing {direction:?}"); i += 1;}
		// If we're standing on an obstacle, then the current point is none
		let point = if let Some(u) = point_map.at(current_position).unwrap() 
		{ 
			points[u].clone() // We're standing on a point :)
		}
		else
		{ // Walk from here
			let temp_loc = map.step(current_position, direction.opposite()).unwrap();
			let new_dir = direction.turn_right();

			match walk_from(map, temp_loc, new_dir, obstacle_location)
			{
				None => { break },
				Some((new_position, new_direction)) => 
				{
					next_point = None;
					position = Some(new_position);
					direction = new_direction;
				}
			}
			
			continue
		};
		
		map_clone.set(point.location, '+'); // TEMP
		
		// Have we been here before?
		for hit in &hits
		{
			if hit.0 == current_position
			&& hit.1 == direction
			{
				is_loop = true; // we've been here before!
				break 'mainLoop
			}
		}
		hits.push((current_position, direction)); // Add this spot to the list
		
		let Some(going_to) = point.from(direction) // Where are we going?!
		
		else // Off the edge of the map :(
		{
			let going_from = map.step(point.location, direction.opposite()).unwrap();
			direction = direction.turn_right();
			if debug_print {println!("point {:?} walking from {going_from:?} going {direction:?}", point.location);}
			
			match walk_from(map, going_from, direction, obstacle_location)
			{
				None =>
				{
					map_clone.set(going_from, direction.to_char()); // TEMP
					if debug_print{println!("Walked off the edge - going_to was none & from {going_from:?}");}
					is_loop = false;
					break 'mainLoop
				},
				Some((walked_to, now_facing)) =>
				{
					next_point = None;
					position = Some(walked_to);
					direction = now_facing;
				}
			}
			if position == Some(obstacle_location)
			{
				println!("We hit
	    	(( going_from.0 < going_to.0 // We're less than both the final location
	    	&& going_from.0 < obstacle.0 // And the obs the obstacle!");
			}
			continue
		};

		next_point = Some(
			obstacle_or_location(
				map.step(current_position, direction.opposite()).unwrap(), 
				Some(going_to), 
				obstacle_location
			)
		);
		position = None;
		direction = direction.turn_right();		
	}
	// And that's the end...
	if debug_print {
		util::print_map(&map_clone); // TEMP
		println!("Is loop? {is_loop}"); // TEMP
	}
	is_loop
}

fn walk_from(map: &LevelMap, location: MapLoc, direction: Direction, obstacle_location: MapLoc) -> Option<(MapLoc, Direction)>
{
	// let start_loc = location;
	let (mut location, mut direction) = (location, direction);
	let mut hit_something = false;
	
	while ! hit_something
	{
		// Have we walked off of the map?
		let new_loc = map.step(location, direction)?;
		if map.at(new_loc) == Some('#')
		{
			hit_something = true;
		}
		else if new_loc == obstacle_location
		{
			direction = direction.turn_right();
			continue
		}
		location = new_loc;
	}
	Some((location, direction))
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



#[derive(Default, Clone, Debug)]
struct Point
{
	pub location: MapLoc,
	pub from_below: Option<MapLoc>,
	pub from_above: Option<MapLoc>,
	pub from_left: Option<MapLoc>,
	pub from_right: Option<MapLoc>,
}
type PointMap = Vec<Vec<Option<usize>>>;
impl Point
{
	pub fn from(&self, direction: Direction) -> Option<MapLoc>
	{
		match direction
		{
			Direction::Up => self.from_below,
			Direction::Down => self.from_above,
			Direction::Left => self.from_right,
			Direction::Right => self.from_left,
		}
	}
	#[allow(clippy::needless_range_loop)]
	pub fn build_point_map(map: &LevelMap) -> (Vec<Point>, PointMap)
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
			let location = points[i].location;
			
			if let Some(pos) = map.step(location, Direction::Down)
			{ points[i].from_below = Point::best_point_on_line(&points, pos, Direction::Right) }

			if let Some(pos) = map.step(location, Direction::Left)
			{ points[i].from_left = Point::best_point_on_line(&points, pos, Direction::Down) }
			
			if let Some(pos) = map.step(location, Direction::Right)
			{ points[i].from_right = Point::best_point_on_line(&points, pos, Direction::Up) }

			if let Some(pos) = map.step(location, Direction::Up)
			{ points[i].from_above = Point::best_point_on_line(&points, pos, Direction::Left) }
		}
		(points, point_map)
	}

	pub fn best_point_on_line(points: &[Point], location: MapLoc, direction: Direction) -> Option<MapLoc>
	{
		let mut best: Option<MapLoc> = None;

		for point in points
		{
			let l1 = point.location;
			let l2 = location;

			if direction != match (l1.0.cmp(&l2.0), l1.1.cmp(&l2.1))
			{
			    (std::cmp::Ordering::Less   , std::cmp::Ordering::Equal  ) => Direction::Left,
			    (std::cmp::Ordering::Equal  , std::cmp::Ordering::Less   ) => Direction::Up,
			    (std::cmp::Ordering::Equal  , std::cmp::Ordering::Greater) => Direction::Down,
			    (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal  ) => Direction::Right,
			    _ => { continue }
			}
			{
				continue
			}
			// println!("{l2:?} {l1:?} {:?} {:?}", l1.0.cmp(&l2.0), l1.1.cmp(&l2.1));
			// println!("{:?} {:?} {:?}", l1.0.abs_diff(l2.0), l1.1.abs_diff(l2.1), direction);
			
			match (l1.0.abs_diff(l2.0), l1.1.abs_diff(l2.1), direction)
			{
				(_, 0, Direction::Left | Direction::Right) => if best.is_none() || l1.0.abs_diff(l2.0) < l1.0.abs_diff(best.unwrap().0) { best = Some(l1) }
				(0, _, Direction::Up   | Direction::Down ) => if best.is_none() || l1.1.abs_diff(l2.1) < l1.1.abs_diff(best.unwrap().1) { best = Some(l1) },
				_ => {} // Bad
			 }
		}
		best
	}
}

fn set_if_less_or_none(point: &mut Point, location: MapLoc, direction: Direction)
{
	match direction
	{
		Direction::Up =>
		{
			if point.from_above.is_none()
			|| point.location.1.abs_diff(location.1) 
				< point.location.1.abs_diff(point.from_above.unwrap().1)
			{
				point.from_above = Some(location);
			}
		},
		Direction::Down =>
		{
			if point.from_below.is_none()
			|| point.location.1.abs_diff(location.1) 
				< point.location.1.abs_diff(point.from_below.unwrap().1)
			{
				point.from_below = Some(location);
			}
		},
		Direction::Left =>
		{
			if point.from_left.is_none()
			|| point.location.0.abs_diff(location.0) 
				< point.location.0.abs_diff(point.from_left.unwrap().0)
			{
				point.from_left = Some(location);
			}
		},
		Direction::Right =>
		{
			if point.from_right.is_none()
			|| point.location.0.abs_diff(location.0) 
				< point.location.0.abs_diff(point.from_right.unwrap().0)
			{
				point.from_right = Some(location);
			}
		},
	}
}

#[test] // (6, 4) to (6, 9) or (6, 7)
fn test_obstacle_or_location_1() { println!();
	assert_eq!((6, 7),
		obstacle_or_location((6, 4), Some((6, 9)), (6, 7))
	);
}
#[test] // (8, 6) to (1, 6) or (3, 6)
fn test_obstacle_or_location_2() { println!();
	assert_eq!((3, 6),
		obstacle_or_location((8, 6), Some((1, 6)), (3, 6))
	);
}

fn obstacle_or_location(going_from: MapLoc, going_to: Option<MapLoc>, obstacle: MapLoc) -> MapLoc
{
	let Some(going_to) = going_to else { return obstacle };
	// println!("\tfrom {going_from:?} to {going_to:?} or {obstacle:?}");	

	// Did we change in X, or Y?
	match ( going_from.0.abs_diff(going_to.0) == 0, going_from.1.abs_diff(going_to.1) == 0 )
	{
	    (false, false) | // That's... not on a straight line...?
	    (true , true ) => {}, // We stayed in the same spot?
	    
	    (true , false) => // Change in Y
	    {
	    	#[cfg(test)] { println!("\tChange in Y") }
	    	// No X difference please
	    	if going_from.0.abs_diff(obstacle.0) == 0
	    	&& going_from.1.abs_diff(going_to.1) > going_from.1.abs_diff(obstacle.1) &&
	    	(( going_from.1 < going_to.1 // We're less than both the final location
	    	&& going_from.1 < obstacle.1 // And the obstacle
	    	) ||
	    	(  going_from.1 > going_to.1 // We're less than both the final location
	    	&& going_from.1 > obstacle.1 // And the obstacle
	    	))
	    	{
	    		return obstacle
	    	}
	    },
	    (false, true ) => // Change in X
	    {
	    	#[cfg(test)] { println!("\tChange in X") }
	    	// No Y difference please
	    	if going_from.1.abs_diff(obstacle.1) == 0
	    	&& going_from.0.abs_diff(going_to.0) > going_from.0.abs_diff(obstacle.0) &&
	    	(( going_from.0 < going_to.0 // We're less than both the final location
	    	&& going_from.0 < obstacle.0 // And the obstacle
	    	) ||
	    	(  going_from.0 > going_to.0 // We're less than both the final location
	    	&& going_from.0 > obstacle.0 // And the obstacle
	    	))
	    	{
	    		return obstacle
	    	}
	    },
	}
	going_to
}

