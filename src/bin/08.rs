use itertools::Itertools;
use num::Integer;
use std::collections::HashMap;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Hash)]
struct Node {
    name: String,
    left: String,
    right: String,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
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

pub fn part_one(input: &str) -> Option<u64> {
    let directions: Vec<Direction> = parse_directions(input);
    let nodes: HashMap<String, Node> = parse_nodes(input);

    let mut counter = 0;
    let mut current_node = nodes.get("AAA").unwrap();
    for direction in directions.iter().cycle() {
        if current_node.name == "ZZZ" {
            break;
        }
        current_node = match direction {
            Direction::Left => nodes.get(&current_node.left).unwrap(),
            Direction::Right => nodes.get(&current_node.right).unwrap(),
        };
        counter += 1;
    }

    Some(counter)
}

pub fn part_two(input: &str) -> Option<u64> {
    let directions: Vec<Direction> = parse_directions(input);
    let nodes: HashMap<String, Node> = parse_nodes(input);

    // find all starting nodes
    let starting_nodes: HashMap<String, Node> = nodes
        .clone()
        .into_iter()
        .filter(|(name, _)| name.ends_with('A'))
        .collect();

    // for each node, find the length it takes to reach the end node
    let mut cycle_lengths = HashMap::<String, usize>::new();
    for (start_node_name, start_node) in starting_nodes {
        let mut counter = 0;
        let mut current_node = start_node;
        for next_direction in directions.iter().cycle() {
            if current_node.name.ends_with('Z') {
                break;
            }
            current_node = match next_direction {
                Direction::Left => nodes.get(&current_node.left).unwrap().clone(),
                Direction::Right => nodes.get(&current_node.right).unwrap().clone(),
            };
            counter += 1;
        }
        cycle_lengths.insert(start_node_name, counter);
    }

    let answer: u64 = cycle_lengths
        .into_values()
        .reduce(|acc, e| acc.lcm(&e))
        .unwrap()
        .try_into()
        .unwrap();

    Some(answer)
}

advent_of_code::main!(8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u64> = part_one(&advent_of_code::template::read_file("examples", 8));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 8));
        assert_eq!(result, Some(6));
    }
}
