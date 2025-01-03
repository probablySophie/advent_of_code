use std::{cmp::Ordering, ops::Add};

use crate::{Direction, MapFunction, MapLoc, PairFunctions, VecMap};

#[derive(Clone, Copy, Debug)]
pub struct Connection<T>
{
	pub other_point: usize,
	pub distance: T,
}
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub struct ConnectedPoint<T: Add<Output = T>>
{
	pub position: MapLoc,
	pub connections: Vec<Connection<T>>,
	pub score: T,
}
impl<T: Clone + Copy + Add<Output = T> + TryFrom<usize>> ConnectedPoint<T>
{
	#[must_use]
	/// Takes a set of Point locations and the connections between the points
	/// The connections are in the form (point 1 position, point 2 position, distance as usize)
	/// (`Vec<( (usize, usize), (usize, usize), usize )>`)
	/// ### Panics
	/// * If given a set of connections containing positions that there aren't points for
	pub fn new_points(point_locations: Vec<MapLoc>, connections: Vec<(MapLoc, MapLoc, usize)>, default_score: T)
	-> Vec<ConnectedPoint<T>> 
	where <T as std::convert::TryFrom<usize>>::Error: std::fmt::Debug
	{
		let mut points = Vec::new();
		// Make all our points
		for point in point_locations
		{
			points.push(ConnectedPoint { 
				position: point,
				connections: Vec::new(),
				score: default_score,
			});
		}
		
		// Now connect them together :)
		for connection in connections
		{
			let mut point_1 = None;
			let mut point_2 = None;

			for (i, point) in points.iter().enumerate()
			{
				if connection.0 == point.position
				{
					point_1 = Some(i);
				}
				if connection.1 == point.position
				{
					point_2 = Some(i);
				}
				if point_1.is_some()
				&& point_2.is_some()
				{ break }
			}
			let Some(point_1) = point_1
			else { panic!("Given a connection position that does not have a matching Point") };
			let Some(point_2) = point_2
			else { panic!("Given a connection position that does not have a matching Point") };
			
			// Else, add the connection to our points!
			points[point_1].connections.push(
				Connection { other_point: point_2, distance: T::try_from(connection.2).expect(":(") }
			);
			points[point_2].connections.push(
				Connection { other_point: point_1, distance: T::try_from(connection.2).expect(":(") }
			);
		}

		points
	}

	#[must_use]
	/// Make a `Vec<ConnectedPoint>` from a `Vec<Point>`
	pub fn from_points(points: &[Point], default_score: T) -> Vec<ConnectedPoint<T>>
	where <T as std::convert::TryFrom<usize>>::Error: std::fmt::Debug
	{
		let mut connected_points = Vec::new();

		let mut point_map: VecMap<Option<usize>> = vec![ vec![ None ] ];

		// Make the ConnectedPoints
		for point in points
		{
			connected_points.push(ConnectedPoint { 
				position: point.position, 
				connections: Vec::new(),
				score: default_score,
			});

			// Y - height
			if point_map.len() <= point.position.1
			{
				for _ in point_map.len()..=point.position.1
				{
					point_map.push( vec![ None; point_map[0].len() ] );
				}
			}

			// X - width
			if point_map[0].len() <= point.position.0
			{
				for y in 0..point_map.len()
				{
					for _ in point_map[0].len()..=point.position.0
					{
						point_map[y].push(None);
					}
				}
			}
			point_map.set(point.position, Some(connected_points.len() - 1));
		}

		// Make the connections
		for (i, point) in connected_points.iter_mut().enumerate()
		{
			for direction in crate::DIRECTIONS
			{
				let Some(other_point) = points[i].get_from_direction(direction)
				else { continue };

				let Some(wrapped_usize) = point_map.at(other_point)
				else { continue };

				let Some(other_point_i) = wrapped_usize
				else { continue };

				point.connections.push(
					Connection { 
						other_point: other_point_i,
						distance: T::try_from(point.position.distance(other_point)).expect(":(")
				});
			}
		}

		connected_points
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point
{
	pub position: MapLoc,
	
	pub up: Option<MapLoc>,
	pub down: Option<MapLoc>,
	pub left: Option<MapLoc>,
	pub right: Option<MapLoc>,
}
impl Point
{
	#[must_use]
	pub fn new(position: MapLoc) -> Self
	{
		Point { position, up: None, down: None, left: None, right: None}
	}

	pub fn update_if_closer(&mut self, new_location: MapLoc) -> bool
	{
		match 
		(
			self.position.0.cmp(&new_location.0), 
			self.position.1.cmp(&new_location.1)
		)
		{
			// it's to our right
		    (Ordering::Less   , Ordering::Equal) =>
			{
				if new_location.0 <= self.position.0 { return false }
				if self.right.is_some_and( | right | right.0 > new_location.0 )
				|| self.right.is_none()
				{
					self.right = Some(new_location);
				}
			},

		    // It's below us
		    (Ordering::Equal  , Ordering::Less) =>
			{
				if new_location.1 <= self.position.1 { return false }
				if self.down.is_some_and( | down | down.1 > new_location.1 )
				|| self.down.is_none()
				{
					self.down = Some(new_location);
				}
			},
		    // It's above us
		    (Ordering::Equal  , Ordering::Greater) =>
			{
				if new_location.1 >= self.position.1 { return false }
				if self.up.is_some_and( | up | up.1 < new_location.1 )
				|| self.up.is_none()
				{
					self.up = Some(new_location);
				}
			},

		    // It's to our left
		    (Ordering::Greater, Ordering::Equal) =>
			{
				if new_location.0 >= self.position.0 { return false }
				if self.left.is_some_and( | left | left.0 < new_location.0 )
				|| self.left.is_none()
				{
					self.left = Some(new_location);
				}
			},
		    _ => return false,
		}

		true
	}

	#[must_use]
	pub fn to_char(&self) -> char
	{
		match ( 
			self.left.is_some(), 
			self.right.is_some(), 
			self.down.is_some(), 
			self.up.is_some() 
		)
		{	
			(true , true , true , true ) => '╋',
			(false, false, true , true ) => '┃',
			(true , true , false, false) => '━',
			(true , true , false, true ) => '┻',
			(true , true , true , false) => '┳',
			(true , false, true , true ) => '┫',
			(false, true , true , true ) => '┣',
			(false, true , true , false) => '┏',
			(true , false, true , false) => '┓',
			(false, true , false, true ) => '┗',
			(true , false, false, true ) => '┛',
			_ => {'?'},
		}
	}

	#[must_use]
	pub fn get_from_direction(&self, direction: Direction) -> Option<MapLoc>
	{
		match direction
		{
			Direction::Up => self.up,
			Direction::Down => self.down,
			Direction::Left => self.left,
			Direction::Right => self.right,
		}
		
	}

	pub fn clear_direction(&mut self, direction: Direction)
	{
		match direction
		{
			Direction::Up => self.up = None,
			Direction::Down => self.down = None,
			Direction::Left => self.left = None,
			Direction::Right => self.right = None,
		}
	}

	pub fn set_with_direction(&mut self, direction: Direction, new_location: MapLoc)
	{
		match direction
		{
			Direction::Up => self.up = Some(new_location),
			Direction::Down => self.down = Some(new_location),
			Direction::Left => self.left = Some(new_location),
			Direction::Right => self.right = Some(new_location),
		}
	}
}
