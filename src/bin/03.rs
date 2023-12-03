use regex::Regex;
use std::collections::HashMap;

fn insert_grid(input: &str, mut schematic: EngineSchematic) -> EngineSchematic {
    // insert all numbers into grid
    for (y, line) in input.lines().enumerate() {
        for (x, value) in line.chars().enumerate() {
            schematic.grid.insert(OriginalLocation { x, y }, value);
        }
    }
    schematic
}

/// insert numbers into grid
///
/// Match { start: 0, end: 3, string: "467"}
/// insert (location), ((start location), number)
/// insert (0, 0), ((0, 0), 467)
/// insert (1, 0), ((0, 0), 467)
/// insert (2, 0), ((0, 0), 467)
fn insert_numbers(input: &str, mut schematic: EngineSchematic) -> EngineSchematic {
    let re = Regex::new(r"\d+").unwrap();
    // insert all numbers into grid
    for (y, line) in input.lines().enumerate() {
        for captures in re.captures_iter(line) {
            for capture in captures.iter().flatten() {
                let digit = capture.as_str().parse::<u32>().unwrap();
                let x_start_location = capture.start();
                for x in x_start_location..capture.end() {
                    schematic.numbers.insert(
                        OffsetLocation { x, y },
                        (
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
    schematic
}

fn part_one_insert_symbols(mut schematic: EngineSchematic) -> EngineSchematic {
    for y in 0..schematic.height {
        for x in 0..schematic.width {
            let location = OriginalLocation { x, y };
            let value = schematic.grid[&location];
            match value {
                '0'..='9' => {}
                '.' => {}
                _ => {
                    schematic.symbols.insert(location, value);
                }
            }
        }
    }
    schematic
}

fn part_two_insert_symbols(mut schematic: EngineSchematic) -> EngineSchematic {
    for y in 0..schematic.height {
        for x in 0..schematic.width {
            let location = OriginalLocation { x, y };
            let value = schematic.grid[&location];
            if let '*' = value {
                schematic.symbols.insert(location, value);
            }
        }
    }
    schematic
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

#[derive(Debug)]
struct EngineSchematic {
    grid: HashMap<OriginalLocation, char>,
    symbols: HashMap<OriginalLocation, char>,
    // location, (start location, number)
    numbers: HashMap<OffsetLocation, (OriginalLocation, u32)>,
    width: usize,
    height: usize,
}

enum Part {
    One,
    Two,
}

impl EngineSchematic {
    fn create_schematic(input: &str, part: Part) -> EngineSchematic {
        let schematic = EngineSchematic {
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

        let schematic = { insert_grid(input, schematic) };
        let schematic = { insert_numbers(input, schematic) };
        match part {
            Part::One => part_one_insert_symbols(schematic),
            Part::Two => part_two_insert_symbols(schematic),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let schematic = EngineSchematic::create_schematic(input, Part::One);

    let mut found_numbers: HashMap<OriginalLocation, u32> = HashMap::new();

    // search around every symbol
    for (location, _) in schematic.symbols {
        for dy in -1..=1 {
            for dx in -1..=1 {
                let x: usize = (location.x as isize + dx).unsigned_abs();
                let y: usize = (location.y as isize + dy).unsigned_abs();
                let search = OffsetLocation { x, y };
                // found a number around symbol
                if let Some((original_location, number)) = schematic.numbers.get(&search) {
                    found_numbers.insert(*original_location, *number);
                }
            }
        }
    }

    // sum all found numbers
    let result = found_numbers.values().sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let schematic = EngineSchematic::create_schematic(input, Part::Two);

    // search around every symbol
    let result: u32 = schematic
        .symbols
        .iter()
        .flat_map(|(location, _)| {
            let mut found_numbers: HashMap<OriginalLocation, u32> = HashMap::new();
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let x: usize = (location.x as isize + dx).unsigned_abs();
                    let y: usize = (location.y as isize + dy).unsigned_abs();
                    let search = OffsetLocation { x, y };
                    // found a number around symbol
                    if let Some((original_location, number)) = schematic.numbers.get(&search) {
                        found_numbers.insert(*original_location, *number);
                    }
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
