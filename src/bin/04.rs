use std::{collections::HashSet, str::FromStr};

pub fn parse_number(input: &str) -> HashSet<u32> {
    input
        .split_ascii_whitespace()
        .map(|number| number.parse::<u32>().unwrap())
        .collect()
}

struct CardSet {
    card: String,
    winning_numbers: HashSet<u32>,
    numbers: HashSet<u32>,
}

impl FromStr for CardSet {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let card_set = s
            .split_once(": ")
            .map(|(card, rest)| (card, rest.split_once(" | ").unwrap()))
            .map(|(card, (winning_numbers, numbers))| CardSet {
                card: card.to_owned(),
                winning_numbers: parse_number(winning_numbers),
                numbers: parse_number(numbers),
            })
            .unwrap();
        Ok(card_set)
    }
}

impl CardSet {
    fn get_winning_score(&self) -> u32 {
        let count = self.winning_numbers.intersection(&self.numbers).count() as u32;
        if count == 0 {
            return 0;
        }
        (2u32).pow(count - 1)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            let card_set = CardSet::from_str(line).unwrap();
            card_set.get_winning_score()
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(4);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 4));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 4));
        assert_eq!(result, None);
    }
}
