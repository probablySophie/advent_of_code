use std::ops::Sub;

use colored::Colorize;

#[allow(unused)]
const INPUT: &str = include_str!("../../input/4.txt");
#[allow(unused)]
const EXAMPLE_INPUT_1: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
#[allow(unused)]
const EXAMPLE_INPUT_2: &str = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";


const DIRECTIONS: [(i32, i32); 8] = [
	(0 ,  1), // up
	(0 , -1), // down
	(1 ,  0), // right
	(1 ,  1), // right up
	(1 , -1), // right down
	(-1,  0), // left
	(-1,  1), // left up
	(-1, -1), // left down
];

pub fn go()
{
	println!("Day 4");
	
	let mut grid = Vec::new();
	for line in INPUT.lines()
	// for line in EXAMPLE_INPUT_1.lines()
	{
		if line.is_empty() {continue}
		grid.push( line.chars().collect() );
	}

	println!("\t{}\n\tThe number of times 'XMAS' appears is: {}",
		"Part 1".bold(),
		part_one(&grid)
	);
	
	println!("\t{}\n\tThe number of times an 'X-MAS' appears is: {}",
		"Part 2".bold(),
		part_two(&grid)
	);
}

#[allow(clippy::cast_sign_loss)]
/// Check if a space in a given direction is safe to look at
fn is_safe(x: usize, y: usize, change_x: i32, change_y: i32, max_width: usize, max_height: usize) -> Option<(usize, usize)>
{
	let return_x = (match 0.cmp(&change_x)
		{
			std::cmp::Ordering::Less => x.checked_add( change_x as usize ),
			std::cmp::Ordering::Equal => Some(x),
			std::cmp::Ordering::Greater => x.checked_sub( change_x.unsigned_abs() as usize ),
		})?;

	let return_y = (match 0.cmp(&change_y)
		{
			std::cmp::Ordering::Less => y.checked_add( change_y as usize ),
			std::cmp::Ordering::Equal => Some(y),
			std::cmp::Ordering::Greater => y.checked_sub( change_y.unsigned_abs() as usize ),
		})?;
	
	if return_x > max_width || return_y > max_height
	{
		return None;
	}

	Some((return_x, return_y))
}

fn check_direction(input_grid: &[Vec<char>], start_x: usize, start_y: usize, direction: (i32, i32), chars: &[char]) -> i32
{
	let mut current_x = start_x;
	let mut current_y = start_y;
	for i in 0..(chars.len()-1)
	{
		match is_safe(current_x, current_y, direction.0, direction.1, input_grid[0].len()-1, input_grid.len()-1)
		{
			None => return 0,
			Some((new_x, new_y)) => 
			{
				current_x = new_x;
				current_y = new_y;
			}
		};
		// If we're here, then it's at least a valid spot
		if input_grid[current_y][current_x] != chars[i+1]
		{
			return 0
		}
	}
	// Yay?!
	1
}

fn part_one(input_grid: &[Vec<char>]) -> i32
{
	let mut num_found = 0;
	let chars = ['X', 'M', 'A', 'S'];

	for ( y, line ) in input_grid.iter().enumerate()
	{
		for (x, c) in line.iter().enumerate()
		{
			// X
			if c != &'X' {continue}
			// Else it is X

			for direction in DIRECTIONS
			{
				num_found += check_direction(
					input_grid, x, y, direction, &chars
				);
			}
		}
	}

	
	num_found
}

#[allow(clippy::manual_map)]
fn char_at(input_grid: &[Vec<char>], from_x: usize, from_y: usize, direction: (i32, i32)) -> Option<char>
{
	match is_safe(from_x, from_y, direction.0, direction.1, input_grid[0].len()-1, input_grid.len()-1)
	{
		None => None,
		Some(location) => Some( input_grid[location.1][location.0] )
	}
}

fn part_two(input_grid: &[Vec<char>]) -> i32
{
	let mut num_found = 0;
	
	for ( y, line ) in input_grid.iter().enumerate()
	{
		for (x, c) in line.iter().enumerate()
		{
			// Find an A
			if c != &'A' { continue }

			match
			(
				char_at(input_grid, x, y, (-1, -1)),
				char_at(input_grid, x, y, ( 1,  1)),
				char_at(input_grid, x, y, (-1,  1)),
				char_at(input_grid, x, y, (1 , -1)),
			)
			{
				(Some('S'), Some('M'), Some('S'), Some('M')) |
				(Some('S'), Some('M'), Some('M'), Some('S')) |
				(Some('M'), Some('S'), Some('S'), Some('M')) |
				(Some('M'), Some('S'), Some('M'), Some('S')) =>
				{
					num_found += 1;
				}
				_ => {},
			}
		}
	}

	num_found
}
