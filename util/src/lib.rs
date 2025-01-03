use colored::Colorize;

#[allow(clippy::module_name_repetitions)]


mod pair;		pub use pair::*;
mod vecmap;		pub use vecmap::*;
mod direction;	pub use direction::*;
mod point;      pub use point::*;

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

/// Find whether a given item of type `&T` is in a given iterator of type `&[T]`
/// Returns `true` if found, `false` if not
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

#[macro_export]
/// Takes:  
/// * a `Vec<T>` to insert items into  
/// * an item of type `T` that will be inserted
/// * A closure function that when given an item of type `T` will return the item's score
macro_rules! BinaryInsert {
	($vec:ident, $item:tt, $score:block) =>
	{
		if $vec.is_empty() { $vec.push( $item ) }
		else
		{
    		let mut start = 0;
    		let mut end = $vec.len();
    		let score = $score($item);
    		let mut inserted = false;

			while start < end
			{
				let middle = (start + end) / 2;
				
	    		match ( score.cmp(&$score(middle)), start.abs_diff(end) )
	    		{
	    			(Ordering::Less, 1) =>
	    			{
	    				$vec.insert( start, $item );
	    				inserted = true;
	    				break
	    			},
	    			(Ordering::Greater, 1) =>
	    			{
	    				$vec.insert( end, $item );
	    				inserted = true;
	    				break
	    			}
	    			(Ordering::Equal, _) =>
	    			{
	    				$vec.insert( middle, $item );
	    				inserted = true;
	    				break
	    			},
	    			(Ordering::Less, _) => end = middle,
	    			(Ordering::Greater, _) => start = middle,
	    		}
			}
			if ! inserted
			{
				$vec.insert( start, $item );
			}
    	}
	};
}
