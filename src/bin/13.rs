use std::{
    cmp::{max, min},
    collections::HashMap,
};

use itertools::Itertools;

// enum Ground {
//     Ash,
//     Rock,
// }
#[derive(Debug)]
struct Pattern {
    pattern: HashMap<(usize, usize), char>,
    width: usize,
    height: usize,
}

impl Pattern {
    fn from_string(s: &str) -> Self {
        // make pattern
        let pattern: Vec<((usize, usize), char)> = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| ((x, y), c))
                    .collect_vec()
            })
            .collect_vec();

        // get width and height
        let width = s
            .lines()
            .collect::<Vec<&str>>()
            .first()
            .unwrap()
            .chars()
            .count();
        let height = s.lines().count();

        Self {
            pattern: HashMap::from_iter(pattern),
            width,
            height,
        }
    }

    fn get_rows(&self) -> Vec<String> {
        (0..self.height)
            .map(|y| {
                (0..self.width)
                    .map(|x| *self.pattern.get(&(x, y)).unwrap())
                    .collect()
            })
            .collect_vec()
    }

    fn get_columns(&self) -> Vec<String> {
        (0..self.width)
            .map(|x| {
                (0..self.height)
                    .map(|y| *self.pattern.get(&(x, y)).unwrap())
                    .collect()
            })
            .collect_vec()
    }

    fn find_reflection(&self) -> usize {
        let rows = self.get_rows();
        let possible_row_reflections: Vec<((usize, &String), (usize, &String))> = rows
            .iter()
            .enumerate()
            .tuple_windows()
            .filter(|((_, a), (_, b))| a == b)
            .collect_vec();

        let row_reflection = possible_row_reflections
            .into_iter()
            .find(|r| self.check_valid_reflection(*r, rows.clone()));

        if let Some((_, (middle, _))) = row_reflection {
            format!("found horizontal reflection at: {}", middle);
            return middle * 100;
        }

        let columns = self.get_columns();
        let possible_column_reflections: Vec<((usize, &String), (usize, &String))> = columns
            .iter()
            .enumerate()
            .tuple_windows()
            .filter(|((_, a), (_, b))| a == b)
            .collect_vec();

        let column_reflection = possible_column_reflections
            .into_iter()
            .find(|c| self.check_valid_reflection(*c, columns.clone()));

        if let Some((_, (middle, _))) = column_reflection {
            format!("found vertical reflection at: {}", middle);
            return middle;
        };

        panic!("did not find pattern");
    }

    fn check_valid_reflection(
        &self,
        reflection: ((usize, &String), (usize, &String)),
        items: Vec<String>,
    ) -> bool {
        let reflection_middle = reflection.1 .0;
        let lenght = items.len();
        let first_part = reflection_middle;
        let last_part = lenght - first_part;
        let max_length = min(first_part, last_part);

        let first_part: Vec<&String> = items.iter().take(reflection_middle).rev().collect_vec();
        let last_part: Vec<&String> = items.iter().skip(reflection_middle).collect();
        let first_part = first_part.iter().take(max_length).collect_vec();
        let last_part = last_part.iter().take(max_length).collect_vec();

        first_part == last_part
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let answer: usize = input
        .split("\n\n")
        .map(Pattern::from_string)
        .map(|pattern| {
            // dbg!(pattern);
            // dbg!();
            // dbg!(pattern.get_columns());
            // dbg!(pattern.get_rows());
            pattern.find_reflection()
        })
        .sum();
    Some(answer)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

advent_of_code::main!(13);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 13));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 13));
        assert_eq!(result, None);
    }
}
