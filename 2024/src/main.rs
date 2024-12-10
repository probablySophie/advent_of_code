use colored::Colorize;

mod days;

#[macro_use]
extern crate util;

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
		util::read_line_into(input);
	}

	let _ = match input.as_str()
	{
		"e" => days::empty::go(true),
		"1" => days::one::go(true),
		"2" => days::two::go(true),
		"3" => days::three::go(true),
		"4" => days::four::go(true),
		"5" => days::five::go(true),
		"6" => days::six::go(true),
		"7" => days::seven::go(true),
		"8" => days::eight::go(true),
		"9" => days::nine::go(true),
		_ => {
			panic!("{}", "Unrecognised input".yellow());
		},
	};

	// TODO: Make a table with all of the times!
}
