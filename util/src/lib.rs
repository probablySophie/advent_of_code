use colored::Colorize;

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
pub fn str_lines_i32s(str: &str) -> Result<Vec<Vec<i32>>, String>
{
	let mut line_numbers: Vec<Vec<i32>> = Vec::new();

	for (line_i, line) in str.lines().enumerate()
	{
		// Skip empty lines
		if line.trim().is_empty() { continue }
		let mut new_line = Vec::new();

		for (item_i, item) in line.split_whitespace().enumerate()
		{
			match item.parse::<i32>()
			{
				Ok(num) => new_line.push(num),
				Err(error) => return Err(
					String::from("Error with line: ") + &line_i.to_string()
					+ " item: " + &item_i.to_string()
					+ "\n" + &error.to_string().red()
				),
			}
		}

		line_numbers.push(new_line);
	}

	Ok(line_numbers)
}

pub type VecMap<T> = Vec<Vec<T>>;
pub type MapLoc = (usize, usize);

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
/// Print a `util::CharMap` nicely onto the screen :)
pub fn print_map<T>(map: &Vec<Vec<T>>)
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

pub trait MapFunction<T> {
	fn get_new_location(&self, start_location: MapLoc, change: (i32, i32)) -> Option<MapLoc>;
	fn step(&self, start_location: MapLoc, direction: Direction) -> Option<MapLoc>;
	fn at(&self, location: MapLoc) -> Option<T>;
	fn set(&mut self, location: MapLoc, c: T) -> bool;
	/// Returns the FIRST occurance of a given item
	fn find(&self, item: T) -> Option<MapLoc>;
}
#[allow(clippy::cast_sign_loss, clippy::manual_let_else)]
impl<T: Copy + PartialEq> MapFunction<T> for VecMap<T>
{
	/// Get a new location relative to the previous one ( can be more than 1 )
	/// Returns `None` if out of bounds on either axis
	fn get_new_location(&self, start_location: MapLoc, change: (i32, i32)) -> Option<MapLoc>
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
	fn step(&self, from: MapLoc, direction: Direction) -> Option<MapLoc>
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

	/// Get the `char` at a given `MapLoc` location
	fn at(&self, location: MapLoc) -> Option<T>
	{
		if location.0 >= self[0].len()
		|| location.1 >= self.len()
		{
			return None
		}
		
		Some(self[location.1][location.0])
	}

	fn set(&mut self, location: MapLoc, item: T) -> bool
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
	fn find(&self, item: T) -> Option<MapLoc>
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
}

/// The direction the guard is currently facing
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction
{
	Up,
	Down,
	Left,
	Right
}
impl Direction
{
	#[must_use] 
	pub fn turn_right(self) -> Self
	{
		match self
		{
		    Direction::Up => Direction::Right,
		    Direction::Down => Direction::Left,
		    Direction::Left => Direction::Up,
		    Direction::Right => Direction::Down,
		}
	}
	#[must_use]
	pub fn to_char(self) -> char
	{
		match self
		{
		    Direction::Up => '^',
		    Direction::Down => 'v',
		    Direction::Left => '<',
		    Direction::Right => '>',
		}	
	}
	#[must_use]
	pub fn opposite(self) -> Self
	{
		match self
		{
		    Direction::Up => Direction::Down,
		    Direction::Down => Direction::Up,
		    Direction::Left => Direction::Right,
		    Direction::Right => Direction::Left,
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
