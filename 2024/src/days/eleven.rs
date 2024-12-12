use std::time::{Duration, Instant};

#[allow(unused)]
const INPUT: &str = include_str!("../../input/11.txt");
#[allow(unused)]
const EXAMPLE_INPUT_1: &str = "0 1 10 99 999";
#[allow(unused)]
const EXAMPLE_INPUT_2: &str = "125 17";

// For if we need to change it up!
type ResultType = i64;

//https://adventofcode.com/2024/day/11
pub fn go(print_results: bool) -> (Duration, Duration, Duration)
{
	if print_results {println!("Day 11");}
	
	let time_before = Instant::now();
	
	let stones = INPUT
		.split_whitespace().map(|s| s.parse::<ResultType>().expect("failed to parse"))
		.collect::<Vec<ResultType>>();

	let pre_calc_time = time_before.elapsed();

	// Part 1
	TimedRun!(time_before, part_one_result, part_one(&stones), part_one_time);

	if print_results
	{
		util::print_result("Part 1", part_one_time, "Part 1 description", &part_one_result);
	}

	// Part 2
	TimedRun!(time_before, part_two_result, part_two(&stones), part_two_time);
	
	if print_results
	{
		println!();
		util::print_result("Part 2", part_two_time, "Part 2 description", &part_two_result);
	}

	// Return how long it took!
	(pre_calc_time, part_one_time, part_two_time)	
}

fn part_one(stones: &[ResultType]) -> ResultType
{
	do_the_thing(stones, 25, false)
}

#[derive(Debug, Clone)]
struct Stone
{
	pub count: ResultType,
	pub value: ResultType
}

// Just doing part_one 75 times ends up with so many stones that my laptop crashes :(
// Also around iteration 35 it starts to take over a second per blink
#[allow(clippy::manual_saturating_arithmetic)]
#[allow(clippy::too_many_lines)]
fn part_two(input_stones: &[ResultType]) -> ResultType
{
	let print = false;
	let mut stones: Vec<Stone> = Vec::new();
	// Remove any duplicates - not that there actually are any in my input :/
	'mainStoneLoop: for input_stone in input_stones
	{
		for stone in &mut stones
		{
			if *input_stone == stone.value
			{
				stone.count += 1;
				continue 'mainStoneLoop;
			}
		}
		// If we're here, then it's a fancy new guy
		stones.push(Stone { count: 1, value: *input_stone });
	}
	for i in 0..75
	{
		let time = Instant::now();
		let mut new_stones = Vec::new();

		for old_stone in stones
		{
			let mut stone = old_stone;
			if stone.value == 0
			{
				stone.value = 1;
				new_stones.push(stone);
				continue;
			}

			if let Some(halves) = halves(stone.value)
			{
				// catch for 77 -> 7 & 7
				if halves.0 == halves.1
				{
					stone.value = halves.0;
					stone.count *= 2;
					insert_or_add(&mut new_stones, stone);
					continue
				}

				insert_or_add(&mut new_stones, Stone { count: stone.count, value: halves.0 });
				insert_or_add(&mut new_stones, Stone { count: stone.count, value: halves.1 });
				continue
			}

			if let Some(checked_mul) = stone.value.checked_mul(2024)
			{
				stone.value = checked_mul;
				new_stones.push( stone );
			}
			else
			{
				panic!("Number wrapped");
			}
		}
		stones = new_stones;

		if print {println!("{i} {:.2?}", time.elapsed());}
	}

	let mut count = 0;
	for stone in stones
	{
		count += stone.count;
	}
	count
}

fn insert_or_add(stones: &mut Vec<Stone>, new_stone: Stone)
{
	for i in 0..stones.len()
	{
		if stones[i].value == new_stone.value
		{
			stones[i].count += new_stone.count;
			return
		}
	}
	stones.push( new_stone );
}

/// Take stone inputs & blink a given number of times
fn do_the_thing(stones: &[ResultType], iterations: usize, print_iteration: bool) -> ResultType
{
	let mut stones: Vec<ResultType> = stones.to_vec();

	for i in 0..iterations
	{
		blink(&mut stones);
		if print_iteration { println!("{i} {}", stones.len()) }
	}

	stones.len().try_into().expect("Failed to usize -> ResultType")
}

/// Run a single blink on the given `Vec<ResultType>`
fn blink(stones: &mut Vec<ResultType>)
{
	// Apply the first applicable rule
	for i in 0..stones.len()
	{
	    // If the stone is engraved with the number 0
	    // It is replaced by a stone engraved with the number 1.
		if stones[i] == 0
		{
			stones[i] = 1;
			continue
		}
	    // If the stone is engraved with a number that has an even number of digits
	    // it is replaced by two stones. 

		if let Some(halves) = halves(stones[i])
		{
			stones[i] = halves.0;
			stones.push(halves.1);
			continue
		}

	    // If none of the other rules apply
	    // the stone is replaced by a new stone;
	    // the old stone's number multiplied by 2024 is engraved on the new stone.
 		stones[i] *= 2024;
	}
}

/// Get the two halves from a given number `123456` -> `123` & `456`
/// Returns None if the input value is an odd length
fn halves(value: ResultType) -> Option<(ResultType, ResultType)>
{
	let mut n = value;
	let mut len = 1;
	while n/10 > 0
	{
		len += 1;
		n /= 10;
	}
	if len % 2 != 0
	{
		return None;
	}
		
    // The left half of the digits are engraved on the new left stone
    // The right half of the digits are engraved on the new right stone
	let half = (10 as ResultType).pow( len / 2 );

	let left_stone = value / half;
	let right_stone = value - ( left_stone * half );

	Some((left_stone, right_stone))
}
