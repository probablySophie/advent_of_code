use crate::{ConnectedPoint, MapLoc};
use std::{cmp::Ordering, ops::Add};

#[derive(Clone)]
struct Node
{
	pub position: MapLoc,
	pub point: usize,
	pub has_distance: bool
}

pub fn get_shortest_distance()
{
	//
}

/// ### Panics
/// * A bunch, actually
pub fn get_shortest_paths<T: std::ops::Add<Output = T> + std::cmp::Ord + Copy>(points: &mut [ConnectedPoint<T>], start: MapLoc, goal: MapLoc) -> Option<(T, Paths)>
{
	let (distance, wrapped_paths) = do_the_thing(points, start, goal, true, true)?;

	let paths = wrapped_paths?;

	Some((distance, paths))
}

/// Faster than `get_shortest_path`, but the path returned may not be the shortest
pub fn get_any_path()
{
	//
}

type Paths = Vec<(MapLoc, Vec<MapLoc>)>;

fn build_node_set<T: Add<Output = T>>(points: &[ConnectedPoint<T>], start: MapLoc, goal: MapLoc) -> (Vec<Node>, Vec<usize>, usize)
{
	let mut open_set = Vec::new();
	let mut nodes = Vec::new();
	let mut goal_node_i = 0;
	for (i, point) in points.iter().enumerate()
	{
		nodes.push(
			Node
			{
				position: point.position,
				point: i,
				has_distance: false,
			}
		);
		// Add the start node
		if point.position == start
		{
			open_set.push(i);
			nodes[i].has_distance = true;
		}
		if point.position == goal
		{
			goal_node_i = i;
		}
	}
	(nodes, open_set, goal_node_i)
}

/// Returns (distance, paths)
/// Paths is a vec: (Position, next positions)
fn do_the_thing<T: std::ops::Add<Output = T> + std::cmp::Ord + Copy>(points: &mut [ConnectedPoint<T>], start: MapLoc, goal: MapLoc, get_paths: bool, get_shortest: bool) -> Option<(T, Option<Paths>)>
{	
	let (mut nodes, mut open_set, goal_node_i) = build_node_set(points, start, goal);
	
	// The nodes[i] usize for the shortest current distance
	// We can't do while let Some() because .remove() doesn't return an Option, it just panics ._.
	while ! open_set.is_empty()
	{
		let shortest_distance_i = open_set.remove(0);
		
		if ! get_shortest // We don't want the shortest path, just A path
		&& nodes[shortest_distance_i].position == goal // That's the goal
		{
			break
		}
		// Else

		let distance = points[shortest_distance_i].score;

		// For each connection
		for connection in &points[nodes[shortest_distance_i].point].connections
		{
			let other_node = connection.other_point;
			let length = connection.distance;
			let mut better = false;

			if nodes[other_node].has_distance
			{
				if points[other_node].score > distance + length
				{
					points[other_node].score = distance + length;
					better = true;
				}
			}
			else
			{
				nodes[other_node].has_distance = true;
				points[other_node].score = distance + connection.distance;
				better = true;
			}

			if better
			{
				crate::BinaryInsert!(
					open_set,
					other_node,
					{|a: usize| {
						points[a].score
					}}
				);
			}
		}		
	}
	// If we don't care about the paths
	if ! get_paths
	{
		if nodes[goal_node_i].has_distance
		{
			return Some((points[goal_node_i].score, None))
		}
		// Else
		return None
	}
	// Else, we do want the paths

	let mut paths: Paths = vec![ ( nodes[goal_node_i].position, Vec::new() ) ];
	
	// Starting at the end
	let mut checking = vec![ goal_node_i ];

	while let Some(i) = checking.pop()
	{
		let my_pos = nodes[i].position;
		if ! nodes[i].has_distance { continue };
		let my_distance = points[i].score;
		
		'linkLoop: for link in &points[i].connections
		{
			let your_pos = nodes[link.other_point].position;
			if ! nodes[i].has_distance { continue }
			let your_distance = points[link.other_point].score;
			
			// If they're closer than we are - then that's a good route!
			// And because we already got the shortest distances...
			//    ...this should also be the shortest paths
			// println!("me: {my_pos:2?}, you: {your_pos:2?} my N: {my_distance} your N: {your_distance}, length: {}", link.length);
			if your_distance + link.distance <= my_distance
			{
				// Is it already there?
				for path in &mut paths
				{
					// Yes
					if path.0 == your_pos
					{
						// Add us to it's next nodes
						path.1.push( my_pos );
						continue 'linkLoop;
					}
				}
				// No, it's not there
				paths.insert(0, (your_pos, vec![ my_pos ]) );
				checking.push(link.other_point);
			}
		}
	}
	if ! nodes[goal_node_i].has_distance
	{
		return None
	}
	// Else
	Some((points[goal_node_i].score, Some(paths)))
}







#[test]
// Specifically this guy:
// https://en.wikipedia.org/wiki/Dijkstra's_algorithm#/media/File:Dijkstra_Animation.gif
fn wikipedia_example()
{
	let positions = vec![
		(0, 0),
		(0, 2),
		(1, 1),
		(1, 2),
		(2, 0),
		(3, 1),
	];

	let connections = vec![
		( (0, 0), (0, 2), 14 ),
		( (0, 0), (1, 1),  2 ),
		( (0, 0), (2, 0),  9 ),

		( (0, 2), (1, 1),  9 ),
		( (0, 2), (1, 2),  7 ),

		( (1, 1), (3, 1), 11 ),
		( (1, 1), (1, 2), 10 ),

		( (3, 1), (2, 0),  6 ),
		( (3, 1), (1, 2), 15 ),
	];
	
	let mut points = ConnectedPoint::new_points(positions, connections, 0);
	let Some((distance, paths)) = do_the_thing(&mut points, (0, 2), (2, 0), true, true)
	else { panic!("Got None when requesting paths") };

	assert_eq!(distance, 20, "The provided distance was {distance:?}, it should be 20");

	for node in &points
	{
		println!("{:2?} {:?}", node.position, node.score);
		for link in &node.connections
		{
			println!("\t{:2?} {}", points[link.other_point].position, link.distance);
		}
	}

	let paths = paths.unwrap();
	for (i, guy) in paths.iter().enumerate()
	{
		println!("{i}: {:2?} {:?}", guy.0, guy.1);
	}
	assert_eq!(paths[0].0, (0, 2)); // The start
	assert_eq!(paths[1].0, (1, 1)); // Point 3
	assert_eq!(paths[2].0, (0, 0)); // Point 6
	assert_eq!(paths[3].0, (2, 0));	// The goal
}
