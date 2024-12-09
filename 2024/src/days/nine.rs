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
	
	util::print_result("Part 1", time_before.elapsed(), "Defragged checksum", &part_one_result);

	println!();
	
	let time_before = std::time::Instant::now();
	let part_two_result = part_two();
	
	util::print_result("Part 2", time_before.elapsed(), "Redefragged checksum", &part_two_result);	
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
	let input = INPUT;
	// let input = EXAMPLE_INPUT_1;
	
	let mut frags = fancy_frags(input); // Get our frags, but as (file_id, size)	
	let mut empties = get_empties(&frags); // Get the empties, but as (location, size)
	
	let mut processed = 0;
	// Do the moves :)
	
	while processed < frags.len()
	{
		processed += 1;
		let i = frags.len() - processed;
		
		let (file_id, block_size) = frags[i];
		if file_id == -1 { continue } // Skip if empty

		// Find a valid empty
		let mut empty_i: Option<usize> = None;
		let mut empty_size = 0;
		for x in 0..empties.len()
		{
			if empties[x].0 > i
			{
				break;
			}
			// Else
			if empties[x].1 >= block_size
			{
				// We have space!
				empty_i = Some(empties[x].0);
				if empties[x].1 == block_size
				{
					empties.remove(x);
					break;
				}
				// Else there will still be some space
				empties[x].1 -= block_size;
				empty_size = empties[x].1;
				break;
			}
		}
		let Some(empty_i) = empty_i
		else { continue };

		// Do the actual swap
		frags[empty_i] = frags[i];
		frags[i] = (-1, block_size);
		if empty_size != 0
		{
			for empty in &mut empties
			{
				if empty.0 >= empty_i
				{
					empty.0 += 1;
				}
			}
			frags.insert(empty_i+1, (-1, empty_size));
		}
	}

	let checksum = calculate_checksum(&make_single_frags(&frags));
	match input
	{
		INPUT => assert_eq!(checksum, 6_431_472_344_710),
		EXAMPLE_INPUT_1 => assert_eq!(checksum, 2858),
		_ => {},
	}
	checksum
}

fn fancy_frags(input: &str) -> Vec<(ResultType, usize)>
{
	let mut is_file = true;
	let mut file_id: ResultType = 0;
	let mut frags = Vec::new();
	for c in input.chars()
	{
		if ! c.is_numeric() { continue };
		let size: usize = c.to_string().parse().expect("Failed to convert to usize");
		let now = if is_file {
				file_id
			} else {
				-1
			};
		frags.push( ( now , size ) );

		if is_file {file_id += 1};
		is_file = !is_file; // we alternate between free space & file block sizes
	}
	frags
}

fn get_empties(frags: &[(ResultType, usize)]) -> Vec<(usize, usize)>
{
	let mut empties = Vec::new();

	(0..frags.len()).for_each(|i| {
		if frags[i].0 == -1
		{
			empties.push((i, frags[i].1));
		}
	});

	empties
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
