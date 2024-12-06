use colored::Colorize;

mod days;

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

	match input.as_str()
	{
		"1" => days::one::go(),
		"2" => days::two::go(),
		"3" => days::three::go(),
		"4" => days::four::go(),
		"5" => days::five::go(),
		"6" => days::six::go(),
		_ => {
			println!("{}", "Unrecognised input".yellow());
		},
	}
}
