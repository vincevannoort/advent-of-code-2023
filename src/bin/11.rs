use colored::Colorize;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Galaxy(i64);

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Universe {
    width: i64,
    height: i64,
    map: HashMap<Position, Galaxy>,
}

impl Universe {
    fn expand(&self, expansion: usize) -> Self {
        // vertical expands
        let mut vertical_expands: Vec<i64> = vec![];
        for x in 0..self.width {
            let no_galaxies = (0..self.height).all(|y| self.map.get(&Position { x, y }).is_none());
            if no_galaxies {
                vertical_expands.push(x);
            }
        }

        // horizontal expands
        let mut horizontal_expands: Vec<i64> = vec![];
        for y in 0..self.height {
            let no_galaxies = (0..self.width).all(|x| self.map.get(&Position { x, y }).is_none());
            if no_galaxies {
                horizontal_expands.push(y);
            }
        }

        // create new universe map, using the expansions
        let mut new_map: HashMap<Position, Galaxy> = HashMap::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let x_offset =
                    (vertical_expands.iter().filter(|exp| exp < &&x).count() * expansion) as i64;
                let y_offset =
                    (horizontal_expands.iter().filter(|exp| exp < &&y).count() * expansion) as i64;

                let old_position = Position { x, y };
                if let Some(t) = self.map.get(&old_position) {
                    let new_position = Position {
                        x: x + x_offset,
                        y: y + y_offset,
                    };
                    new_map.insert(new_position, t.clone());
                }
            }
        }

        Universe {
            width: self.width + (vertical_expands.len() * expansion) as i64,
            height: self.height + (horizontal_expands.len() * expansion) as i64,
            map: new_map,
        }
    }

    fn draw(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(_galaxy) = self.map.get(&Position { x, y }) {
                    // print!("{}", galaxy.0.to_string().red());
                    print!("{}", "#".red());
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

    fn find_difference_between_all_pairs(&self) -> i64 {
        let answer: i64 = self
            .map
            .iter()
            .tuple_combinations::<(_, _)>()
            .map(|((a_pos, _), (b_pos, _))| {
                let y_diff = (b_pos.y - a_pos.y).abs();
                let x_diff = (b_pos.x - a_pos.x).abs();
                (y_diff + x_diff).abs()
            })
            .sum();
        answer
    }
}

fn parse_universe(input: &str) -> Universe {
    let mut counter = 0;
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .flat_map(|(x, c)| match c {
                    '#' => {
                        counter += 1;
                        Some((
                            Position {
                                x: x.try_into().unwrap(),
                                y: y.try_into().unwrap(),
                            },
                            Galaxy(counter),
                        ))
                    }
                    _ => None,
                })
                .collect_vec()
        })
        .collect_vec();

    Universe {
        width: input
            .lines()
            .collect::<Vec<&str>>()
            .first()
            .unwrap()
            .chars()
            .count() as i64,
        height: input.lines().count() as i64,
        map: HashMap::from_iter(map),
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let universe = parse_universe(input);
    let expanded_universe = universe.expand(2 - 1);
    let answer: i64 = expanded_universe.find_difference_between_all_pairs();
    Some(answer)
}

pub fn part_two(input: &str) -> Option<i64> {
    let universe = parse_universe(input);
    let expanded_universe = universe.expand(1000000 - 1);
    let answer: i64 = expanded_universe.find_difference_between_all_pairs();
    Some(answer)
}

advent_of_code::main!(11);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 11));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 11));
        assert_eq!(result, Some(8410));
    }
}
