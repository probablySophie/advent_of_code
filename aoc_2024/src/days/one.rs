
// https://adventofcode.com/2024/day/1
// Take two lists of numbers
// Sort the lists by size
// Make pairs, starting with the smallest from each list
// Calculate the distance between each pair
// Add up all the distances

use core::panic;

use colored::Colorize;

const INPUT: &str = include_str!("../../input/one.txt");

pub fn go()
{
	println!("Day 1:");

	println!("\t{}\n\tThe total distance is: {}",
		"Part 1".bold(),
		part_one()
	);

	println!("\t{}\n\tThe similarity score is: {}",
		"Part 2".bold(),
		part_two(),
	);
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

	let num_1 = match split.next()
		{
			None => panic!( "Couldn't split 1 from line {}", line ),
			Some(str) => 
			{
				str_to_i32(str)
			}
		};
	
	let num_2 = match split.next()
		{
			None => panic!( "Couldn't get split 2 from line {}", line ),
			Some(str) => 
			{
				str_to_i32(str)
			}
		};

	( num_1, num_2 )
}

fn str_to_i32(str: &str) -> i32
{
	match str.parse::<i32>()
	{
		Err(error) => panic!( "Error parsing number: {}", error ),
		Ok(num) => num,
	}
}
