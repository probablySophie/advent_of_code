use super::get_best_path;
#[cfg(test)]
use super::{binary_insert, Node, NodeMap};
#[cfg(test)]
use crate::{MapLoc, MapFunction};
#[cfg(test)]
use colored::Colorize;

#[cfg(test)]
fn make_new_node(position: MapLoc, g: usize, h: usize, nodes: &mut Vec<Node<usize>>, node_map: &mut NodeMap, open_set: &mut Vec<MapLoc>)
{
	node_map[position.1][position.0] = Some(nodes.len());
	nodes.push(
		Node {position, g_score: g, h_score: h, previous: 0, open: true}
	);
	open_set.push(position);
}

#[cfg(test)]
fn binary_insert_test(test_node: Node<usize>, expected_pos: usize)
{
	let mut node_map: NodeMap = vec![ vec![ None; 10 ]; 10 ];
	let mut nodes = Vec::new();
	let mut open_set = Vec::new();
	
		make_new_node((0, 0), 0, 5 , &mut nodes, &mut node_map, &mut open_set);
		make_new_node((0, 1), 0, 8 , &mut nodes, &mut node_map, &mut open_set);
		make_new_node((0, 2), 0, 12, &mut nodes, &mut node_map, &mut open_set);
		make_new_node((0, 3), 0, 14, &mut nodes, &mut node_map, &mut open_set);
		make_new_node((0, 4), 0, 20, &mut nodes, &mut node_map, &mut open_set);
	
	for item in &open_set
	{
		let i = node_map.at(*item).unwrap().unwrap();
		println!("{:2?} {} {}",
			item,
			i,
			nodes[i].f(),
		);
	}
	
	let insert_i = nodes.len();
	nodes.push(
		test_node
	);
	node_map.set(nodes[insert_i].position, Some(insert_i));
	binary_insert(&mut open_set, &nodes, &node_map, insert_i);

	println!();
	let mut inserted_pos = 0;
	for (j, item) in open_set.iter().enumerate()
	{
		let i = node_map.at(*item).unwrap().unwrap();

		let f = if i == insert_i
		{
			inserted_pos = j;
			nodes[i].f().to_string().bold()
		}
		else
		{
			nodes[i].f().to_string().normal()
		};
		
		println!("{:2?} {} {}",
			item,
			i,
			if i == insert_i {f.to_string().bold()} else { f.to_string().normal() },
		);
	}
	println!();

	assert_eq!(inserted_pos, expected_pos);
}

#[test]
fn binary_insert_test_1()
{
	binary_insert_test(
		Node { position: (1, 0), g_score: 0, h_score: 9 , previous: 0, open: true},
		2
	);
}
#[test]
fn binary_insert_test_2()
{
	binary_insert_test(
		Node { position: (1, 0), g_score: 0, h_score: 13 , previous: 0, open: true},
		3
	);
}

#[test]
fn weird_keypad_issue() // from advent of code 2024 day 21 part 1
{
	let map = vec![
		vec![ '7', '8', '9'],
		vec![ '4', '5', '6'],
		vec![ '1', '2', '3'],
		vec![ ' ', '0', 'A']
	];

	let (distance, path) = get_best_path(&map, &[' '], (2, 0), (2, 3))
		.expect("Yeah, that's bad");

	println!("{distance}");
	assert_eq!(distance, 3); // It's literally 3 steps
}
