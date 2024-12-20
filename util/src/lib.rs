#[allow(clippy::module_name_repetitions)]

use std::cmp::Ordering;
use colored::Colorize;

mod pair;		pub use pair::*;
mod vecmap;		pub use vecmap::*;
mod direction;	pub use direction::*;

pub mod pathfinding;

pub type MapLoc = pair::Pair<usize>;

pub fn read_line_into(string: &mut String) -> bool
{
	string.clear();
	match std::io::stdin().read_line(string) {
    	Ok(_)  => {
    		*string = string.trim().to_string();
    		true
    	},// do whatever you want, line is String
    	Err(_) => {false},// handle error, e is IoError
	}
}

#[macro_export]
macro_rules! TimedRun {
    ($before:ident, $result:ident, $func:expr, $after:ident) => {
        let $before = Instant::now();
        let $result = $func;
        let $after = $before.elapsed();
    };
}

pub fn print_precalc(time: std::time::Duration)
{
	println!("\t{} took {:.2?}\n", "Pre-Calculation".bold(), time);
}

/// Prints a day's part's result!
pub fn print_result<T: ToString>(part: &str, time_elapsed: std::time::Duration, description: &str, result: &T)
{
	println!("\t{} took {:.2?} to calculate\n\t{}: {}",
		part.bold(),
		time_elapsed,
		description,
		result.to_string().bold(),
	);
}

/// Takes an `&str`
/// 1. Breaks it into lines `[line, line]`
/// 2. Breaks the lines by whitespace `[[str, str], [str, str]]`
/// 3. Turns the broken result into i32s `[[i32, i32], [i32, i32]]`
/// 4. Returns a `Vec<Vec<i32>>`
/// # Errors
/// * If an item on a line cannot be converted to an i32
pub fn str_lines<T: std::str::FromStr>(str: &str) -> Result<Vec<Vec<T>>, String>
{
	let mut line_numbers: Vec<Vec<T>> = Vec::new();

	for line in str.lines()
	{
		// Skip empty lines
		if line.trim().is_empty() { continue }
		let mut new_line = Vec::new();

		for item in line.split_whitespace()
		{
			match item.parse::<T>()
			{
				Ok(num) => new_line.push(num),
				Err(_) => return Err("Error :(".to_string()),
			}
		}

		line_numbers.push(new_line);
	}

	Ok(line_numbers)
}







#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point
{
	pub position: MapLoc,
	pub up: Option<MapLoc>,
	pub down: Option<MapLoc>,
	pub left: Option<MapLoc>,
	pub right: Option<MapLoc>,
}
impl Point
{
	#[must_use]
	pub fn new(position: MapLoc) -> Self
	{
		Point { position, up: None, down: None, left: None, right: None}
	}

	pub fn update_if_closer(&mut self, new_location: MapLoc) -> bool
	{
		match 
		(
			self.position.0.cmp(&new_location.0), 
			self.position.1.cmp(&new_location.1)
		)
		{
			// it's to our right
		    (Ordering::Less   , Ordering::Equal) =>
			{
				if new_location.0 <= self.position.0 { return false }
				if self.right.is_some_and( | right | right.0 > new_location.0 )
				|| self.right.is_none()
				{
					self.right = Some(new_location);
				}
			},

		    // It's below us
		    (Ordering::Equal  , Ordering::Less) =>
			{
				if new_location.1 <= self.position.1 { return false }
				if self.down.is_some_and( | down | down.1 > new_location.1 )
				|| self.down.is_none()
				{
					self.down = Some(new_location);
				}
			},
		    // It's above us
		    (Ordering::Equal  , Ordering::Greater) =>
			{
				if new_location.1 >= self.position.1 { return false }
				if self.up.is_some_and( | up | up.1 < new_location.1 )
				|| self.up.is_none()
				{
					self.up = Some(new_location);
				}
			},

		    // It's to our left
		    (Ordering::Greater, Ordering::Equal) =>
			{
				if new_location.0 >= self.position.0 { return false }
				if self.left.is_some_and( | left | left.0 < new_location.0 )
				|| self.left.is_none()
				{
					self.left = Some(new_location);
				}
			},
		    _ => return false,
		}

		true
	}

	#[must_use]
	pub fn to_char(&self) -> char
	{
		match ( 
			self.left.is_some(), 
			self.right.is_some(), 
			self.down.is_some(), 
			self.up.is_some() 
		)
		{	
			(true , true , true , true ) => '╋',
			(false, false, true , true ) => '┃',
			(true , true , false, false) => '━',
			(true , true , false, true ) => '┻',
			(true , true , true , false) => '┳',
			(true , false, true , true ) => '┫',
			(false, true , true , true ) => '┣',
			(false, true , true , false) => '┏',
			(true , false, true , false) => '┓',
			(false, true , false, true ) => '┗',
			(true , false, false, true ) => '┛',
			_ => {'?'},
		}
	}

	#[must_use]
	pub fn get_from_direction(&self, direction: Direction) -> Option<MapLoc>
	{
		match direction
		{
			Direction::Up => self.up,
			Direction::Down => self.down,
			Direction::Left => self.left,
			Direction::Right => self.right,
		}
		
	}

	pub fn set_with_direction(&mut self, direction: Direction, new_location: MapLoc)
	{
		match direction
		{
			Direction::Up => self.up = Some(new_location),
			Direction::Down => self.down = Some(new_location),
			Direction::Left => self.left = Some(new_location),
			Direction::Right => self.right = Some(new_location),
		}
	}
}

pub fn find_in<T: std::cmp::PartialEq>(iter: &[T], item: &T) -> bool
{
	for i in iter
	{
		if item == i
		{
			return true
		}
	}
	false
}
