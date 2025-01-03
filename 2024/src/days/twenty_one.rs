use std::{cmp::Ordering, time::{Duration, Instant}};

use util::{Direction, MapFunction, MapLoc, PairFunctions, VecMap};

#[allow(unused)]
const INPUT: &str = include_str!("../../input/21.txt");
#[allow(unused)]
const EXAMPLE_INPUT_1: &str = "029A";
#[allow(unused)]
const EXAMPLE_INPUT_2: &str = "029A
980A
179A
456A
379A";

// For if we need to change it up!
type ResultType = i32;

//https://adventofcode.com/2024/day/21
pub fn go(print_results: bool) -> (Duration, Duration, Duration)
{
	if print_results {println!("Day 21");}
	
	let time_before = Instant::now();
	// ~ ~ ~ ~ ~ PRE CALCULATION ~ ~ ~ ~ ~
	
	// TODO: Do any pre-calculation here
	// let (to_type, numpad, keypad) = parse_input(EXAMPLE_INPUT_1, "Example 1", print_results);
	// let (to_type, numpad, keypad) = parse_input(EXAMPLE_INPUT_2, "Example 2", print_results);
	let (to_type, numpad, keypad) = parse_input("379A", "Example 2_4", print_results);

	// ~ ~ ~ ~ ~ END OF PRE CALCULATION ~ ~ ~ ~ ~
	let pre_calc_time = time_before.elapsed();
	if print_results { util::print_precalc(pre_calc_time) };

	// Part 1
	TimedRun!(time_before, part_one_result, part_one(&to_type, &numpad, &keypad), part_one_time);

	if print_results
	{
		util::print_result("Part 1", part_one_time, "The sum of complexities is", &part_one_result);
	}

	// Part 2
	TimedRun!(time_before, part_two_result, part_two(), part_two_time);
	
	if print_results
	{
		println!();
		util::print_result("Part 2", part_two_time, "Part 2 description", &part_two_result);
	}

	// Return how long it took!
	(pre_calc_time, part_one_time, part_two_time)	
}

const NUMPAD: [[char; 3]; 4] = [
	[ '7', '8', '9' ],
	[ '4', '5', '6' ],
	[ '1', '2', '3' ],
	[ ' ', '0', 'A' ],
];
const KEYPAD: [[char; 3]; 2] = [
	[' ', '^', 'A'],
	['<', 'v', '>'],		
];

fn part_one(to_type: &VecMap<char>, numpad: &VecMap<char>, keypad: &VecMap<char>)
-> ResultType
{	
	// Robot arms start pointing at the A
	// TODO WARN We need to update the move bit so it NEVER points at a space

	// util::print_map(numpad);
	let mut total_complexity = 0;
	let numpad_a = numpad.find('A').unwrap();
	let keypad_a = keypad.find('A').unwrap();

	// Robot_1 presses the numbers :)
	for line in to_type
	{
		let mut pads = vec![ // Our list of keypads and numpads :)
			Pad {
				position: numpad_a, 
				keys: numpad.clone(), 
				desire: numpad.find(line[0]), 
				pressed: Vec::new() 
			},
			Pad {
				position: keypad_a,
				keys: keypad.clone(),
				desire: None,
				pressed: Vec::new()
			},
			Pad {
				position: keypad_a,
				keys: keypad.clone(),
				desire: None,
				pressed: Vec::new()
			},
			Pad {
				position: keypad_a,
				keys: keypad.clone(),
				desire: None,
				pressed: Vec::new()
			}
		];
		for i in 1..pads.len() - 1
		{
			pads[i].desire = Some( pads[i].new_desire(&pads, i, line) );
			// pads[i].desire = Some(pads[i].new_desire(&pads[i-1].keys, pads[i-1].desire.unwrap(), pads[i-1].position, pads[i+1].position));
		}
		print!("\t"); for ch in line {print!("{ch}")}; println!();

		// While we haven't typed out the line
		while pads[0].pressed != *line
		{
			
		}

		// Our points, score, & stuff
		let number = line.iter().filter( |c| c.is_ascii_digit()
		).collect::<String>().parse::<ResultType>().expect("Well beans...");
		total_complexity += ResultType::try_from( pads[3].pressed.len() ).unwrap() * number;
		
		println!("Length: {}, Number: {number}", pads[3].pressed.len());
		println!("Total Complexity: {total_complexity}\n");
	}
	
	total_complexity
}

struct Pad
{
	pub position: MapLoc,
	pub keys: VecMap<char>,
	pub desire: Option<MapLoc>,
	pub pressed: Vec<char>,
}
impl Pad
{
	pub fn press(pads: &mut [Pad], line: &[char])
	{
		for i in (0..pads.len()).rev()
		{
			
		}
		// And new desires maybe
		for i in 0..pads.len()
		{
			if pads[i].desire.is_some() { continue }
			// Else
			pads[i].desire = Some( pads[i].new_desire(pads, i, line) );
		}
	}
	
	pub fn new_desire(&self, pads: &[Pad], me: usize, chars: &[char] /*previous_keypad: &VecMap<char>, previous_desire: MapLoc, previous_position: MapLoc, next_position: MapLoc*/) -> MapLoc
	{
		// If I'm number 0, then I want to press the next letter
		if me == 0
		{
			return self.keys.find( chars[ self.pressed.len() ] ).unwrap();
		}
		// Else

		let  previous_desire  = pads[me-1].desire.expect("Oh no");
		let previous_position = pads[me-1].position;
		
		// If they're already where they need to be, then we want to go to the 'A' key
		if previous_desire == previous_position
		{
			return self.keys.find('A').unwrap();
		}
		// How they're getting from where they are to where they want
		let (wrapped_left_right, wrapped_up_down) = previous_position.directions_to(previous_desire);

		// Just one direction?
		if wrapped_left_right.is_none() || wrapped_up_down.is_none()
		{
			let direction = wrapped_left_right.unwrap_or( wrapped_up_down.unwrap() );

			// The button I need to press to make them go that direction
			return self.keys.find(direction.to_char()).unwrap();
		}
		// Else: Two directions!?

		// Is one of those directions evil?
		let midpoint = (previous_position.0, previous_desire.1);
		if pads[me-1].keys.at_unchecked(midpoint) == ' '
		{
			// TODO: One of those directions IS evil
		}

		// Am I already ON one of those directional buttons?

		// TODO: Which of those directions is best for me

		// TODO: Am I the last in the queue?
		if me == pads.len() - 1
		{
			//
		}

		// TODO: Which of those directions is best for who's pressing me

		(0, 0)
	}
	
}

fn part_two() -> ResultType
{
	0
}

fn parse_input<'a>(input: &'a str, name: &str, print_results: bool)
-> (/* to_type */VecMap<char>, /*Numpad*/ VecMap<char>, /*Keypad*/ VecMap<char>)
{
	if print_results { println!("Parsing: {name}") };
	
	let to_type = input.lines().map( |line| line.chars().collect() ).collect();
	let numpad = NUMPAD.iter().map(
		|line|
		{
			line.to_vec()
		}
	).collect();
	let keypad = KEYPAD.iter().map(
		|line|
		{
			line.to_vec()
		}
	).collect();
	(to_type, numpad, keypad)
}
