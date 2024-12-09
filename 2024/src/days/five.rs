use colored::Colorize;

#[allow(unused)]
const INPUT: &str = include_str!("../../input/5.txt");
#[allow(unused)]
const EXAMPLE_INPUT_1: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
#[allow(unused)]
const EXAMPLE_INPUT_2: &str = "";
pub fn go()
{
	println!("Day 5");
	let (page_order_pairs, update_sets) = parse_input(INPUT);

	let time_before = std::time::Instant::now();
	let part_one_result = part_one(&page_order_pairs, &update_sets);
	
	util::print_result("Part 1", time_before.elapsed(), "The correctly ordered sum is", &part_one_result);

	println!();
	
	let time_before = std::time::Instant::now();
	let part_two_result = part_two(&page_order_pairs, &update_sets);
	
	util::print_result("Part 2", time_before.elapsed(), "The incorrectly ordered sum is", &part_two_result);	
}


fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>)
{
	let mut page_pairs: Vec<(i32, i32)> = Vec::new();
	let mut pages: Vec<Vec<i32>> = Vec::new();
	
	for line in input.lines()
	{
		if line.is_empty() { continue }
		
		// Is it a pair?
		if line.contains('|')
		{
			match parse_pair(line)
			{
				Some(num_pair) => 
				{
					page_pairs.push(num_pair);
					continue;
				},
				None => continue,
			}
		}
		// Is it a set of pages?
		if line.contains(',')
		{
			 pages.push(line.split(',').map(|str| {
				str.parse::<i32>().expect("Failed to parse i32")
			}).collect());
		}
	}

	(page_pairs, pages)
}

fn get_index(search_in: &[i32], search_for: i32) -> Option<usize>
{
	for (i, num) in search_in.iter().enumerate()
	{
		if *num == search_for
		{
			return Some(i)
		}
	}
	
	None
}

fn check_rules(page_order_pairs: &[(i32, i32)], update_set: &[i32]) -> bool
{
	// For each rule
	for order_pair in page_order_pairs
	{
		// Are the two numbers we care about even in the update_set?
		let Some(i_1) = get_index(update_set, order_pair.0)
		else { continue };
		let Some(i_2) = get_index(update_set, order_pair.1)
		else { continue };

		// Make sure the numbers are in the right order
		if i_1.gt(&i_2)
		{
			return false; // they aren't :(
		}
	}
	true
}

fn part_one(page_order_pairs: &[(i32, i32)], update_sets: &[Vec<i32>]) -> i32
{
	let mut total_middles = 0;
	
	// For each update in the set of updates
	for update_set in update_sets
	{
		// Skip any that don't meet the rules
		if ! check_rules(page_order_pairs, update_set)
		{
			continue
		}
		
		// If we're here then we should be good
		total_middles += update_set[ update_set.len()/2 ];
	}
	
	total_middles
}

fn part_two(page_order_pairs: &[(i32, i32)], update_sets: &[Vec<i32>]) -> i32
{
	let mut total_middles = 0;
	
	// For each update in the set of updates
	for update_set in update_sets
	{
		// Skip any that DO meet the rules
		if check_rules(page_order_pairs, update_set)
		{
			continue
		}

		// Re-order the numbers
		let mut new_order = Vec::new();
		update_set.clone_into(&mut new_order);

		// This is wonderfully inefficient, but it works!
		// WARN: It will run forever though if given a set of rules & inputs that cannot be made to properly line up
		let mut in_order = false;
		while !in_order
		{
			in_order = true; // we can be hopeful
			for order_pair in page_order_pairs
			{
				let Some(i_1) = get_index(&new_order, order_pair.0)
				else {continue};

				let Some(i_2) = get_index(&new_order, order_pair.1)
				else {continue};

				// If they're both there, then!
				// Skip if they're already good
				if i_1.lt(&i_2) { continue }
				// Else, SWAP EM
				new_order.swap(i_1, i_2);
				in_order = false; // drat
			}
		}
		
		// If we're here then we should be good
		total_middles += new_order[ new_order.len()/2 ];
	}
	
	total_middles
}




fn parse_pair(pair: &str) -> Option<(i32, i32)>
{
	let split: Vec<&str> = pair.split('|').collect();
	if split.len() != 2
	{
		return None;
	}
	// Else it is just two
	let num_one = match split.first()
	{
		None => return None,
		Some(string) =>
		{
			match string.parse::<i32>()
			{
				Err(_) => return None,
				Ok(v) => v,
			}
		}
	};
	let num_two = match split.get(1)
	{
		None => return None,
		Some(string) =>
		{
			match string.parse::<i32>()
			{
				Err(_) => return None,
				Ok(v) => v,
			}
		}
	};
	Some((num_one, num_two))
}
