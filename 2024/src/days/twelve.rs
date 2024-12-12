use std::time::{Duration, Instant};

use util::{print_map, CharMap, Direction, MapFunction, MapLoc};

#[allow(unused)]
const INPUT: &str = include_str!("../../input/12.txt");
#[allow(unused)]
const EXAMPLE_INPUT_1: &str = "AAAA
BBCD
BBCC
EEEC";
#[allow(unused)]
const EXAMPLE_INPUT_2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
#[allow(unused)]
const EXAMPLE_INPUT_3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

// For if we need to change it up!
type ResultType = i32;
const DIRECTIONS: [Direction; 4] = [ Direction::Up, Direction::Right, Direction::Down, Direction::Left ];

//https://adventofcode.com/2024/day/12
pub fn go(print_results: bool) -> (Duration, Duration, Duration)
{
	if print_results {println!("Day 12");}
	
	let time_before = Instant::now();
	// ~ ~ ~ ~ ~ PRE CALCULATION ~ ~ ~ ~ ~

	// let regions = get_regions(EXAMPLE_INPUT_3);
	let regions = get_regions(INPUT);

	// ~ ~ ~ ~ ~ END OF PRE CALCULATION ~ ~ ~ ~ ~
	let pre_calc_time = time_before.elapsed();

	// Part 1
	TimedRun!(time_before, part_one_result, part_one(&regions), part_one_time);

	if print_results
	{
		util::print_result("Part 1", part_one_time, "The total fence price is", &part_one_result);
	}

	// Part 2
	TimedRun!(time_before, part_two_result, part_two(&regions), part_two_time);
	
	if print_results
	{
		println!();
		util::print_result("Part 2", part_two_time, "The bulk fence price is", &part_two_result);
	}

	// Return how long it took!
	(pre_calc_time, part_one_time, part_two_time)	
}

fn part_one(region_map: &[Region]) -> ResultType
{
	// Work out the area and perimeter for each
	// multiply them together
	let mut fence_price = 0;
	// let debug = true;
	let debug = false;
	for region in region_map
	{
		let mut perimeter = 0;

		for y in 0..region.shape.len()
		{			
			for x in 0..region.shape[0].len()
			{
				// If it's empty, then we don't want it
				if region.shape.at((x, y)) != Some(region.plant)
				{
					continue
				};
				
				// work out the perimeter per spot
				for direction in DIRECTIONS
				{
					// Is there anything in this direction?
					if let Some(loc) = region.shape.step((x, y), direction)
					{
						// Is it an empty spot?
						if region.shape.at(loc) != Some(region.plant)
						{
							perimeter += 1;
							if debug && region.plant == PROBLEM_PLANT
							{
								println!("({x},{y}) {direction:?} {loc:?}");
							}
						}
					}
					else // no
					{
						perimeter += 1;
						if debug && region.plant == PROBLEM_PLANT
						{
							println!("({x},{y}) {direction:?}");
						}
					}
				}
			}			
		}
		if debug && region.plant == PROBLEM_PLANT
		{
			print_map(&region.shape);
		}
		fence_price += perimeter * region.area;
		if debug
		{
			println!("{}: {} * {perimeter} = {}", region.plant, region.area, perimeter * region.area);
			util::print_map(&region.shape);
		}
	}
	fence_price
}

fn part_two(region_map: &[Region]) -> ResultType
{
	let mut fence_price = 0;
	// let debug = true;
	let debug = false;
	// Work out the total number of continuous sides per region * multiply it by the area
	for (i, region) in region_map.iter().enumerate()
	{
		let mut num_sides = 0;

		// Working out lines on the tops and bottoms of the shape's locations
		for y in 0..region.shape.len()
		{
			let mut top_line = false;
			let mut bottom_line = false;
			for x in 0..region.shape[0].len()
			{
				let currently_good = region.shape.at((x, y)) == Some(region.plant);
				if is_empty(&region.shape, (x, y), Direction::Up, region.plant)
				&& currently_good
				{
					if ! top_line { top_line = true; }
				}
				else if top_line
				{
					if debug { println!("{y}: top line"); }
					top_line = false;
					num_sides += 1;
				}
				if is_empty(&region.shape, (x, y), Direction::Down, region.plant)
				&& currently_good
				{
					if ! bottom_line { bottom_line = true; }
				}
				else if bottom_line
				{
					if debug { println!("{y}: bottom line"); }
					bottom_line = false;
					num_sides += 1;
				}
			}
			if top_line
			{
				if debug { println!("{y}: top line"); };
				num_sides += 1;
			}
			if bottom_line
			{
				if debug { println!("{y}: bottom line"); }
				num_sides += 1;
			}
		}
		// Working out lines on the lefts and rights of the shape's locations
		for x in 0..region.shape[0].len()
		{
			let mut left_line = false;
			let mut right_line = false;
			for y in 0..region.shape.len()
			{
				let currently_good = region.shape.at((x, y)) == Some(region.plant);
				if is_empty(&region.shape, (x, y), Direction::Left, region.plant)
				&& currently_good
				{
					if ! left_line { left_line = true; }
				}
				else if left_line
				{
					if debug { println!("{x}: left line"); }
					left_line = false;
					num_sides += 1;
				}
				if is_empty(&region.shape, (x, y), Direction::Right, region.plant)
				&& currently_good
				{
					if ! right_line { right_line = true; }
				}
				else if right_line
				{
					if debug { println!("{x}: right line"); }
					right_line = false;
					num_sides += 1;
				}
			}
			if left_line
			{ 
				if debug { println!("{x}: left line"); }
				num_sides += 1;
			};
			if right_line {
				if debug { println!("{x}: right line"); }
				num_sides += 1;
			};
		}

		// println!("sides: {num_sides} area: {}", region.area);
		// util::print_map(&region.shape);
		// println!();
		
		fence_price += num_sides * region.area;
	}
	fence_price
}

fn is_empty(map: &CharMap, location: MapLoc, direction: Direction, non_empty: char) -> bool
{
	if let Some(loc) = map.step(location, direction)
	{
		if map.at(loc) == Some(non_empty)
		{
			return false
		}
	}
	// Else
	true
}

#[allow(unused)]
struct Region
{
	pub plant: char,
	pub shape: Vec<Vec<char>>,
	pub area: i32,
}

fn get_regions(input: &str) -> Vec<Region>
{
	let mut regions = Vec::new();

	let mut plot_map = util::read_char_map(input);

	for y in 0..plot_map.len()
	{
		for x in 0..plot_map[0].len()
		{
			if plot_map.at((x, y)) != Some(' ')
			{
				regions.push(get_single_region(&mut plot_map, (x, y), true));
			}
		}
	}
	regions
}

/// Returns
/// * Width
/// * Height
/// * Offset
/// * Non-relative Locations
fn find_locations(map: &mut CharMap, start_location: MapLoc, destructive: bool, plant: char) -> (usize, usize, MapLoc, Vec<MapLoc>)
{
	let mut locations = Vec::new();
	let mut search_range = vec![ start_location ];
	let mut left_most = map[0].len();
	let mut right_most = 0;
	let mut top_most = map.len();
	let mut bottom_most = 0;
	
	while let Some(location) = search_range.pop()
	{
		if map.at(location) != Some(plant)
		{
			continue // If it's not what we're looking for.  Skip
		}

		if location.0 < left_most { left_most = location.0 };
		if location.0 > right_most { right_most = location.0 };
		if location.1 < top_most { top_most = location.1 };
		if location.1 > bottom_most { bottom_most = location.1 };
		
		// else
		locations.push(location);
		// Add all the possible directions to our search list
		for direction in DIRECTIONS
		{
			if let Some(new_location) = map.step(location, direction)
			{
				if map.at(location) == Some(plant)
				{
					search_range.push(new_location);
				}
			}
		}
		if destructive
		{
			if plant == PROBLEM_PLANT {
				map.set(location, '~');
			} // TEMP
			else
			{
				map.set(location, ' ');
			}
		}
	}

	let offset: MapLoc = (start_location.0 - left_most, start_location.1 - top_most);
	
	(right_most - left_most, bottom_most - top_most, offset, locations)
}

const PROBLEM_PLANT: char = '.';

fn get_single_region(map: &mut CharMap, start_location: MapLoc, destructive: bool) -> Region
{	
	let plant = map.at(start_location).unwrap();
	let mut area = 0;

	let (width, height, offset, found_locations) = find_locations(map, start_location, destructive, plant);
	let mut region_map: CharMap = vec![ vec![ ' '; width + 1 ]; height + 1 ];

	if plant == PROBLEM_PLANT {
		util::print_map(map);
	} // TEMP

	for location in found_locations
	{
		// Get the relative location for each spot
		if plant == PROBLEM_PLANT
		{
			print!("{location:?} ");
		}
		
		let x_dif = (start_location.0).abs_diff(location.0);
		let shape_y = start_location.1.abs_diff(location.1);
		let shape_x = 
			match start_location.0.cmp(&location.0)
			{
				std::cmp::Ordering::Less => offset.0 + x_dif,
				std::cmp::Ordering::Equal => offset.0,
				std::cmp::Ordering::Greater => offset.0 - x_dif,
			};
		
		region_map[shape_y][shape_x] = plant;
		
		area += 1;
		// TEMP
		if plant == PROBLEM_PLANT {
			println!("shapeloc ({shape_x}, {shape_y}) width: {} height: {}", region_map[0].len(), region_map.len());
			util::print_map(&region_map);
		};
		// TEMP
	}
	Region
	{ 
		plant,
		area,
		shape: region_map
	}
}
