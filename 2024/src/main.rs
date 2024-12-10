use colored::Colorize;
use std::{fmt::Debug, time::Duration};

mod days;

#[macro_use]
extern crate util;

macro_rules! MatchAndTimeTable {
    ( $input:ident, $($matching:tt, $day:ident,)+ ) => {
        match $input.as_str()
        {
        	$($matching => {let _ = days::$day::go(true);})+,
        	"times" => {
        		println!("Getting times, this will take a moment or two...");
        		let time_sets = vec![
	        		$(
	        			($matching, days::$day::go(false)),
	        		)+
        		];
        		println!();
        		let mut table_widths = [4, 8, 6, 6];
        		for time_set in &time_sets
        		{
        			let l1 = time_set.0.chars().count();
        			let l2 = format!("{:.2?}", time_set.1.0).chars().count();
        			let l3 = format!("{:.2?}", time_set.1.1).chars().count();
        			let l4 = format!("{:.2?}", time_set.1.2).chars().count();

        			if l1 > table_widths[0] { table_widths[0] = l1 + 3 };
        			if l2 > table_widths[1] { table_widths[1] = l2 + 3 };
        			if l3 > table_widths[2] { table_widths[2] = l3 + 3 };
        			if l4 > table_widths[3] { table_widths[3] = l4 + 3 };
        		}
        		println!("| {} | {} | {} | {} |",
        			String::from("Day")      + &(" ".repeat(table_widths[0] - 3)),
        			String::from("Pre-Calc") + &(" ".repeat(table_widths[1] - 8)),
        			String::from("Part 1")   + &(" ".repeat(table_widths[2] - 6)),
        			String::from("Part 2")   + &(" ".repeat(table_widths[3] - 6))
        		);
        		println!("| :{}: | :{}: | :{}: | :{}: |",
        			"-".repeat(table_widths[0]-2),
        			"-".repeat(table_widths[1]-2),
        			"-".repeat(table_widths[2]-2),
        			"-".repeat(table_widths[3]-2),
        		);
        		for time_set in time_sets
        		{
        			let s1 = time_set.0.to_owned();
        			let s2 = format_and_mark(time_set.1.0);
        			let s3 = format_and_mark(time_set.1.1);
        			let s4 = format_and_mark(time_set.1.2);
        			println!("| {} | {} | {} | {} |", 
        				s1.clone() + &(" ".repeat(table_widths[0] - s1.chars().count())), 
        				s2.clone() + &(" ".repeat(table_widths[1] - s2.chars().count())), 
        				s3.clone() + &(" ".repeat(table_widths[2] - s3.chars().count())), 
        				s4.clone() + &(" ".repeat(table_widths[3] - s4.chars().count()))
        			);
        			// TODO: Highlight any numbers above 200 ms as needing improvement
        			// TODO: Put backticks around it `NUM`
        		}
        	},
        	_ => println!("{}: {}", "Unrecognised input".yellow(), $input.yellow()),
        }
    };
}

const WARNING_LEVEL: Duration = Duration::new(0, 200_000_000);
fn format_and_mark(duration: Duration) -> String
{
	let time_str = format!("{duration:.2?}");
	if duration > WARNING_LEVEL {
		return String::from("'") + &time_str + "'"
	}
	// Else
	time_str
}

fn main()
{
	let args: Vec<String> = std::env::args().collect();
	let input: &mut String = &mut String::new();
	
	println!("Sophie's Advent of Code 2024!\n{}\n",
		"https://adventofcode.com/2024".underline());

	// Were we given a day to run?
	if args.len() > 1
	{
		// yes
		input.clone_from(&args[1]);
	}
	else
	{
		// no
		println!("{}", "Please select a day! [1-25]".yellow());
		println!("Or 'times' for all of the day's times!");
		util::read_line_into(input);
	}

	MatchAndTimeTable!(
		input,
		"e", empty,
		"1", one,
		"2", two,
		"3", three,
		"4", four,
		"5", five,
		"6", six,
		"7", seven,
		"8", eight,
		"9", nine,
		// "10", ten,
		// "11", eleven,
		// "12", twelve,
		// "13", thirteen,
		// "14", fourteen,
		// "15", fifteen,
		// "16", sixteen,
		// "17", seventeen,
		// "18", eighteen,
		// "19", nineteen,
		// "20", twenty,
		// "21", twenty_one,
		// "22", twenty_two,
		// "23", twenty_three,
		// "24", twenty_four,
		// "25", twenty_five,
	);
}
