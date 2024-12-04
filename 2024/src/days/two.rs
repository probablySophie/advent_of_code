
use colored::Colorize;
const INPUT: &str = include_str!("../../input/2.txt");
/* const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"; */

pub fn go()
{
	println!("Day 2");
	// The input is split, by line, into reports
	let reports: Vec<String> = INPUT.lines().map( std::string::ToString::to_string ).collect();
	
	println!("\t{}\n\tThe number of safe reports: {}",
		"Part 1".bold(),
		part_one(&reports)
	);
	println!("\t{}\n\tThe number of actually safe reports: {}",
		"Part 2".bold(),
		part_two(&reports)
	);
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction
{
	Up,
	Down,
	Neither
}

fn get_levels(str: &str) -> Vec<Option<i32>>
{	
	str
		// split into [str, str, str]
		.split_whitespace()
		// [str, str, str] -> [Option<i32>, Option<i32>, Option<i32>]
		// Or panicking if something went wrong
		.map( |str| Some(str.parse().expect("failed to parse str")) )
		// And collect back into a Vec<Option<i32>>
		.collect()
}

pub fn part_one(reports: &[String]) -> i32
{
	let mut num_safe = 0;
	
	'report: for report in reports
	{
		if report.is_empty() { continue }
		
		// Covert the &str into a Vec<i32>
		let levels = get_levels(report);
		
		let direction = get_direction(&levels);
		// if we aren't going either way, that's bad
		if direction == Direction::Neither { continue };

		// For each level but the last
		for i in 0..(levels.len() - 1)
		{	
			// Are this level & the next one going in the right direction?
			if ! verify(levels[i], levels[i+1], direction)
			{
				continue 'report;
			}
		}
		num_safe += 1;
	}
	num_safe
}

#[allow(clippy::module_name_repetitions, clippy::unnested_or_patterns)]
pub fn part_two(reports: &[String]) -> i32
{
	let mut num_safe = 0;
	
	'report: for report in reports
	{
		// Skip empty lines
		if report.trim().is_empty() { continue }
		
		let mut levels = get_levels(report);

		let direction = get_direction(&levels);
		// if we aren't going either way, that's bad
		if direction == Direction::Neither { continue };
		// We're now allowed to skip one bad result
		let mut skipped_one = false;
		// // For each level but the first & last
		'level: for i in 1..(levels.len()-1)
		{
			if levels[i].is_none()
			{
				// Make sure prev-next is good
				if verify(levels[i-1], levels[i+1], direction)
				{
					// They are
					continue 'level
				}
				// Else they aren't
				// and if current is None, then we've already had a bad check
				continue 'report
			}
				
			// If the previous is known bad, go back one more
			let prev_i = if levels[i-1].is_some() { i - 1 } else { i - 2 };

			match (
				// prev & current
				verify( levels[prev_i], levels[i],   direction ),
				// current & next
				verify( levels[i],      levels[i+1], direction ),
				// prev & next
				verify( levels[prev_i], levels[i+1], direction )
			)
			{
				// If prev-current & current-next are fine, then we're good!
			    (true, true, _) => continue 'level,
			    
			    // If no-one is friendly, then that's just baad
			    (false, false, false) => continue 'report,

			    // next is the common denominator
			    (true, false, false) =>
			    {
			    	levels[i+1] = None;
			    },

			    // prev is the common denominator
			    (false, true, false) => 
			    {
			    	levels[prev_i] = None;
			    },

			    // current is the common denominator
			    (false, true, true) |
			    (true, false, true) |
			    (false, false, true) =>
			    {
			    	levels[i] = None;
			    },
			}
			// If we get here, then something went wrong
			if skipped_one
			{
				continue 'report
			}
			// Else
			skipped_one = true;
		}		
		num_safe += 1;
	}
	
	num_safe
}

fn get_direction(levels: &[Option<i32>]) -> Direction
{	
	let mut direction_decider = 0;
	for i in 0..(levels.len()-1) {
		match levels[i].cmp(&levels[i+1])
		{
			// next is greater (going up)
		    std::cmp::Ordering::Less => 
		    {
		    	direction_decider += 1;
		    },
		    std::cmp::Ordering::Equal => 
		    {
		    	// ignore
		    },
			// next is smaller (going down)
		    std::cmp::Ordering::Greater =>
		    {
		    	direction_decider -= 1;
		    },
		}
	}
	match direction_decider.cmp(&0)
	{
		std::cmp::Ordering::Less    => Direction::Down,
		std::cmp::Ordering::Greater => Direction::Up,
		std::cmp::Ordering::Equal   => Direction::Neither
	}
}


fn verify(one: Option<i32>, two: Option<i32>, direction: Direction) -> bool
{
	// Are they in the acceptible range of difference?
	if one.unwrap().abs_diff(two.unwrap()) > 3 || one == two
	{
		return false
	}

	// Are they going the right direction?
	match direction
	{
	    Direction::Up => one < two,
	    Direction::Down => one > two,
	    Direction::Neither => false,
	}
}
