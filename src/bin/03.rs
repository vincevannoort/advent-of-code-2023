use regex::Regex;
use std::collections::{HashMap, HashSet};

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
                let start_location = capture.start();
                for x in start_location..capture.end() {
                    schematic.numbers.insert(
                        OffsetLocation(x, y),
                        (OriginalLocation(start_location, y), digit),
                    );
                }
            }
        }

        for (x, value) in line.chars().enumerate() {
            schematic.grid.insert(OriginalLocation(x, y), value);
        }
    }
    schematic
}

fn part_one_update_schematic(mut schematic: EngineSchematic) -> EngineSchematic {
    for y in 0..schematic.height {
        for x in 0..schematic.width {
            let location = OriginalLocation(x, y);
            let value = schematic.grid[&location];
            match value {
                '0'..='9' => {}
                '.' => {}
                _ => {
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let _x: usize = (x as isize + dx).unsigned_abs();
                            let _y: usize = (y as isize + dy).unsigned_abs();
                            schematic
                                .symbols
                                .insert(OffsetLocation(_x, _y), (OriginalLocation(x, y), value));
                        }
                    }
                }
            }
        }
    }
    schematic
}

fn part_two_update_schematic(mut schematic: EngineSchematic) -> EngineSchematic {
    for y in 0..schematic.height {
        for x in 0..schematic.width {
            let location = OriginalLocation(x, y);
            let value = schematic.grid[&location];
            if let '*' = value {
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let _x: usize = (x as isize + dx).unsigned_abs();
                        let _y: usize = (y as isize + dy).unsigned_abs();
                        schematic
                            .symbols
                            .insert(OffsetLocation(_x, _y), (OriginalLocation(x, y), value));
                    }
                }
            }
        }
    }
    schematic
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct OriginalLocation(usize, usize);

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct OffsetLocation(usize, usize);

#[derive(Debug)]
struct EngineSchematic {
    grid: HashMap<OriginalLocation, char>,
    symbols: HashMap<OffsetLocation, (OriginalLocation, char)>,
    // location, (start location, number)
    numbers: HashMap<OffsetLocation, (OriginalLocation, u32)>,
    width: usize,
    height: usize,
}

pub fn part_one(input: &str) -> Option<u32> {
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

    let schematic = { insert_numbers(input, schematic) };
    let schematic = { part_one_update_schematic(schematic) };

    let mut found_numbers: HashMap<OriginalLocation, u32> = HashMap::new();
    for (location, (start_location, number)) in schematic.numbers {
        if schematic.symbols.get(&location).is_some() {
            found_numbers.insert(start_location, number);
        }
    }
    let result = found_numbers.values().sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
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

    let schematic = { insert_numbers(input, schematic) };
    let schematic = { part_two_update_schematic(schematic) };

    let mut found_gears: HashMap<OriginalLocation, HashSet<u32>> = HashMap::new();
    for (location, (_, number)) in schematic.numbers {
        if let Some((symbol_location, _)) = schematic.symbols.get(&location) {
            // found_numbers.insert(start_location, number);
            found_gears
                .entry(*symbol_location)
                .and_modify(|set| {
                    set.insert(number);
                })
                .or_insert(HashSet::from([number]));
        }
    }

    let found_gears: u32 = found_gears
        .iter()
        // only keep gears with 2 numbers adjacent
        .filter(|gear| gear.1.len() == 2)
        // multiply the two numbers
        .map(|gear| {
            let gear_ratio: u32 = gear.1.iter().product();
            dbg!(gear.1);
            dbg!(gear_ratio);
            gear_ratio
        })
        .sum();

    dbg!(found_gears);
    // let result = found_numbers.values().sum();
    Some(found_gears)
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
