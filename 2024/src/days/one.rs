use std::time::{Duration, Instant};

// https://adventofcode.com/2024/day/1
// Take two lists of numbers
// Sort the lists by size
// Make pairs, starting with the smallest from each list
// Calculate the distance between each pair
// Add up all the distances

const INPUT: &str = include_str!("../../input/1.txt");

//https://adventofcode.com/2024/day/1
pub fn go(print_results: bool) -> (Duration, Duration, Duration)
{
	if print_results {println!("Day 1");}
	
	let time_before = Instant::now();
	// TODO: Do any pre-calculation here

	let pre_calc_time = time_before.elapsed();

	TimedRun!(time_before, part_one_result, part_one(), part_one_time);

	if print_results
	{
		util::print_result("Part 1", part_one_time, "The total distance is", &part_one_result);
	}
	
	TimedRun!(time_before, part_two_result, part_two(), part_two_time);
	
	if print_results
	{
		println!();
		util::print_result("Part 2", part_two_time, "The similarity score is", &part_two_result);
	}

	// Return how long it took!
	(pre_calc_time, part_one_time, part_two_time)	
}


fn part_one() -> i32
{
	// Get the lists of numbers
	let ( mut list_one, mut list_two ) = read_lists(INPUT);

	// TODO: Sort the lists of numbers small->large
	list_one.sort_unstable();
	list_two.sort_unstable();

	let mut distance = 0;
	// Calculate the distances between the list pairs
	for i in 0..list_one.len()
	{
		distance += (
			list_one.get(i).unwrap()
			-
			list_two.get(i).unwrap()
			).abs();
	}

	distance
}

fn part_two() -> i32
{
	// Get the lists of numbers
	let (list_one, list_two) = read_lists(INPUT);

	// Make a list of 0s
	let mut list_one_occurances: Vec<i32> = vec![0; list_one.len()];
	
	// Figure out how often each number in list_one appears in list_two
	for (i, num) in list_one.iter().enumerate()
	{
		for cmp_num in &list_two
		{
			if num == cmp_num
			{
				list_one_occurances[i] += 1;
			}
		}
	}

	let mut similarity = 0;
	// Multiply each number in list_one by how how often it appears
	// Add up the multiplications

	for (i, num) in list_one.iter().enumerate()
	{
		similarity += num * list_one_occurances[i];
	}
	
	similarity
}

fn read_lists(list_pairs: &str) -> ( Vec<i32>, Vec<i32> )
{
	let mut list_one = Vec::new();
	let mut list_two = Vec::new();

	for line in list_pairs.lines()
	{
		// Skip empty lines
		if line.is_empty() { continue }
		
		let ( num_1, num_2 ) = line_to_i32s( line );
		list_one.push( num_1 );
		list_two.push( num_2 );
	}

	( list_one, list_two )
}


fn line_to_i32s( line: &str ) -> ( i32, i32 )
{
	let mut split = line.split_whitespace();

	let num_1 = str_to_i32( split.next().expect("Couldn't get split 1") );
	let num_2 = str_to_i32( split.next().expect("Couldn't get split 2") );

	( num_1, num_2 )
}

fn str_to_i32(str: &str) -> i32
{
	str.parse::<i32>().expect("Error parsing number")
}
