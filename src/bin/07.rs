use core::panic;
use std::cmp::{Ordering, Reverse};

use counter::Counter;
use itertools::Itertools;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
struct Hand {
    original_cards: [Card; 5],
    sorted_cards: [Card; 5],
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
enum Strenght {
    FiveOfAKind,
    FourOfAkind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

impl Hand {
    fn get_strength(self) -> Strenght {
        match self.sorted_cards {
            [a, b, c, d, e] if (a == b && b == c && c == d && d == e) => Strenght::FiveOfAKind,
            [a, b, c, d, _] if (a == b && b == c && c == d) => Strenght::FourOfAkind,
            [a, b, c, d, e] if (a == b && b == c && d == e) => Strenght::FullHouse,
            [a, b, c, _, _] if (a == b && b == c) => Strenght::ThreeOfAKind,
            [a, b, c, d, _] if (a == b && c == d) => Strenght::TwoPairs,
            [a, b, _, _, _] if (a == b) => Strenght::OnePair,
            [a, _, _, _, _] => Strenght::HighCard,
        }
    }

    fn get_strength_with_jokers(&self) -> Strenght {
        let jokers = self.get_jokers();
        self.get_strength().increase_strength_by_jokers(jokers)
    }

    fn get_jokers(&self) -> u32 {
        self.original_cards
            .into_iter()
            .filter(|c| *c == Card::Joker)
            .count()
            .try_into()
            .unwrap()
    }
}

impl Strenght {
    fn increase(strength: Strenght) -> Strenght {
        match strength {
            Strenght::HighCard => Strenght::OnePair,
            Strenght::OnePair => Strenght::TwoPairs,
            Strenght::TwoPairs => Strenght::ThreeOfAKind,
            Strenght::ThreeOfAKind => Strenght::FourOfAkind,
            Strenght::FourOfAkind => Strenght::FiveOfAKind,
            Strenght::FiveOfAKind => Strenght::FiveOfAKind,
            _ => panic!(),
        }
    }
    fn increase_strength_by_jokers(&self, jokers: u32) -> Strenght {
        if jokers == 0 {
            return *self;
        }

        // two pairs and one joker get full house
        if *self == Strenght::TwoPairs && jokers == 1 {
            return Strenght::FullHouse;
        }

        // full house and two or three jokers get five of a kind
        if *self == Strenght::FullHouse && (jokers == 2 || jokers == 3) {
            return Strenght::FiveOfAKind;
        }

        fn apply_n_times<T>(f: impl Fn(T) -> T, n: u32) -> impl Fn(T) -> T {
            move |arg| (0..n).fold(arg, |a, _| f(a))
        }
        apply_n_times(Strenght::increase, jokers)(*self)
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
enum Part {
    One,
    Two,
}

impl Card {
    fn parse_to_card(s: char, part: Part) -> Card {
        match s {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => match part {
                Part::One => Card::Jack,
                Part::Two => Card::Joker,
            },
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!(),
        }
    }
}

fn get_sorted_cards(input: &str, part: Part) -> Vec<(Hand, u32)> {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse::<u32>().unwrap();
            let cards: [Card; 5] = cards
                .chars()
                .take(5)
                .map(|c| Card::parse_to_card(c, part))
                .collect::<Vec<Card>>()
                .try_into()
                .unwrap();

            // card counting is okay, don't tell the casino's
            let counted_cards = cards.into_iter().collect::<Counter<_>>();

            // sort by count of cards
            let sorted_counted_cards: Vec<_> = counted_cards
                .into_iter()
                .map(|(card, amount)| (Reverse(amount), card))
                .sorted()
                // .sorted_by(|a, b| Ord::cmp(&b.0, &a.0))
                .collect();

            // converted sorted cards into array of 5 again
            let sorted_cards: [Card; 5] = sorted_counted_cards
                .into_iter()
                .flat_map(|(amount, card)| vec![card; amount.0])
                .collect::<Vec<Card>>()
                .try_into()
                .unwrap();

            (
                Hand {
                    original_cards: cards,
                    sorted_cards,
                },
                bid,
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let total_winnings: u32 = get_sorted_cards(input, Part::One)
        .iter()
        // sort hands by strength
        .sorted_by(|(a_hand, _), (b_hand, _)| {
            let comparison = Ord::cmp(&b_hand.get_strength(), &a_hand.get_strength());
            if comparison == Ordering::Equal {
                // if their strength is the same, compare original hand
                return Ord::cmp(&b_hand.original_cards, &a_hand.original_cards);
            }
            comparison
        })
        // enumerate to get the rank
        .enumerate()
        .map(|(rank, (_, bid))| {
            // 0 indexed
            let rank = rank + 1;
            rank as u32 * bid
        })
        .sum();

    Some(total_winnings)
}

pub fn part_two(input: &str) -> Option<u32> {
    let total_winnings: u32 = get_sorted_cards(input, Part::Two)
        .iter()
        // sort hands by strength
        .sorted_by(|(a_hand, _), (b_hand, _)| {
            let comparison = Ord::cmp(
                &b_hand.get_strength_with_jokers(),
                &a_hand.get_strength_with_jokers(),
            );
            if comparison == Ordering::Equal {
                // if their strength is the same, compare original hand
                return Ord::cmp(&b_hand.original_cards, &a_hand.original_cards);
            }
            comparison
        })
        // enumerate to get the rank
        .enumerate()
        .inspect(|(_, (hand, _))| {
            // dbg!();
            // dbg!(rank);
            let cards = format!(
                "{:?} {:?} {:?} {:?} {:?}",
                hand.original_cards[0],
                hand.original_cards[1],
                hand.original_cards[2],
                hand.original_cards[3],
                hand.original_cards[4]
            );
            dbg!();
            dbg!(cards);
            dbg!(&hand.get_strength());
            dbg!(&hand.get_strength_with_jokers());
            // dbg!(hand.get_strength());
            // dbg!(hand.get_strength_with_jokers());
        })
        .map(|(rank, (_, bid))| {
            // 0 indexed
            let rank = rank + 1;
            rank as u32 * bid
        })
        .sum();
    Some(total_winnings)
}

advent_of_code::main!(7);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increasing_by_jokers() {
        assert_eq!(
            Strenght::OnePair.increase_strength_by_jokers(2),
            Strenght::ThreeOfAKind
        );
        assert_eq!(
            Strenght::ThreeOfAKind.increase_strength_by_jokers(1),
            Strenght::FourOfAkind
        );
        assert_eq!(
            Strenght::OnePair.increase_strength_by_jokers(4),
            Strenght::FiveOfAKind
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 7));
        assert_eq!(result, Some(6592));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 7));
        assert_eq!(result, Some(6839));
    }
}
