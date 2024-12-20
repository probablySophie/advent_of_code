use std::ops::{Add, Sub};


pub type Pair<T> = (T, T);

#[allow(unused)]
pub trait PairFunctions<T>
{
	/// Get whether a given pair of `T` items fall inbetween two other sets of `T` items
	fn is_between(&self, pos_1: Pair<T>, pos_2: Pair<T>) -> bool;
	/// Get the distance between a given pair of `T` items and another
	fn distance(&self, pair: Pair<T>) -> T;
	/// Get the first item in the tuple
	fn x(&self) -> T;
	/// Get the second item in the tuple
	fn y(&self) -> T;
}

impl<T> PairFunctions<T> for Pair<T>
where T: Copy + PartialOrd + PartialEq + Sub<Output = T> + Add<Output = T> {
	fn x(&self) -> T { self.0 }
	fn y(&self) -> T { self.1 }
	
	fn distance(&self, pair: Pair<T>) -> T
	{
		let a = if self.0 > pair.0 { self.0 - pair.0 }
			else { pair.0 - self.0 };
		let b = if self.1 > pair.1 { self.1 - pair.1 }
			else { pair.1 - self.1 };
		
		a + b
	}
	
	fn is_between(&self, pos_1: Pair<T>, pos_2: Pair<T>) -> bool
	{
		// We're on the same X
		if self.0 == pos_1.0 && self.0 == pos_2.0
		{
			return ( pos_1.1 < self.1 && self.1 < pos_2.1 ) || (pos_1.1 > self.1 && self.1 > pos_2.1)
		}
		// We're on the same Y
		if self.1 == pos_1.1 && self.1 == pos_2.1
		{
			return ( pos_1.0 < self.0 && self.0 < pos_2.0 ) || (pos_1.0 > self.0 && self.0 > pos_2.0)
		}
		false
	}
}
#[test]
fn is_between_1()
{
	let my_loc = (3, 5);
	assert!(  my_loc.is_between((1, 5), (7, 5)));
	assert!(  my_loc.is_between((7, 5), (1, 5)));
	assert!(! my_loc.is_between((7, 5), (4, 5)));

	assert!(  my_loc.is_between((3, 1), (3, 9)));
	assert!(  my_loc.is_between((3, 9), (3, 1)));
	assert!(! my_loc.is_between((3, 1), (3, 2)));
}
