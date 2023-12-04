use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

type CardNumber = u32;

#[derive(Debug, Eq, PartialEq, Clone)]
struct ScratchCard {
    number: CardNumber,
    winning_numbers: HashSet<CardNumber>,
    numbers: HashSet<CardNumber>,
}

pub fn parse_into_numbers(input: &str) -> HashSet<u32> {
    input
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect()
}

impl FromStr for ScratchCard {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let card_set = s
            .split_once(": ")
            .map(|(card, rest)| (card, rest.split_once(" | ").unwrap()))
            .map(|(card, (winning_numbers, numbers))| ScratchCard {
                // parse `Card   1` into 1u32
                number: card.replace("Card", "").trim_start().parse().unwrap(),
                winning_numbers: parse_into_numbers(winning_numbers),
                numbers: parse_into_numbers(numbers),
            })
            .unwrap();
        Ok(card_set)
    }
}

impl ScratchCard {
    fn get_matching_number_count(&self) -> u32 {
        self.winning_numbers.intersection(&self.numbers).count() as u32
    }
    fn get_score(&self) -> u32 {
        let count = self.get_matching_number_count();
        if count == 0 {
            0
        } else {
            (2u32).pow(count - 1)
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let total_score = input
        .lines()
        .map(|line| ScratchCard::from_str(line).unwrap().get_score())
        .sum();
    Some(total_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut card_copies: HashMap<CardNumber, u32> = HashMap::new();
    let cards: u32 = input.lines().count() as u32;

    for line in input.lines() {
        let card = ScratchCard::from_str(line).unwrap();
        let matching_numbers_count = card.get_matching_number_count();

        // get current card copies, if we don't have any stored, it means we have only one card
        let current_card_copies = card_copies.get(&card.number).cloned().unwrap_or(1);
        let next_card = card.number + 1;

        // update copies of the next cards
        for number in next_card..(next_card + matching_numbers_count) {
            card_copies
                .entry(number)
                .and_modify(|c: &mut u32| *c += current_card_copies)
                .or_insert(current_card_copies + 1);
        }
    }

    let card_with_copies = card_copies.values().sum::<u32>();
    let cards_without_copies = cards - card_copies.len() as u32;
    Some(card_with_copies + cards_without_copies)
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
        assert_eq!(result, Some(30));
    }
}
