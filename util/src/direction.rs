use std::cmp::Ordering;

use crate::Pair;

pub const DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Right, Direction::Down, Direction::Left];

/// The direction the guard is currently facing
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction
{
	Up,
	Down,
	Left,
	Right
}
impl Direction
{
	#[must_use] 
	pub fn turn_right(self) -> Self
	{
		match self
		{
		    Direction::Up => Direction::Right,
		    Direction::Down => Direction::Left,
		    Direction::Left => Direction::Up,
		    Direction::Right => Direction::Down,
		}
	}
	#[must_use]
	pub fn turn_left(self) -> Self
	{
		match self
		{
		    Direction::Up => Direction::Left,
		    Direction::Down => Direction::Right,
		    Direction::Left => Direction::Down,
		    Direction::Right => Direction::Up,
		}
	}
	
	#[must_use]
	pub fn to_char(self) -> char
	{
		match self
		{
		    Direction::Up => '^',
		    Direction::Down => 'v',
		    Direction::Left => '<',
		    Direction::Right => '>',
		}	
	}
	#[must_use]
	pub fn opposite(self) -> Self
	{
		match self
		{
		    Direction::Up => Direction::Down,
		    Direction::Down => Direction::Up,
		    Direction::Left => Direction::Right,
		    Direction::Right => Direction::Left,
		}
	}

	#[must_use]
	pub fn from_positions(pos_1: Pair<usize>, pos_2: Pair<usize>) -> Option<Direction>
	{
		match ( pos_1.0.cmp(&pos_2.0), pos_1.1.cmp(&pos_2.1) )
		{
			(Ordering::Less   , Ordering::Equal  ) => Some(Direction::Right),
			(Ordering::Equal  , Ordering::Less   ) => Some(Direction::Down),
			(Ordering::Equal  , Ordering::Greater) => Some(Direction::Up),
			(Ordering::Greater, Ordering::Equal  ) => Some(Direction::Left),
			_ => None,
		}
	}
}


#[cfg(test)]
fn test_from_positions(direction: Direction, pos_1: Pair<usize>, pos_2: Pair<usize>)
{
	assert_eq!(Some(direction), Direction::from_positions(pos_1, pos_2));
}
#[test] fn from_pos_1() { test_from_positions(Direction::Up, (4, 3), (4, 1)) }
#[test] fn from_pos_2() { test_from_positions(Direction::Left, (4, 3), (1, 3)) }
#[test] fn from_pos_3() { test_from_positions(Direction::Down, (4, 3), (4, 7)) }
#[test] fn from_pos_4() { test_from_positions(Direction::Right, (4, 3), (5, 3)) }
