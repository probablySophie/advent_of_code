use std::time::{Duration, Instant};

use util::{VecMap, MapFunction};

#[allow(unused)]
const INPUT: &str = include_str!("../../input/8.txt");
#[allow(unused)]
const EXAMPLE_INPUT_1: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

// For if we need to change it up!
type ResultType = i32;

//https://adventofcode.com/2024/day/8
pub fn go(print_results: bool) -> (Duration, Duration, Duration)
{
	if print_results {println!("Day 8");}

	let time_before = Instant::now();
	
	let map = util::read_char_map(INPUT);
	// let map = util::read_char_map(EXAMPLE_INPUT_1);
	
	let pre_calc_time = time_before.elapsed();

	TimedRun!(time_before, part_one_result, part_one(&map), part_one_time);

	if print_results
	{
		util::print_result("Part 1", part_one_time, "Num Unique Antinode Locations", &part_one_result);
	}

	
	TimedRun!(time_before, part_two_result, part_two(&map), part_two_time);

	if print_results
	{
		println!();
		util::print_result("Part 2", part_two_time, "Num Unique repeated antinode locations", &part_two_result);	
	}	
	
	(pre_calc_time, part_one_time, part_two_time)
}

fn part_one(map: &VecMap<char>) -> ResultType
{
	let frequencies = get_frequencies(map);
	let mut antinode_maps: Vec<VecMap<char>> = Vec::new();

	// Make an antinode map for each frequency
	for frequency in frequencies
	{
		antinode_maps.push(find_antinodes(map, frequency, true));
	}

	// And count all the unique antinode locations
	count_antinodes(&antinode_maps)
}

fn part_two(map: &VecMap<char>) -> ResultType
{
	let frequencies = get_frequencies(map);
	let mut antinode_maps: Vec<VecMap<char>> = Vec::new();

	// Make an antinode map for each frequency
	for frequency in frequencies
	{
		antinode_maps.push(find_antinodes(map, frequency, false));
	}

	// And count all the unique antinode locations
	count_antinodes(&antinode_maps)
}

fn count_antinodes(maps: &[VecMap<char>]) -> ResultType
{
	let mut antinode_locations = 0;
	for y in 0..maps[0].len()
	{
		for x in 0..maps[0][0].len()
		{
			for antinode_map in maps
			{
				if antinode_map[y][x] == '#'
				{
					antinode_locations += 1;
					break
				}
			}
		}
	}
	
	antinode_locations
}

/// Get all the frequencies that are in a map
fn get_frequencies(map: &VecMap<char>) -> Vec<char>
{
	let mut frequencies = Vec::new();

	for map_line in map // For each line on the map (y)
	{
		for map_spot in map_line // For each spot on the line (x)
		{
			if map_spot != &'.' // Is there something there?
			{
				let mut already_there = false;
				for c in &frequencies { if c == map_spot { already_there = true } }
				if ! already_there
				{
					frequencies.push(*map_spot);
				}
			}
		}
	}

	frequencies
}

/// Finds all anti-nodes for a given frequency
fn find_antinodes(map: &VecMap<char>, frequency: char, single_loop: bool) -> VecMap<char>
{
	let mut antinodes = vec![ vec![ '.' ; map[0].len() ] ; map.len() ];
	let mut node_locations: Vec<(usize, usize)> = Vec::new();

	// Make a list of all the node locations
	for (y, line) in map.iter().enumerate()
	{
		for (x, c) in line.iter().enumerate()
		{
			if c == &frequency
			{
				node_locations.push((x, y));
			}
		}
	}
	// Calculate all the antinode locations
	for i_1 in 0..node_locations.len()
	{
		for i_2 in 0..node_locations.len()
		{
			if i_1 == i_2 { continue } // that's the same guy!
			let change = (
				i32::try_from(node_locations[i_2].0).expect(":(")
				-
				i32::try_from(node_locations[i_1].0).expect(":(")
				,
				i32::try_from(node_locations[i_2].1).expect(":(")
				-
				i32::try_from(node_locations[i_1].1).expect(":(")
			);
			// Take 1 step (should get us to i_2)
			let mut current_loc = 
			{
				if single_loop
				{
					match map.get_new_location(node_locations[i_1], change)
					{ 
						None => continue, 
						Some(v) => v 
					}
				}
				else
				{
					node_locations[i_1]
				}
			};
			
			while let Some(antinode) = map.get_new_location(current_loc, change)
			{
				current_loc = antinode;
				// If there's a tower on the spot, then it's not a spot
				antinodes[antinode.1][antinode.0] = '#';
				
				if single_loop { break }
			}
		}
	}
	antinodes
}

