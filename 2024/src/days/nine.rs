use std::fmt::Result;

#[allow(unused)]
const INPUT: &str = include_str!("../../input/9.txt");
#[allow(unused)]
const EXAMPLE_INPUT_1: &str = "2333133121414131402";
#[allow(unused)]
const EXAMPLE_INPUT_2: &str = "";

// For if we need to change it up!
type ResultType = i64;

//https://adventofcode.com/2024/day/9
pub fn go()
{
	println!("Day 9");

	let time_before = std::time::Instant::now();
	let part_one_result = part_one();
	let time_elapsed = time_before.elapsed();
	
	util::print_result("Part 1", time_elapsed, "Part 1 description", &part_one_result);

	println!();
	
	let time_before = std::time::Instant::now();
	let part_two_result = part_two();
	let time_elapsed = time_before.elapsed();
	
	util::print_result("Part 2", time_elapsed, "Part 2 description", &part_two_result);	
}

fn part_one() -> ResultType
{
	// let mut frags = disk_map_to_file_space(EXAMPLE_INPUT_1);
	let mut frags = disk_map_to_file_space(INPUT);
	let mut empty_spaces: Vec<usize> = Vec::new();

	for (i, id) in frags.iter().enumerate()
	{
		if *id == -1
		{
			empty_spaces.push(i);
		}
	}

	for i in (0..frags.len()).rev()
	{
		// Skip if its an empty spot
		if frags[i] == -1 { continue }

		let empty_index = empty_spaces.remove(0);

		// Break if we're done
		if empty_index >= i { break; }

		frags[empty_index] = frags[i];
		frags[i] = -1;
	}

	calculate_checksum(&frags)
}

#[allow(clippy::needless_range_loop)]
fn part_two() -> ResultType
{
	let mut frags = Vec::new();	
	let mut is_file = true;
	let mut file_id = 0;

	// Get our frags, but as (file_id, size)
	for c in INPUT.chars()
	{
		if ! c.is_numeric() { continue };
		let size: usize = c.to_string().parse().expect("Failed to convert to usize");
		let now = if is_file { file_id } else { -1 };
		frags.push( ( ResultType::from(now) , size ) );

		if is_file {file_id += 1};
		is_file = !is_file; // we alternate between free space & file block sizes
	}

	// Do the moves :)
	for i in (0..frags.len()).rev()
	{
		let (file_id, block_size) = frags[i];
		// Skip if its an empty spot
		if file_id == -1 { continue }

		let mut empty_index: Option<usize> = None;
		let mut empty_space = 0;
		for i in 0..frags.len()
		{
			if frags[i].0 != -1 {continue};
			let empty_size = frags[i].1;
			
			if empty_size >= block_size
			{
				empty_index = Some(i);
				empty_space = empty_size;
				break;
			}
		}
		let empty_index = match empty_index
		{	None    => continue,
			Some(v) => v,
		};
		if empty_index > i
		{
			continue;
		}
		// println!("{i}->{empty_index} moving: {}", frags[i].0);
		frags[empty_index] = frags[i];
		frags[i] = (-1, block_size);
		if empty_space-block_size > 0
		{
			frags.insert(empty_index+1, (-1, empty_space-block_size));
		}
		// print_frags(&make_single_frags(&frags));
	}

	calculate_checksum(&make_single_frags(&frags))
}

fn make_single_frags(frags: &[(ResultType, usize)]) -> Vec<ResultType>
{
	let mut single_frags: Vec<ResultType> = Vec::new();

	for (id, size) in frags
	{
		single_frags.append( &mut vec![ *id; *size ] );
	}
	single_frags
}

#[allow(unused)]
fn print_frags(ids: &[ResultType])
{
	for id in ids
	{
		if *id == -1
		{
			print!(".");
		}
		else
		{
			print!("{id}");
		}
	}
	println!();
}

fn calculate_checksum(frags: &[ResultType]) -> ResultType
{
	let mut checksum = 0;
	for (i, file_id) in frags.iter().enumerate()
	{
		if *file_id == -1 { continue }
		checksum += file_id * ResultType::try_from(i).expect("Failed to convert usize to i32");
	}
	checksum
}

fn disk_map_to_file_space(input: & str) -> Vec<ResultType>
{
	let mut frags = Vec::new();

	let mut is_file = true;
	let mut file_id = 0;
	
	for c in input.chars()
	{
		if ! c.is_numeric() { continue };
		let size: usize = c.to_string().parse().expect("Failed to convert to i32");
		let now = if is_file
		{
			file_id
		}
		else
		{
			-1
		};
		frags.append( &mut vec![ now ; size ] );

		if is_file {file_id += 1};
		is_file = !is_file; // we alternate between free space & file block sizes
	}

	frags
}

fn find_empty(ids: &[ResultType]) -> Option<usize>
{
	for (i, id) in ids.iter().enumerate()
	{
		if *id == -1
		{
			return Some(i)
		}
	}
	None
}
