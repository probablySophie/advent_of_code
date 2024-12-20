use std::{cmp::Ordering, ops::Add};

use crate::{find_in, MapFunction, MapLoc, PairFunctions, VecMap};

#[path = "../test/pathfinding/a_star_test.rs"] mod test;

type NodeMap = VecMap<Option<usize>>;

#[derive(Clone, Copy)]
struct Node<T>
{
	pub position: MapLoc,
	pub g_score: T,
	pub h_score: T,
	pub previous: usize,
	pub open: bool,
}
impl<T: Add<Output = T> + Copy> Node<T>
{
	pub fn f(&self) -> T
	{
		self.g_score + self.h_score
	}
}

#[must_use]
pub fn get_shortest_distance(map: &VecMap<char>, obstacles: &[char], start: MapLoc, goal: MapLoc) -> Option<usize>
{
	let (distance, _) = do_the_thing(map, obstacles, start, goal, false)?;
	Some(distance)
}

#[must_use]
fn get_best_path()
{
	// TODO
}

#[must_use]
fn get_path_and_distance()
{
	// TODO
}

// TODO: Allow for diagonal movements


// * Maybe this should be turned into a macro instead of a function so that it can take a closure with custom costs for the directions?
// Using a macro might also allow us to use a pattern for the obstacles too!?
fn do_the_thing(map: &VecMap<char>, obstacles: &[char], start: MapLoc, goal: MapLoc, get_path: bool)
	-> Option<(usize, Option<Vec<MapLoc>>)>
{
	let mut open_set: Vec<MapLoc> = vec![ start ];
	let mut closed_set: Vec<MapLoc> = Vec::new();

	let mut nodes: Vec<Node<usize>> = Vec::new();
	let mut node_map: NodeMap = map.new_same_size(None);

	// Add the start as a node!
	nodes.push(
		Node { 
			position: start,
			g_score: 0, 
			h_score: start.distance(goal), 
			previous: 0, 
			open: true  
		} 
	);
	node_map.set(start, Some(0));

	let mut goal_node: Option<usize> = None;

	while ! open_set.is_empty() && goal_node.is_none()
	{
		// TODO: Get the Node with the lowest f() score from the open set
		let i = node_map.at(open_set.remove(0)).unwrap().unwrap();
		let position = nodes[i].position;

		#[cfg(test)]
		println!("Open: {} Closed: {} Total: {} Position: {position:2?}", open_set.len(), closed_set.len(), nodes.len());

		// Is that node the goal?
		if position == goal
		{
			#[cfg(test)] println!("Yay!");
			goal_node = Some(i);
			break
		};

		// Remove us from the open_set
		for (j, item) in open_set.iter().enumerate()
		{
			if item == &position
			{
				#[cfg(test)] println!("Removed");
				open_set.remove(j);
				break
			}
		}
		nodes[i].open = false;
		closed_set.push(position);
		// Else

		// TODO: Update (or create) all neighboring nodes' g_scores & prev's if appropriate
		for direction in crate::DIRECTIONS
		{
			let Some(step_pos) = node_map.step(nodes[i].position, direction)
			else 
			{
				#[cfg(test)]
				println!("Left the map going {direction:?} from {:2?}", nodes[i].position);
				
				continue
			}; // else we left the map

			if find_in(obstacles, &map.at(step_pos).unwrap())
			{
				#[cfg(test)]
				println!("Wall at {step_pos:?}");
				
				continue
			} // Did we hit a wall or something?

			if let Some(i2) = node_map.at(step_pos).unwrap()
			{
				let temp_g = nodes[i].g_score + 1; // TODO: allow for differential costs
				
				// (maybe) Update the neighbor
				if temp_g < nodes[i2].g_score
				{
					nodes[i2].previous = i;
					nodes[i2].g_score = temp_g;
					nodes[i2].h_score = step_pos.distance(goal);
					
					// (maybe) add the neighbor to the open set
					if ! nodes[i2].open
					{
						nodes[i2].open = true;
						binary_insert(&mut open_set, &nodes, &node_map, i2);
					}
				}
			}
			else
			{
				// Create NEW LIFE!!
				node_map.set(step_pos, Some(nodes.len()));
				nodes.push(
					Node {
						position: step_pos,
						g_score: nodes[i].g_score + 1,
						h_score: step_pos.distance(goal),
						previous: i,
						open: true,
					}
				);
				binary_insert(&mut open_set, &nodes, &node_map, nodes.len()-1);
			}
		}
	}
	#[cfg(test)]
	{
		println!("{} {}", open_set.is_empty(), goal_node.is_none());
		debug_print(map, &node_map, &nodes);
	}

	let goal_node = goal_node?; // TODO: Allow for not findable

	let distance = 0;
	if ! get_path
	{
		return Some(( nodes[goal_node].g_score, None ));
	}
	// Else
	// TODO: Getting the path
	Some(( distance, None ))
}

#[allow(dead_code)]
fn debug_print(map: &VecMap<char>, node_map: &NodeMap, nodes: &[Node<usize>])
{
	for (y, line) in map.iter().enumerate()
	{
		for (x, ch) in line.iter().enumerate()
		{
			if let Some(i) = node_map[y][x]
			{
				if nodes[i].open
				{
					print!("O ");
				}
				else
				{
					print!("X ");
				}
			}
			else
			{
				print!("{ch} ");
			}
		}
		println!();
	}
}

fn binary_insert(open_set: &mut Vec<MapLoc>, nodes: &[Node<usize>], node_map: &NodeMap, i: usize)
{
	if open_set.is_empty()
	{ 
		open_set.push(nodes[i].position);
		return
	}
	
	let new_f = nodes[i].f();

	let mut start = 0;
	let mut end = open_set.len()-1;

	while start < end
	{
		let i_check = (start + end) / 2;
		let f2 = nodes[ node_map.at( open_set[i_check] ).unwrap().unwrap() ].f();
		
		#[cfg(test)]
		print!("{start}<->{end} {new_f} is {:?} than/to {f2}", new_f.cmp(&f2));

		match (new_f.cmp(&f2), start.abs_diff(end))
		{
			(Ordering::Less, 1) =>
			{
				open_set.insert(start, nodes[i].position);
				return;
			},
			(Ordering::Greater, 1) =>
			{
				open_set.insert(end, nodes[i].position);
				return;
			}
			(Ordering::Less, _) =>
			{
				end = i_check;
			},
			(Ordering::Equal, _) =>
			{ 
				open_set.insert(i_check, nodes[i].position);
				return;
			},
			(Ordering::Greater, _) => {
				start = i_check;
			},
		}
	}
	open_set.insert(start, nodes[i].position);
}
