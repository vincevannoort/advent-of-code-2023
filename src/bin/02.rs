use std::{collections::HashMap, str::FromStr};

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Color {
    Blue,
    Red,
    Green,
}

impl FromStr for Color {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blue" => Ok(Color::Blue),
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct DiceReveal {
    color: Color,
    value: u32,
}

impl FromStr for DiceReveal {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value, color) = s.split_once(' ').unwrap();
        Ok(DiceReveal {
            color: Color::from_str(color)?,
            value: value.parse::<u32>().unwrap(),
        })
    }
}

fn parse_line_to_game_and_dices(line: &str) -> (u32, Vec<DiceReveal>) {
    let (game, dices) = line.split_once(": ").unwrap();
    let game = &game[5..].parse::<u32>().unwrap();

    // dont care about sets, only about amount
    let dices: String = dices.replace(';', ",").to_owned();
    let dice_reveals = dices
        .split(", ")
        .map(|dice| DiceReveal::from_str(dice).unwrap())
        .collect();

    (*game, dice_reveals)
}

pub fn part_one(input: &str) -> Option<u32> {
    let test: u32 = input
        .lines()
        .filter_map(|line| {
            // dont care about sets, only about amount
            let (game, dice_reveals) = parse_line_to_game_and_dices(line);

            let dice_reveal_more_than_max: Option<&DiceReveal> =
                dice_reveals.iter().find(|d| match d.color {
                    Color::Red => d.value > MAX_RED,
                    Color::Green => d.value > MAX_GREEN,
                    Color::Blue => d.value > MAX_BLUE,
                });

            // game is not possible
            if dice_reveal_more_than_max.is_some() {
                return None;
            }

            // game is possible
            Some(game)
        })
        .sum();

    Some(test)
}

pub fn part_two(input: &str) -> Option<u32> {
    let test: u32 = input
        .lines()
        .map(|line| {
            let (_, dice_reveals) = parse_line_to_game_and_dices(line);

            // loop over all dice, replace if color value is higher
            let mut minimum_bag_configuration: HashMap<Color, u32> = HashMap::new();
            dice_reveals.iter().for_each(|d| {
                minimum_bag_configuration
                    // check for color
                    .entry(d.color)
                    // if there, replace if value is higher
                    .and_modify(|e| {
                        if d.value > *e {
                            *e = d.value;
                        }
                    })
                    // otherwise, insert value
                    .or_insert(d.value);
            });

            // product all minimal values together
            minimum_bag_configuration.values().product::<u32>()
        })
        .sum();

    Some(test)
}

advent_of_code::main!(2);

#[cfg(test)]
mod tests {
    use advent_of_code::template::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_file("examples", 2));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file("examples", 2));
        assert_eq!(result, Some(2286));
    }
}
