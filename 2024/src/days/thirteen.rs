use std::{ops::Mul, time::{Duration, Instant}};

use util::MapLoc;

#[allow(unused)]
const INPUT: &str = include_str!("../../input/13.txt");
#[allow(unused)]
const EXAMPLE_INPUT_1: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
#[allow(unused)]
const EXAMPLE_INPUT_2: &str = "";

// For if we need to change it up!
type ResultType = i64;

//https://adventofcode.com/2024/day/13
pub fn go(print_results: bool) -> (Duration, Duration, Duration)
{
	if print_results {println!("Day 13");}
	
	let time_before = Instant::now();
	// ~ ~ ~ ~ ~ PRE CALCULATION ~ ~ ~ ~ ~

	// let machines = parse_input(EXAMPLE_INPUT_1);
	let machines = parse_input(INPUT);

	// ~ ~ ~ ~ ~ END OF PRE CALCULATION ~ ~ ~ ~ ~
	let pre_calc_time = time_before.elapsed();

	// Part 1
	TimedRun!(time_before, part_one_result, part_one(&machines), part_one_time);

	if print_results
	{
		util::print_result("Part 1", part_one_time, "The minimum number of tokens is", &part_one_result);
	}

	// Part 2
	TimedRun!(time_before, part_two_result, part_two(&machines), part_two_time);
	
	if print_results
	{
		println!();
		util::print_result("Part 2", part_two_time, "The NEW minimum number of tokens is", &part_two_result);
	}

	// Return how long it took!
	(pre_calc_time, part_one_time, part_two_time)	
}

#[allow(clippy::cast_possible_wrap)]
fn part_one(machines: &[Machine]) -> ResultType
{
	let mut total_cost = 0;
	// The cost of pressing the A & B buttons
	let (a_cost, b_cost): (ResultType, ResultType) = (3, 1);
	let debug = false;

	// Work out the smallest number of tokens required to win as many prizes as possible
	for machine in machines
	{
		match machine.get_prize(Some(100))
		{
			None => {},
			Some(presses) =>
			{
				let cost = (presses[0] * a_cost) + (presses[1] * b_cost);
				assert_eq!(
					presses[0] * (machine.a.0 as ResultType)
					+ presses[1] * (machine.b.0 as ResultType),
					machine.prize.0 as ResultType
				);
				assert_eq!(
					presses[0] * (machine.a.1 as ResultType)
					+ presses[1]  * (machine.b.1 as ResultType),
					machine.prize.1 as ResultType
				);
				if debug { println!("A: {}\tB: {}\tCost: {cost}", presses[0], presses[1]) };
				total_cost += cost;
			},
		}
	}
	assert_eq!(total_cost, 29438);
	total_cost
}

#[allow(clippy::cast_possible_wrap)]
fn part_two(machines: &[Machine]) -> ResultType
{
	let mut total_cost = 0;
	// The cost of pressing the A & B buttons
	let (a_cost, b_cost) = (3, 1);
	let debug = false;

	// Work out the smallest number of tokens required to win as many prizes as possible
	for (i, machine) in machines.iter().enumerate()
	{
		// println!("{} {i}", machines.len());
		let mut machine = machine.clone();
		machine.prize.0 += 10_000_000_000_000;
		machine.prize.1 += 10_000_000_000_000;
		match machine.get_prize(None)
		{
			None => {},
			Some(presses) =>
			{
				let cost = (presses[0] * a_cost) + (presses[1] * b_cost);
				assert_eq!(
					presses[0] * (machine.a.0 as ResultType)
					+ presses[1] * (machine.b.0 as ResultType),
					machine.prize.0 as ResultType
				);
				assert_eq!(
					presses[0] * (machine.a.1 as ResultType)
					+ presses[1]  * (machine.b.1 as ResultType),
					machine.prize.1 as ResultType
				);
				if debug { println!("A: {}\tB: {}\tCost: {cost}", presses[0], presses[1]) };
				total_cost += cost;
			},
		}
	}

	total_cost
}


#[derive(Default, Clone)]
struct Machine
{
	a: MapLoc,
	b: MapLoc,
	prize: MapLoc,
}
impl Machine
{
	#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::cast_precision_loss)]
	// Returns the minimum cost of getting a prize, and the number of button presses required
	pub fn get_prize(&self, max_presses: Option<usize>) -> Option<[ResultType; 2]>
	{		
		// Check if we can get there perfectly
		if let Some(result) = try_simple(self)
		{
			println!("It's perfect!");
			return Some(result)
		}		

		let (x1, y1) = (0., 0.);
		let m1 = m(self.b.0 as f64, self.b.1 as f64);
		let c1 = c(x1, y1, m1);

		let (x2, y2) = (self.prize.0 as f64, self.prize.1 as f64);
		let m2 = m(self.a.0 as f64, self.a.1 as f64);
		let c2 = c(x2, y2, m2);

		let cross_x = (c1 - c2)/(m2 - m1);
		let cross_y = m1*cross_x + c1;

		if cross_x < 0. || cross_y < 0.
		{
			return None
		}

		let b_presses = ((cross_y)/(self.b.1 as f64)).round() as ResultType;
		let a_presses = ((self.prize.0 as f64 - cross_x)/(self.a.0 as f64)).round() as ResultType;
		if a_presses < 0 || b_presses < 0 { return None }

		if (self.a.0 as ResultType) * a_presses + (self.b.0 as ResultType) * b_presses == (self.prize.0 as ResultType)
		&& (self.a.1 as ResultType) * a_presses + (self.b.1 as ResultType) * b_presses == (self.prize.1 as ResultType)
		{
			return Some([a_presses, b_presses]);
		}
		None
	}
}

fn m(x: f64, y: f64) -> f64
{
	y / x
}
fn c(x: f64, y: f64, m: f64) -> f64
{
	y - (m*x)
}

fn get_max_b(machine: &Machine, max_presses: Option<usize>) -> usize
{
	let div_x = machine.prize.0 / machine.b.0;
	let div_y = machine.prize.1 / machine.b.1;
	// Work out the maximum number of B presses we can have
	let max_b = if div_x > div_y
		&& machine.prize.1.checked_sub(machine.b.1 * div_x).is_some()
		{ div_x }
		else if div_y > div_x
		&& machine.prize.0.checked_sub(machine.b.0 * div_y).is_some()
		{ div_y }
		else
		{ div_x.min(div_y) };
	// Clamp if we're clamping
	if max_presses.is_some_and( |max| max < max_b )
	{
		max_presses.unwrap()
	}
	else
	{
		max_b
	}
}

fn attempt_machine(machine: &Machine, max_presses: Option<usize>, i: usize) -> Option<[ResultType; 2]>
{
	let x_rem = machine.prize.0 - ( machine.b.0 * i );
	let y_rem = machine.prize.1 - ( machine.b.1 * i );

	// Button A has to go into both X & Y the same number of times
	if x_rem / machine.a.0 != y_rem / machine.a.1
	{ return None }
	
	let times = x_rem / machine.a.0;
	if times == 0 { return None }

	if max_presses.is_some_and( |m| m < times )
	{ return None }

	if machine.prize.0 == (machine.a.0 * times) + (machine.b.0 * i)
	&& machine.prize.1 == (machine.a.1 * times) + (machine.b.1 * i)
	{
		return Some([
			// A
			(x_rem / machine.a.0).try_into().unwrap(),
			// B
			(i).try_into().unwrap(),
		])
	}
	None
}


#[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
fn try_simple(machine: &Machine) -> Option<[ResultType; 2]>
{
	let b_times = machine.prize.0 / machine.b.0;
	let remainder = machine.prize.0 % machine.b.0;
	let a_times = remainder / machine.a.0;

	if machine.prize.0 == (machine.b.0 * b_times) + (machine.a.0 * a_times)
	&& machine.prize.1 == (machine.b.1 * b_times) + (machine.a.1 * a_times)
	{
		return Some([
			b_times.try_into().unwrap(),
			a_times.try_into().unwrap()
		])
	}
	// Else
	None
}


fn parse_input(input: &str) -> Vec<Machine>
{
	let mut machines = Vec::new();

	let mut new_machine =Machine::default();
	for line in input.lines()
	{
		let splits: Vec<&str> = line.split_whitespace().collect();
		if line.is_empty()
		{
			machines.push(new_machine);
			new_machine = Machine::default();
		}
		else if line.starts_with("Button A")
		{
			new_machine.a = (
				splits[2].replace("X+", "").replace(',', "").parse()
					.expect("Failed to convert to usize"),
				splits[3].replace("Y+", "").replace(',', "").parse()
					.expect("Failed to convert to usize"),
			);
		}
		else if line.starts_with("Button B")
		{
			new_machine.b = (
				splits[2].replace("X+", "").replace(',', "").parse()
					.expect("Failed to convert to usize"),
				splits[3].replace("Y+", "").replace(',', "").parse()
					.expect("Failed to convert to usize"),
			);
		}
		else if line.starts_with("Prize")
		{
			new_machine.prize = (
				splits[1].replace("X=", "").replace(',', "").parse()
					.expect("Failed to convert to usize"),
				splits[2].replace("Y=", "").replace(',', "").parse()
					.expect("Failed to convert to usize"),
			);
		}
		else
		{
			panic!("Unparsable line: {line}");
		}
	}
	if new_machine.a != (0, 0)
	&& new_machine.b != (0, 0)
	&& new_machine.prize    != (0, 0)
	{
		machines.push(new_machine);
	}
	machines
}
