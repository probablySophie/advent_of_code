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
        		let mut table_widths = [4, 8, 6, 6];
        		let mut time_sets: Vec<[String; 4]> = Vec::new();
        		{
	        		$(
	        			let times = days::$day::go(false);
						time_sets.push([
							$matching.to_string(),
							format_and_mark(times.0),
							format_and_mark(times.1),
							format_and_mark(times.2),
						]);
	        		)+
	        	}
        		println!();
        		for time_set in &time_sets
        		{
        			for i in 0..table_widths.len()
        			{
        				let len = time_set[i].chars().count();
        				if table_widths[i] < len
        				{ 
        					table_widths[i] = len
        				}
        			}
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
        			println!("| {} | {} | {} | {} |", 
        				time_set[0].clone() + &(" ".repeat(table_widths[0] - time_set[0].chars().count())), 
        				time_set[1].clone() + &(" ".repeat(table_widths[1] - time_set[1].chars().count())), 
        				time_set[2].clone() + &(" ".repeat(table_widths[2] - time_set[2].chars().count())), 
        				time_set[3].clone() + &(" ".repeat(table_widths[3] - time_set[3].chars().count()))
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
		return String::from("`") + &time_str + "`"
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
		"10", ten,
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
