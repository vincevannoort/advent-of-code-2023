use colored::Colorize;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    x: u64,
    y: u64,
}

impl Position {
    fn get_next_position(&self, direction: &Direction) -> Position {
        match direction {
            Direction::North => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::East => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::South => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Position {
                x: self.x - 1,
                y: self.y,
            },
        }
    }

    fn get_search_positions(&self) -> [(Direction, Position); 4] {
        [
            // north
            (
                Direction::North,
                Position {
                    x: self.x,
                    y: self.y - 1,
                },
            ),
            // east
            (
                Direction::East,
                Position {
                    x: self.x + 1,
                    y: self.y,
                },
            ),
            // south
            (
                Direction::South,
                Position {
                    x: self.x,
                    y: self.y + 1,
                },
            ),
            // west
            (
                Direction::West,
                Position {
                    x: self.x - 1,
                    y: self.y,
                },
            ),
        ]
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    NorthSouthPipe,
    WestEastPipe,
    NorthEaseBend,
    NorthWestBend,
    SouthWestBend,
    SouthEastBend,
    // Ground,
    AnimalStartingPosition,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Tile {
    fn from_char(c: char) -> Option<Tile> {
        match c {
            '|' => Some(Tile::NorthSouthPipe),
            '-' => Some(Tile::WestEastPipe),
            'L' => Some(Tile::NorthEaseBend),
            'J' => Some(Tile::NorthWestBend),
            '7' => Some(Tile::SouthWestBend),
            'F' => Some(Tile::SouthEastBend),
            // '.' => Some(Tile::Ground),
            'S' => Some(Tile::AnimalStartingPosition),
            _ => None,
        }
    }

    fn get_char(&self) -> char {
        match self {
            Tile::NorthSouthPipe => '|',
            Tile::WestEastPipe => '-',
            Tile::NorthEaseBend => 'L',
            Tile::NorthWestBend => 'J',
            Tile::SouthWestBend => '7',
            Tile::SouthEastBend => 'F',
            Tile::AnimalStartingPosition => 'S',
        }
    }

    fn get_out_direction(&self, in_direction: Direction) -> Option<Direction> {
        match (self, in_direction) {
            // |
            (Tile::NorthSouthPipe, Direction::North) => Some(Direction::North),
            (Tile::NorthSouthPipe, Direction::South) => Some(Direction::South),
            // -
            (Tile::WestEastPipe, Direction::East) => Some(Direction::East),
            (Tile::WestEastPipe, Direction::West) => Some(Direction::West),
            // L
            (Tile::NorthEaseBend, Direction::South) => Some(Direction::East),
            (Tile::NorthEaseBend, Direction::West) => Some(Direction::North),
            // J
            (Tile::NorthWestBend, Direction::South) => Some(Direction::West),
            (Tile::NorthWestBend, Direction::East) => Some(Direction::North),
            // 7
            (Tile::SouthWestBend, Direction::North) => Some(Direction::West),
            (Tile::SouthWestBend, Direction::East) => Some(Direction::South),
            // F
            (Tile::SouthEastBend, Direction::West) => Some(Direction::South),
            (Tile::SouthEastBend, Direction::North) => Some(Direction::East),
            //
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct GroundMap {
    width: u64,
    height: u64,
    tiles: HashMap<Position, Tile>,
}

impl GroundMap {
    fn get_start_position(&self) -> Position {
        let animal = self
            .tiles
            .iter()
            .find(|(_, t)| t == &&Tile::AnimalStartingPosition)
            .unwrap();

        *animal.0
    }
    fn draw(&self, loop_tiles: HashMap<Position, Tile>) {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(a) = loop_tiles.get(&Position { x, y }) {
                    print!("{}", a.get_char().to_string().red());
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

fn parse_ground_map(s: &str) -> GroundMap {
    // let map: HashMap<(u64, u64), Tile>/
    let tiles: Vec<(Position, Tile)> = s
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let test = line
                .chars()
                .map(Tile::from_char)
                .enumerate()
                .flat_map(|(x, tile)| {
                    tile.map(|tile| {
                        (
                            Position {
                                x: x.try_into().unwrap(),
                                y: y.try_into().unwrap(),
                            },
                            tile,
                        )
                    })
                })
                .collect_vec();

            test
        })
        .collect_vec();

    GroundMap {
        width: s
            .lines()
            .collect::<Vec<&str>>()
            .first()
            .unwrap()
            .chars()
            .count() as u64,
        height: s.lines().count() as u64,
        tiles: HashMap::from_iter(tiles),
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    // let map: HashMap<(u64, u64), Tile>/
    let ground_map = parse_ground_map(input);
    let start_position = ground_map.get_start_position();
    let (mut next_direction, mut next_position) = start_position
        .get_search_positions()
        .into_iter()
        .find(|(direction, position)| {
            // ground_map.tiles.get(&position)
            if let Some(tile) = ground_map.tiles.get(position) {
                tile.get_out_direction(*direction).is_some()
            } else {
                false
            }
        })
        .unwrap();

    let mut loop_tiles: HashMap<Position, Tile> = HashMap::new();

    loop {
        // if loop_tiles.clone().len() % 1000 == 0 {
        //     ground_map.draw(loop_tiles.clone());
        // }

        let next_tile = ground_map.tiles.get(&next_position).unwrap();
        loop_tiles.insert(next_position, *next_tile);

        // check, completed loop
        if next_tile == &Tile::AnimalStartingPosition {
            dbg!("loop complete");
            break;
        }

        next_direction = next_tile.get_out_direction(next_direction).unwrap();
        next_position = next_position.get_next_position(&next_direction);
    }

    let middle: u64 = loop_tiles.len().try_into().unwrap();

    Some(middle)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

advent_of_code::main!(10);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 10));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 10));
        assert_eq!(result, None);
    }
}
