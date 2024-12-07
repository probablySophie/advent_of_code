use colored::Colorize;

#[allow(unused)]
const INPUT: &str = include_str!("../../input/7.txt");
#[allow(unused)]
const EXAMPLE_INPUT_1: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
#[allow(unused)]
const EXAMPLE_INPUT_2: &str = "";

// https://adventofcode.com/2024/day/7
pub fn go()
{
	println!("Day 7");
	let equations = parse_input(INPUT);
	// let equations = parse_input(EXAMPLE_INPUT_1);

	println!("\t{}\n\tTotal calibration result: {}",
		"Part 1".bold(),
		part_one(&equations)
	);
	
	println!("\t{}\n\tTotal result with new operator: {}",
		"Part 2".bold(),
		part_two(&equations)
	);
}

/// See if the total can be made using the arguments using only `*` and `+`
/// Return the sum of the totals that *can* be made in this way
/// Operators are evaluated **left-to-right**.  Not in usual BIDMAS
fn part_one(equations: &[Equation]) -> i64
{
	do_the_thing(equations, &['*', '+'])
}

/// Same as part 1, but there's now a new operator: `||`
fn part_two(equations: &[Equation]) -> i64
{
	do_the_thing(equations, &['*', '+', '|'])
}

#[allow(clippy::cast_possible_truncation)]
fn do_the_thing(equations: &[Equation], operators: &[char]) -> i64
{
	let mut total_sum = 0;
	'equationLoop: for equation in equations
	{
		let mut totals: Vec<i64> = Vec::new();

		// Calculate all the possible totals
		for num in &equation.arguments
		{
			// Just push the first number
			if totals.is_empty() { totals.push(*num); continue }
			// Else
			let mut new_totals: Vec<i64> = Vec::new();

			while let Some(total) = totals.pop()
			{
			for operator in operators
				{
					match operator
					{
						'*' => new_totals.push( total * num ),
						'+' => new_totals.push( total + num ),
						'|' => new_totals.push(
							total * 10_i64.pow( num.to_string().len() as u32)
							+ num
						),
						_ => {panic!("Given bad operator")},
					}
				}
			}
			totals.clear();
			totals = new_totals;
		}

		// Check if any of the totals are good
		if validate_totals(equation.total, &totals)
		{
			total_sum += equation.total;
			continue 'equationLoop;
		}
	}

	total_sum
}


fn validate_totals(real_total: i64, totals: &[i64]) -> bool
{
	for total in totals
	{
		if *total == real_total
		{
			return true
		}
	}
	return false
}

fn parse_input(input: &str) -> Vec<Equation>
{
	let mut equations = Vec::new();

	for line in input.lines()
	{
		if line.is_empty() { continue }
		// Else
		let Some(colon_i) = line.find(':')
		else {
			println!("No : found in {line}");
			continue
		};
		
		let Ok(total) = line[0..colon_i].parse::<i64>()
		else {
			println!("Failed to convert total to i64");
			continue
		};
		equations.push(Equation
		{
			total,
			arguments: line[ colon_i+1 .. ].split_whitespace().map(
				|v_str| { v_str.parse::<i64>().expect("failed to convert to i64") }
			).collect(),
		});
	}

	equations
}

#[derive(Debug)]
struct Equation
{
	pub total: i64,
	pub arguments: Vec<i64>
}
