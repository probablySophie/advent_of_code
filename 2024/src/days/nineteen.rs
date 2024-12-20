use std::time::{Duration, Instant};

use util::{MapLoc, VecMap};

#[allow(unused)]
const INPUT: &str = include_str!("../../input/19.txt");
#[allow(unused)]
const EXAMPLE_INPUT_1: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
#[allow(unused)]
const EXAMPLE_INPUT_2: &str = "";

// For if we need to change it up!
type ResultType = i64;

//https://adventofcode.com/2024/day/19
pub fn go(print_results: bool) -> (Duration, Duration, Duration)
{
	if print_results {println!("Day 19");}
	
	let time_before = Instant::now();
	// ~ ~ ~ ~ ~ PRE CALCULATION ~ ~ ~ ~ ~

	let (towels, designs) = parse_input(INPUT, "Main Input", print_results);
	// let (towels, designs) = parse_input(EXAMPLE_INPUT_1, "Example 1", print_results);

	// ~ ~ ~ ~ ~ END OF PRE CALCULATION ~ ~ ~ ~ ~
	let pre_calc_time = time_before.elapsed();
	if print_results { util::print_precalc(pre_calc_time) };

	// Part 1
	TimedRun!(time_before, part_one_result, part_one(&towels, &designs), part_one_time);

	if print_results
	{
		util::print_result("Part 1", part_one_time, "The number of possible designs is", &part_one_result);
	}

	// Part 2
	TimedRun!(time_before, part_two_result, part_two(&towels, &designs), part_two_time);
	
	if print_results
	{
		println!();
		util::print_result("Part 2", part_two_time, "The number of total possible arrangements is", &part_two_result);
	}

	// Return how long it took!
	(pre_calc_time, part_one_time, part_two_time)	
}

/// Work out how many of the given designs are possible using the given towels
fn part_one(towels: &[&str], designs: &[&str]) -> ResultType
{
	let mut possible_designs = 0;

	for design in designs
	{
		// println!("{design}");
		let mut towel_sets: Vec<String> = towels
				.iter()
				.filter_map(
				|t|
				{
					if design.starts_with(t)
					{
						return Some((*t).to_string())
					}
					// Else
					None
				})
				.collect();
		let mut done = false;

		while let Some(set) = towel_sets.pop()
		{			
			let trimmed_design = &design[set.len()..];

			if trimmed_design.is_empty()
			{
				done = true;
				break
			}
			// Else
			for towel in towels
			{
				if trimmed_design.starts_with(towel)
				{
					towel_sets.push( set.clone() + towel );
				}
			}
			// println!();
		}
		if done
		{
			// println!("\t\tGood");
			possible_designs += 1;
		}
	}

	possible_designs
}

type MatchTreeInsides<'a> = (&'a str, Option<Vec<MapLoc>>, ResultType);
type MatchTree<'a> = VecMap<MatchTreeInsides<'a>>;

fn part_two(towels: &[&str], designs: &[&str]) -> ResultType
{	
	let mut total_possible_designs = 0;
	for design in designs
	{
		let design = (*design).to_string();
		let mut match_tree: MatchTree = vec![ Vec::new(); design.len() ];

		let mut matched_towels = Vec::new();
		for (i, towel) in towels.iter().enumerate()
		{
			if design.starts_with(towel)
			{
				match_tree[towel.len()-1].push((towel, None, 1));
			}
			// INFO: Doing this chops about 300ms off of the runtime
			if design.contains(towel)
			{
				matched_towels.push(i);
			}
		}
		let design_len = design.len();
		
		let mut y = 0;
		while y < match_tree.len()
		{
			for x in 0..match_tree[y].len()
			{
				let count = match_tree[y][x].2;
				let pre_matched = build_match(&match_tree, y, x);
				if pre_matched.len() >= design_len
				{
					continue
				}
				let trimmed = &design[pre_matched.len()..];
				'towel_loop: for towel_i in &matched_towels
				{
					let towel = towels[*towel_i];
					if trimmed.starts_with(towel)
					{
						let towel_len = towel.len();
						let y2 = y + towel_len;
						if match_tree[y2].is_empty()
						{
							match_tree[y2].push((towel, Some(vec![(x, y)]), count));
							continue 'towel_loop
						}
						// Else
						for x2 in 0..match_tree[y2].len()
						{
							if match_tree[y2][x2].0 == towel
							{
								match_tree[y2][x2].1.as_mut().unwrap().push((x, y));
								match_tree[y2][x2].2 += count;
								continue 'towel_loop;
							}
						}
						// Else else
						match_tree[y2].push((towel, Some(vec![(x, y)]), count));
					}
				}
			}
			y += 1;
		}
		if match_tree.len() == design.len()
		{
			let mut new_designs = 0;
			for i in 0..match_tree[match_tree.len()-1].len()
			{
				new_designs += match_tree[match_tree.len()-1][i].2;
			}
			total_possible_designs += new_designs;
		}
	}	
	assert_eq!(total_possible_designs, 636_483_903_099_279);
	total_possible_designs
}

fn build_match(match_tree: &MatchTree, y: usize, x: usize) -> String
{
	let mut match_str = String::new();
	let mut x = x;
	let mut y = y;

	while let Some(previous_vec) = &match_tree[y][x].1
	{
		match_str = match_tree[y][x].0.to_string() + &match_str;

		(x, y) = previous_vec[0];
	}

	match_tree[y][x].0.to_string() + &match_str
}

fn parse_input<'a>(input: &'a str, name: &str, print_results: bool) -> (Vec<&'a str>, Vec<&'a str>)
{
	if print_results { println!("Parsing: {name}") };
	
	let mut designs = Vec::new();

	let mut lines = input.lines();

	let towels = lines.next().unwrap().split(',')
		.map( str::trim )
		.collect();

	for line in lines
	{
		if line.is_empty() { continue }
		// Else
		designs.push( line.trim() );
	}

	(towels, designs)
}
