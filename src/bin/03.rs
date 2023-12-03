use regex::Regex;
use std::collections::HashMap;

enum Part {
    One,
    Two,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct OriginalLocation {
    x: usize,
    y: usize,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct OffsetLocation {
    x: usize,
    y: usize,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Number(OriginalLocation, u32);

#[derive(Debug)]
struct EngineSchematic {
    grid: HashMap<OriginalLocation, char>,
    symbols: HashMap<OriginalLocation, char>,
    numbers: HashMap<OffsetLocation, Number>,
    width: usize,
    height: usize,
}

impl EngineSchematic {
    fn new(input: &str, part: Part) -> EngineSchematic {
        let mut schematic = EngineSchematic {
            grid: HashMap::new(),
            symbols: HashMap::new(),
            numbers: HashMap::new(),
            width: input
                .lines()
                .collect::<Vec<&str>>()
                .first()
                .unwrap()
                .chars()
                .count(),
            height: input.lines().count(),
        };

        schematic.insert_grid(input);
        schematic.insert_numbers(input);

        // insert symbols
        match part {
            Part::One => {
                schematic.part_one_insert_symbols();
            }
            Part::Two => {
                schematic.part_two_insert_symbols();
            }
        }

        schematic
    }

    fn insert_grid(&mut self, input: &str) {
        // insert all numbers into grid
        for (y, line) in input.lines().enumerate() {
            for (x, value) in line.chars().enumerate() {
                self.grid.insert(OriginalLocation { x, y }, value);
            }
        }
    }

    /// insert numbers into grid
    ///
    /// Match { start: 0, end: 3, string: "467"}
    /// insert (location), ((start location), number)
    /// insert (0, 0), ((0, 0), 467)
    /// insert (1, 0), ((0, 0), 467)
    /// insert (2, 0), ((0, 0), 467)
    fn insert_numbers(&mut self, input: &str) {
        // find all numbers in line
        let re = Regex::new(r"\d+").unwrap();
        // insert all numbers into grid
        for (y, line) in input.lines().enumerate() {
            for captures in re.captures_iter(line) {
                for capture in captures.iter().flatten() {
                    let digit = capture.as_str().parse::<u32>().unwrap();
                    let x_start_location = capture.start();
                    for x in x_start_location..capture.end() {
                        self.numbers.insert(
                            OffsetLocation { x, y },
                            Number(
                                OriginalLocation {
                                    x: x_start_location,
                                    y,
                                },
                                digit,
                            ),
                        );
                    }
                }
            }
        }
    }

    fn part_one_insert_symbols(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let location = OriginalLocation { x, y };
                let value = self.grid[&location];
                match value {
                    '0'..='9' => {}
                    '.' => {}
                    _ => {
                        self.symbols.insert(location, value);
                    }
                }
            }
        }
    }

    fn part_two_insert_symbols(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let location = OriginalLocation { x, y };
                let value = self.grid[&location];
                if let '*' = value {
                    self.symbols.insert(location, value);
                }
            }
        }
    }
}

/// returns all spaces around the given location
fn get_search_spaces(location: OriginalLocation) -> Vec<OffsetLocation> {
    let mut search_space = Vec::new();
    for dy in -1..=1 {
        for dx in -1..=1 {
            let x: usize = (location.x as isize + dx).unsigned_abs();
            let y: usize = (location.y as isize + dy).unsigned_abs();
            let search = OffsetLocation { x, y };
            search_space.push(search);
        }
    }
    search_space
}

pub fn part_one(input: &str) -> Option<u32> {
    let schematic = EngineSchematic::new(input, Part::One);

    let mut found_numbers: HashMap<OriginalLocation, u32> = HashMap::new();

    // search around every symbol
    for (location, _) in schematic.symbols {
        for search in get_search_spaces(location) {
            // found a number around symbol
            if let Some(Number(location, number)) = schematic.numbers.get(&search) {
                found_numbers.insert(*location, *number);
            }
        }
    }

    // sum all found numbers
    let result = found_numbers.values().sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let schematic = EngineSchematic::new(input, Part::Two);

    // search around every symbol
    let result: u32 = schematic
        .symbols
        .into_iter()
        .flat_map(|(location, _)| {
            let mut found_numbers: HashMap<OriginalLocation, u32> = HashMap::new();
            for search in get_search_spaces(location) {
                // found a number around symbol
                if let Some(Number(location, number)) = schematic.numbers.get(&search) {
                    found_numbers.insert(*location, *number);
                }
            }

            // gear does not have two numbers
            if found_numbers.len() != 2 {
                return None;
            }

            Some(found_numbers.iter().map(|n| n.1).product::<u32>())
        })
        .sum();

    Some(result)
}

advent_of_code::main!(3);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 3));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 3));
        assert_eq!(result, Some(467835));
    }
}
