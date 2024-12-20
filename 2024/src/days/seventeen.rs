use std::{ops::BitXor, time::{Duration, Instant}};

#[allow(unused)]
const INPUT: &str = include_str!("../../input/17.txt");
#[allow(unused)]
const EXAMPLE_INPUT_1: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
#[allow(unused)]
const EXAMPLE_INPUT_2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

// For if we need to change it up!
type ResultType = u32;

//https://adventofcode.com/2024/day/17
pub fn go(print_results: bool) -> (Duration, Duration, Duration)
{
	if print_results {println!("Day 17");}
	
	let time_before = Instant::now();
	// ~ ~ ~ ~ ~ PRE CALCULATION ~ ~ ~ ~ ~

	let computer = parse_input(INPUT);
	// let computer = parse_input(EXAMPLE_INPUT_1);

	// ~ ~ ~ ~ ~ END OF PRE CALCULATION ~ ~ ~ ~ ~
	let pre_calc_time = time_before.elapsed();
	if print_results { util::print_precalc(pre_calc_time) };

	// Part 1
	TimedRun!(time_before, part_one_result, part_one(&computer), part_one_time);

	if print_results
	{
		util::print_result("Part 1", part_one_time, "Computer outputs", &part_one_result);
	}

	// Part 2
	TimedRun!(time_before, part_two_result, part_two(&computer), part_two_time);
	
	if print_results
	{
		println!();
		util::print_result("Part 2", part_two_time, "Required register A value", &part_two_result);
	}

	// Return how long it took!
	(pre_calc_time, part_one_time, part_two_time)	
}

fn part_one(computer: &Computer) -> String
{
	let mut computer = computer.clone();
	while computer.step(false) {}

	computer.output_to_string()
}

fn part_two(computer: &Computer) -> ResultType
{	
	let mut c = computer.clone();

	// To get the program output, we want to go BACKWARDS

	c.a = 0; // A must be 0 because we've escaped the loop
	// B must be such that we just got the required_output
	c.b = reverse_mod(*c.instructions.last().unwrap());
	
	// It has to be pointing to the final instruction at the end of execution
	let mut instruction_pointer = c.instructions.len()-2;

	let instructions = ["adv", "bxl", "bst", "jnz", "bxc", "out", "bvd", "cvd"];
	println!("{:?}", c.instructions);
	
	loop
	{
		let (instruction, operand) = (c.instructions[instruction_pointer], c.instructions[instruction_pointer + 1]);
		println!("{} - A: {:5?} B: {:5?} C: {:5?} Outputs: {:?}", instructions[instruction as usize], c.a, c.b, c.c, c.output);

		match instruction
		{
			0 => c.a = reverse_adv(c.combo_operand(operand, false), c.a), // ADV TODO
			1 => c.b = reverse_xor(operand, c.b),
			2 => c.b = reverse_mod(c.combo_operand(operand, false)), // BST
			3 => {/* Ignore? */}, // JNZ
			4 => c.b = reverse_xor(c.c, c.b), // BXC
			5 => // OUT (combo operand) mod 8
			{
				let i = c.instructions.len() - c.output.len() - 1;
				let required_output = c.instructions[i];
				
				// register_b mod 8 must equal required_output
				assert_eq!( fancy_mod(c.b, 8), required_output, "For the output to be {required_output}, c.b should be {}", reverse_mod(required_output));
				
				c.output.insert(0, required_output);
			},
			6 => c.a = reverse_adv(c.combo_operand(operand, false), c.b), // BDV TODO
			7 => c.a = reverse_adv(c.combo_operand(operand, false), c.c), // CDV TODO
			_ => panic!("??"),
		}

		if instruction_pointer == 0 && c.output == c.instructions
		{
			break // Yay, we've reached the start & outputted the instructions :)
		}
		else if instruction_pointer == 0
		{
			// TODO: Register A must not be 0
			instruction_pointer = c.instructions.len() - 2;
		}
		// Else
		instruction_pointer -= 2;
	}

	assert_eq!(c.instructions, c.output);
	
	c.a
}

fn reverse_xor(a: ResultType, answer: ResultType) -> ResultType
{
	let mut i = 0;
	loop
	{
		if a.bitxor(i) == answer
		{
			return i
		}
		i += 1;
	}
}

fn reverse_mod(answer: ResultType) -> ResultType
{
	let mut i = 0;
	loop
	{
		if fancy_mod(i, 8) == answer
		{
			return i;
		}
		i += 1;
	}
}

fn reverse_adv(a: ResultType, answer: ResultType) -> ResultType
{
	// answer = ? / 2^a
	answer * 2_u32.pow(a)
}

#[derive(Clone)]
struct Computer
{
	pub a: ResultType,
	pub b: ResultType,
	pub c: ResultType,
	pub instruction_pointer: usize,
	pub instructions: Vec<ResultType>,
	pub output: Vec<ResultType>
}
#[allow(clippy::match_same_arms)]
impl Computer
{
	pub fn step(&mut self, print_process: bool) -> bool
	{
		if self.instruction_pointer >= self.instructions.len()
		{
			return false
		}
		let instruction = self.instructions[self.instruction_pointer];
		let operand = self.instructions[self.instruction_pointer + 1];
		if print_process { print!("{instruction} {operand}\t"); }
		self.instruction_pointer += 2;
		// println!("Opcode: {instruction}.  Operand {operand}. A:{} B:{} C:{}", self.register_a, self.register_b, self.register_c); // TEMP
		match instruction
		{
			0 => // Divide A by 2^operand
			{
				self.a = self.adv(self.a, operand, print_process);
				if print_process { print!("A: {} ", self.a); };
			},
			
			1 =>// Bitwise XOR B & operand - storing result in B
			{
				self.b = self.b.bitxor(operand);
				if print_process { print!("B: {}", self.b); };
			},
			
			2 => // combo_operand() % 8 write into b
			{
				// self.register_b = self.combo_operand(operand) % 8;
				self.b = fancy_mod(self.combo_operand(operand, print_process), 8);
				if print_process { print!("B: {}", self.b); };
			},
			
			3 =>
			{
				if self.a != 0 {
				// Else
					self.instruction_pointer = operand as usize;
					if print_process { print!("Pointer: {}", self.instruction_pointer); };
				}
			},
			
			4 => // Bitwise XOR B & C, storing in B (ignore operand)
			{
				self.b = self.b.bitxor(self.c);
				if print_process { print!("B: {}", self.b); };
			},
			
			5 =>
			{
				// self.output.push( self.combo_operand(operand) % 8 );
				self.output.push( fancy_mod(self.combo_operand(operand, print_process), 8) );
				if print_process { print!("Output: {}", self.output.last().unwrap()); };
			},
			
			6 =>
			{
				self.b = self.adv(self.a, operand, print_process);
				if print_process { print!("B: {}", self.b); };
			},
			
			7 =>
			{
				self.c = self.adv(self.a, operand, print_process);
				if print_process { print!("C: {}", self.c); };
			},
			
			_ => panic!("Bad instruction op_code provided"),
		}
		if print_process {println!()};
		true
	}

	#[allow(dead_code)]
	pub fn print_state(&self)
	{
		println!("\tRegister A: {}", self.a);
		println!("\tRegister B: {}", self.b);
		println!("\tRegister C: {}", self.c);
		println!();
		println!("\tProgram: {:?}", self.instructions);
		println!("\tOutput: {}", self.output_to_string());
	}

	pub fn combo_operand(&self, operand: ResultType, print_process: bool) -> ResultType
	{
		if print_process { print!("combo: ") }
		
		match operand
		{
			0..=3 => 
			{
				if print_process { print!("{operand} as literal ") }
				operand
			},
			  4   => 
			{
				if print_process { print!("register A ") }
				self.a
			},
			  5   => 
			{
				if print_process { print!("register B ") }
				self.b
			},
			  6   => 
			{
				if print_process { print!("register C ") }
				self.c
			},
			  7   => 
			{
				panic!("Reserved apparently");
			},
			  _   => 
			{
				panic!("Bad instruction op_code provided");
			},
		}
	}

	pub fn output_to_string(&self) -> String
	{	
		let mut output = String::new();

		for o in &self.output
		{
			output += &(o.to_string() + ",");
		}
		output = output.trim_end_matches(',').to_string();
		output
	}
	fn adv(&self, a: ResultType, b: ResultType, print_process: bool) -> ResultType
	{
		let c = 2_u32.pow(self.combo_operand(b, false));
		if print_process {print!("{a} / {c} = {} ", a / c); }
		a / c
	}
}

fn parse_input(input: &str) -> Computer
{
	let (mut register_a, mut register_b, mut register_c) = (0, 0, 0);
	let mut instructions: Vec<ResultType> = Vec::new();

	for line in input.lines()
	{
		if line.starts_with("Register")
		{
			let to_parse = line.split_whitespace().nth(2).unwrap();
			let parsed = to_parse.parse()
				.unwrap_or_else(|_| panic!("Failed to parse '{to_parse}'") );
			
			if line.find('A').is_some() { register_a = parsed }
			else if line.find('B').is_some() { register_b = parsed }
			else if line.find('C').is_some() { register_c = parsed }
		}
		else if line.starts_with("Program:")
		{
			let instruction_split = line.split_whitespace().nth(1).unwrap();

			instructions = instruction_split
				.split(',').map(
					|str| str.parse::<ResultType>()
						.unwrap_or_else(|_| panic!("Unable to parse '{str}'") )
				).collect::<Vec<ResultType>>();
		}
	}
	Computer
	{
	    a: register_a,
	    b: register_b,
	    c: register_c,
	    instruction_pointer: 0,
	    instructions,
	    output: Vec::new(),
	}
}

fn fancy_mod(a: ResultType, b: ResultType) -> ResultType
{
	((a % b) + b) % b
}

#[allow(unused)]
fn print_instructions(c: &Computer)
{	
	let mut is_instruction = true;
	let mut ignore_instruction = false;
	let mut is_combo = false;

	println!();
	for instruction in &c.instructions
	{
		if is_instruction
		{
			print!("{instruction} ");
			if matches!(instruction, 0 | 2 | 5..=7) { is_combo = true }
			if *instruction == 4 { ignore_instruction = true }
			match instruction
			{
				0 => print!(" A = A/2^X \t\t"),
				1 => print!(" B = B xor X \t\t"),
				2 => print!(" B = combo mod 8 \t"),
				3 => print!(" Jump to \t\t"),
				4 => print!(" B = B xor C "),
				5 => print!(" Output combo mod 8 \t"),
				6 => print!(" B = A/2^X \t\t"),
				7 => print!(" C = A/2^X \t\t"),
				_ => panic!("??"),
			}
		}
		else if ignore_instruction 
		{ 
			ignore_instruction = false; 
			println!(); 
		}	
  		else if is_combo
  		{
  			c.combo_operand(*instruction, true);
  			println!();
  			is_combo = false;
  		}
  		else
  		{
  			println!("Operand {instruction}"); 
  		}
		is_instruction = !is_instruction;
	}
}

/*


*/

#[test]
fn part_one_example_1()
{
	let mut c = parse_input(EXAMPLE_INPUT_1);
	while c.step(false) {}
	assert_eq!(&c.output_to_string(), "4,6,3,5,6,3,5,2,1,0");
}
#[test]
fn part_two_example_1()
{
	let c = parse_input(EXAMPLE_INPUT_2);
	let should_be = 117_440;
	let result = part_two(&c);
	assert_eq!(result, should_be, "Part 2() output was {result}.  It should be {should_be}");
}


#[test] // If register C contains 9, the program 2,6 would set register B to 1.
fn test_1() 
{
	let mut c = Computer { a: 0, b: 0, c: 9, instruction_pointer: 0, instructions: vec![2,6], output: Vec::new() };
	c.step(false);
	assert_eq!(c.b, 1);
}
#[test] // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
fn test_2() 
{
	let mut c = Computer { a: 10, b: 0, c: 0, instruction_pointer: 0, instructions: vec![5,0,5,1,5,4], output: Vec::new() };
	while c.step(false) {};
	assert_eq!(&c.output_to_string(), "0,1,2" );
}
#[test] // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
fn test_3() 
{
	let mut c = Computer { a: 2024, b: 0, c: 0, instruction_pointer: 0, instructions: vec![0,1,5,4,3,0], output: Vec::new() };
	while c.step(false) {}
	assert_eq!(c.a, 0);
	assert_eq!(&c.output_to_string(), "4,2,5,6,7,7,7,7,3,1,0");
}
#[test] // If register B contains 29, the program 1,7 would set register B to 26.
fn test_4() 
{
	let mut c = Computer { a: 0, b: 29, c: 0, instruction_pointer: 0, instructions: vec![1,7], output: Vec::new() };
	while c.step(false) {}
	assert_eq!(c.b, 26);
}
#[test] // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
fn test_5() 
{
	let mut c = Computer { a: 0, b: 2024, c: 43690, instruction_pointer: 0, instructions: vec![4,0], output: Vec::new() };
	while c.step(false) {}
	assert_eq!(c.b, 44354);
}

#[test] fn test_opcode_0()
{
	let mut c = Computer { a: 0, b: 0, c: 0, instruction_pointer: 0, instructions: Vec::new(), output: Vec::new() };

	c.a = 10; c.instructions = vec![0,1];
	while c.step(false) {}
	assert_eq!(c.a, 5);
	
	c.a = 16; c.b = 3; c.instructions = vec![0,5]; c.instruction_pointer = 0;
	while c.step(true) {print!(".")}
	assert_eq!(c.a, 2);
}


#[test] fn part_one_test() 
{
	let mut c = parse_input(INPUT);
	while c.step(false) {}
	assert_eq!(&c.output_to_string(), "1,3,5,1,7,2,5,1,6", "Something has gone VERY wrong");
}
