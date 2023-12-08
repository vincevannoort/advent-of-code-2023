use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use itertools::Itertools;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Hash)]
struct Node {
    name: String,
    left: String,
    right: String,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    LEFT,
    RIGHT,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'L' => Direction::LEFT,
            'R' => Direction::RIGHT,
            _ => panic!("unknown direction"),
        }
    }
}

fn parse_directions(input: &str) -> Vec<Direction> {
    input
        .lines()
        .take(1)
        .collect::<String>()
        .chars()
        .map(Direction::from_char)
        .collect_vec()
}

fn parse_nodes(input: &str) -> HashMap<String, Node> {
    input
        .lines()
        .skip(2)
        .map(|node| {
            let (name, directions) = node.split_once(" = ").unwrap();
            let directions = directions.replace(['(', ')'], "");
            let (left, right) = directions.split_once(", ").unwrap();

            Node {
                name: name.to_string(),
                left: left.to_string(),
                right: right.to_string(),
            }
        })
        .map(|node| (node.name.clone(), node))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let directions: Vec<Direction> = parse_directions(input);
    let nodes: HashMap<String, Node> = parse_nodes(input);

    let mut counter = 0;
    let mut current_node = nodes.get("AAA").unwrap();
    for direction in directions.iter().cycle() {
        if current_node.name == "ZZZ" {
            break;
        }
        current_node = match direction {
            Direction::LEFT => nodes.get(&current_node.left).unwrap(),
            Direction::RIGHT => nodes.get(&current_node.right).unwrap(),
        };
        counter += 1;
    }

    Some(counter)
}

pub fn part_two(input: &str) -> Option<u32> {
    let directions: Vec<Direction> = parse_directions(input);
    let nodes: HashMap<String, Node> = parse_nodes(input);

    let current_nodes: HashMap<String, Node> = nodes
        .clone()
        .into_iter()
        .filter(|(name, _)| name.ends_with('A'))
        .collect();

    let current_nodes: Arc<Mutex<HashMap<String, Node>>> = Arc::new(Mutex::new(current_nodes));

    let mut counter = 0;
    'outer: loop {
        for direction in &directions {
            {
                let all_nodes_at_end: bool = current_nodes
                    .clone()
                    .lock()
                    .unwrap()
                    .iter()
                    .all(|(_, node)| node.name.ends_with('Z'));

                if all_nodes_at_end {
                    break 'outer;
                }
            }

            if counter % 100000 == 0 {
                dbg!(counter);
            }

            // update all nodes with direction
            // let mut current_nodes = current_nodes.clone().lock().unwrap();
            let looping_nodes = { current_nodes.clone().lock().unwrap().clone() };
            for (start_node, current_node) in looping_nodes {
                let test = match direction {
                    Direction::LEFT => nodes.get(&current_node.left).unwrap(),
                    Direction::RIGHT => nodes.get(&current_node.right).unwrap(),
                };
                current_nodes
                    .clone()
                    .lock()
                    .unwrap()
                    .insert(start_node.clone(), test.clone());
            }

            counter += 1;
        }
    }

    Some(counter)
}

advent_of_code::main!(8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u32> = part_one(&advent_of_code::template::read_file("examples", 8));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 8));
        assert_eq!(result, Some(6));
    }
}
