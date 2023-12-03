use regex::Regex;
use std::collections::HashMap;

enum Part {
    One,
    Two,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Number(NumberIndex, u32);

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct NumberIndex(usize);

#[derive(Debug)]
struct EngineSchematic {
    grid: HashMap<Location, char>,
    symbols: HashMap<Location, char>,
    numbers: HashMap<Location, Number>,
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
        schematic.insert_symbols(part);

        schematic
    }

    fn insert_grid(&mut self, input: &str) {
        // insert all numbers into grid
        for (y, line) in input.lines().enumerate() {
            for (x, value) in line.chars().enumerate() {
                self.grid.insert(Location { x, y }, value);
            }
        }
    }

    fn insert_numbers(&mut self, input: &str) {
        // find all numbers in line
        let re = Regex::new(r"\d+").unwrap();
        // insert all numbers into grid
        let mut number_index = 0;
        for (y, line) in input.lines().enumerate() {
            for found_number in re.find_iter(line) {
                let number = found_number.as_str().parse::<u32>().unwrap();
                let start_location = found_number.start();
                for x in start_location..found_number.end() {
                    self.numbers
                        .insert(Location { x, y }, Number(NumberIndex(number_index), number));
                }
                number_index += 1;
            }
        }
    }

    fn insert_symbols(&mut self, part: Part) {
        for y in 0..self.height {
            for x in 0..self.width {
                let location = Location { x, y };
                let value = self.grid[&location];
                match part {
                    // part one we insert any symbol that is not a `digit` or `.`
                    Part::One => match value {
                        '0'..='9' => {}
                        '.' => {}
                        _ => {
                            self.symbols.insert(location, value);
                        }
                    },
                    // part two we insert only `*` symbols
                    Part::Two => {
                        if let '*' = value {
                            self.symbols.insert(location, value);
                        }
                    }
                }
            }
        }
    }
}

/// returns all spaces around the given location
fn get_search_spaces(location: Location) -> Vec<Location> {
    let mut search_space = Vec::new();
    for dy in -1..=1 {
        for dx in -1..=1 {
            // make sure we never go negative
            let x: usize = (location.x as isize + dx).unsigned_abs();
            let y: usize = (location.y as isize + dy).unsigned_abs();
            let search = Location { x, y };
            search_space.push(search);
        }
    }
    search_space
}

pub fn part_one(input: &str) -> Option<u32> {
    let schematic = EngineSchematic::new(input, Part::One);

    let mut found_numbers: HashMap<NumberIndex, u32> = HashMap::new();

    for (symbol_location, _) in schematic.symbols {
        // search all spaces around symbol
        for search in get_search_spaces(symbol_location) {
            // found a number around symbol
            if let Some(Number(number_index, number)) = schematic.numbers.get(&search) {
                found_numbers.insert(*number_index, *number);
            }
        }
    }

    // sum all found numbers
    let result = found_numbers.values().sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let schematic = EngineSchematic::new(input, Part::Two);

    let mut result = 0;

    for (symbol_location, _) in schematic.symbols {
        // prevent duplicates by using hashmap
        let mut found_numbers: HashMap<NumberIndex, u32> = HashMap::new();

        // search all spaces around symbol
        for search in get_search_spaces(symbol_location) {
            // found a number around symbol
            if let Some(Number(number_index, number)) = schematic.numbers.get(&search) {
                found_numbers.insert(*number_index, *number);
            }
        }

        // gear does not have two numbers
        if found_numbers.len() != 2 {
            continue;
        }

        let found_numbers: Vec<u32> = found_numbers.into_values().collect();

        // multiply gears
        let value = found_numbers.first().unwrap() * found_numbers.last().unwrap();
        result += value;
    }

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
