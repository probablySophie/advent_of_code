use colored::Colorize;

mod days;

fn main()
{
    println!("Sophie's Advent of Code 2024!\n{}\n",
        "https://adventofcode.com/2024".underline());

    let input: &mut String = &mut String::new();
    println!("{}", "Please select a day! [1-25]".yellow());

	util::read_line_into(input);

	match input.as_str()
	{
	    "1" => {
	        days::one::go();
	    },
	    _ => {
	        println!("{}", "Unrecognised input".yellow());
	    },
	}
}
