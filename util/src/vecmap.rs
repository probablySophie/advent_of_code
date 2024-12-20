use std::fmt::Display;

use crate::{Direction, Pair};

pub type VecMap<T> = Vec<Vec<T>>;


#[must_use]
/// Reads a set of input lines as an `&str` into a `Vec<Vec<char>>`
/// e.g.
/// `...0` →  `[ ['.', '.', '.', '0'], `
/// `.1..` →  `  ['.', '1', '.', '.'],`
/// `..3.` →  `  ['.', '.', '3', '.'] ]`
pub fn read_char_map(input: &str) -> VecMap<char>
{
	let mut vec = Vec::new();
	for line in input.lines()
	{
		if line.is_empty() { continue }
		vec.push( line.chars().collect::<Vec<char>>() );
	}

	vec
}
/// Print a `util::VecMap` nicely onto the screen :)
pub fn print_map<T>(map: &VecMap<T>)
where T: std::fmt::Display
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

pub trait MapFunction<T>
{
	fn get_new_location(&self, start_location: Pair<usize>, change: (i32, i32)) -> Option<Pair<usize>>;
	fn step(&self, start_location: Pair<usize>, direction: Direction) -> Option<Pair<usize>>;
	fn at(&self, location: Pair<usize>) -> Option<T>;
	fn set(&mut self, location: Pair<usize>, c: T) -> bool;
	/// Returns the FIRST occurance of a given item
	fn find(&self, item: T) -> Option<Pair<usize>>;
	fn new_same_size<T2: std::clone::Clone>(&self, default: T2) -> VecMap<T2>;
	
	fn print(&self)
	where 
		T: Display, 
		for<'a> &'a Self: IntoIterator + Iterator,
		for<'a> <&'a Self as IntoIterator>::Item: IntoIterator + Iterator,
		for<'a> <<&'a Self as IntoIterator>::Item as IntoIterator>::Item: Display;
}

#[allow(clippy::cast_sign_loss, clippy::manual_let_else)]
impl<T: Copy + PartialEq> MapFunction<T> for VecMap<T>
{
	fn print(&self)
	where 
		T: Display, 
		for<'a> &'a Self: IntoIterator + Iterator,
		for<'a> <&'a Self as IntoIterator>::Item: IntoIterator + Iterator,
		for<'a> <<&'a Self as IntoIterator>::Item as IntoIterator>::Item: Display
	{
		for line in self
		{
			for item in line
			{
				print!("{item}");
			}
			println!();
		}
	}
	
	/// Get a new location relative to the previous one ( can be more than 1 )
	/// Returns `None` if out of bounds on either axis
	fn get_new_location(&self, start_location: Pair<usize>, change: (i32, i32)) -> Option<Pair<usize>>
	{
		let x = if let (true,  Some(v), _) |
			(false, _, Some(v)) = (
			change.0 < 0,
			start_location.0.checked_sub( change.0.unsigned_abs() as usize ),	
			start_location.0.checked_add( change.0.unsigned_abs() as usize ),
		)
			{ v }
		else { return None };
		
		let y = match (
			change.1 < 0,
			start_location.1.checked_sub( change.1.unsigned_abs() as usize ),	
			start_location.1.checked_add( change.1.unsigned_abs() as usize ),
		)
		{
			(true,  Some(v), _) |
			(false, _, Some(v)) => { v },
			_ => { return None },
		};

		// Make sure we're in the borders
		if x > self[0].len() { return None }
		if y > self   .len() { return None }

		Some((x, y))
	}

	/// Take one step from a given location in a given `Direction`
	/// Returns None if out of bounds on either axis
	fn step(&self, from: Pair<usize>, direction: Direction) -> Option<Pair<usize>>
	{
		let mut x = from.0;
		let mut y = from.1;

		// Make the move and check out of bounds top & left
		match direction
		{
		    Direction::Up    => y = from.1.checked_sub(1)?,
		    Direction::Down  => y = from.1.checked_add(1)?,
		    Direction::Left  => x = from.0.checked_sub(1)?,
		    Direction::Right => x = from.0.checked_add(1)?,
		}
		// Check out of bounds bottom & right
		if y >= self.len()
		|| x >= self[y].len()
		{
			return None
		}
		
		Some((x, y))
	}

	/// Get the `char` at a given `Pair<usize>` location
	fn at(&self, location: Pair<usize>) -> Option<T>
	{
		if location.0 >= self[0].len()
		|| location.1 >= self.len()
		{
			return None
		}
		
		Some(self[location.1][location.0])
	}

	fn set(&mut self, location: Pair<usize>, item: T) -> bool
	{
		if location.0 >= self[0].len()
		|| location.1 >= self.len()
		{
			return false
		}
		self[location.1][location.0] = item;
		true
	}

	#[allow(clippy::needless_range_loop)]
	fn find(&self, item: T) -> Option<Pair<usize>>
	{
		for y in 0..self.len()
		{
			for x in 0..self[y].len()
			{
				if self[y][x] == item
				{
					return Some((x, y))
				}
			}
		}
		None
    }

	fn new_same_size<T2: std::clone::Clone>(&self, default: T2) -> VecMap<T2>
	{
		vec![ vec![ default; self[0].len() ]; self.len() ]
	}
}
