use std::cmp::Ordering;
use std::fmt;

#[derive(Debug)]
struct Instruction(char, char);

impl From<Instruction> for u32 {
    fn from(item: Instruction) -> Self {
        format!("{}{}", item.0, item.1).parse().unwrap()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum SpelledDigit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl From<String> for SpelledDigit {
    fn from(item: String) -> Self {
        match item.as_ref() {
            "one" => SpelledDigit::One,
            "two" => SpelledDigit::Two,
            "three" => SpelledDigit::Three,
            "four" => SpelledDigit::Four,
            "five" => SpelledDigit::Five,
            "six" => SpelledDigit::Six,
            "seven" => SpelledDigit::Seven,
            "eight" => SpelledDigit::Eight,
            "nine" => SpelledDigit::Nine,
            _ => todo!(),
        }
    }
}

impl From<SpelledDigit> for u32 {
    fn from(item: SpelledDigit) -> Self {
        match item {
            SpelledDigit::One => 1,
            SpelledDigit::Two => 2,
            SpelledDigit::Three => 3,
            SpelledDigit::Four => 4,
            SpelledDigit::Five => 5,
            SpelledDigit::Six => 6,
            SpelledDigit::Seven => 7,
            SpelledDigit::Eight => 8,
            SpelledDigit::Nine => 9,
        }
    }
}

impl fmt::Display for SpelledDigit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SpelledDigit::One => write!(f, "one"),
            SpelledDigit::Two => write!(f, "two"),
            SpelledDigit::Three => write!(f, "three"),
            SpelledDigit::Four => write!(f, "four"),
            SpelledDigit::Five => write!(f, "five"),
            SpelledDigit::Six => write!(f, "six"),
            SpelledDigit::Seven => write!(f, "seven"),
            SpelledDigit::Eight => write!(f, "eight"),
            SpelledDigit::Nine => write!(f, "nine"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct FoundDigitLocation(u32, SpelledDigit);

impl FoundDigitLocation {
    fn get_location(&self) -> usize {
        self.0.try_into().unwrap()
    }
}

impl Ord for FoundDigitLocation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for FoundDigitLocation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl Eq for FoundDigitLocation {}

impl PartialEq for FoundDigitLocation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let value: u32 = input
        .lines()
        // create instructions
        .map(|line| {
            Instruction(
                // find first
                line.chars().find(|c| c.is_digit(10)).unwrap(),
                // find last
                line.chars().rev().find(|c| c.is_digit(10)).unwrap(),
            )
        })
        // convert instruction into u32
        .map(|instruction| u32::from(instruction))
        .sum();

    Some(value)
}

static DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn replace_first_digit(line: String) -> String {
    let mut line = line;

    let first_digit_without_conversion = line.chars().position(|c| c.is_digit(10));
    let found_digits: Vec<FoundDigitLocation> = DIGITS
        .map(|digit| {
            line.find(digit).and_then(|location| {
                Some(FoundDigitLocation(
                    location.try_into().unwrap(),
                    SpelledDigit::from(digit.to_owned()),
                ))
            })
        })
        .into_iter()
        .flatten()
        .collect();

    let first_digit = found_digits.iter().min();
    if let Some(first_digit) = first_digit {
        let digit_spelled = first_digit.1.to_string();
        let digit_spelled_converted = u32::from(first_digit.1).to_string();

        if let Some(first_digit_without_conversion) = first_digit_without_conversion {
            if first_digit_without_conversion > first_digit.get_location() {
                line = line.replacen(&digit_spelled, &digit_spelled_converted, 1);
            }
        } else {
            line = line.replacen(&digit_spelled, &digit_spelled_converted, 1);
        }
    };

    line
}

fn replace_last_digit(line: String) -> String {
    let mut line = line;

    let last_digit_without_conversion = line
        .chars()
        .rev()
        .position(|c| c.is_digit(10))
        .and_then(|d| Some(line.len() - d - 1));

    let found_digits: Vec<FoundDigitLocation> = DIGITS
        .map(|digit: &str| {
            let reversed_line = &line.chars().rev().collect::<String>();
            let reversed_digit: String = digit.chars().rev().collect();
            reversed_line.find(&reversed_digit).and_then(|location| {
                Some(FoundDigitLocation(
                    (line.len() - location - reversed_digit.len())
                        .try_into()
                        .unwrap(),
                    SpelledDigit::from(digit.to_owned()),
                ))
            })
        })
        .into_iter()
        .flatten()
        .collect();

    let last_digit = found_digits.iter().max();

    if let Some(last_digit) = last_digit {
        let digit_spelled = last_digit.1.to_string();
        let digit_spelled_converted = u32::from(last_digit.1).to_string();

        if let Some(last_digit_without_conversion) = last_digit_without_conversion {
            if last_digit_without_conversion < last_digit.get_location() {
                line.replace_range(
                    last_digit.get_location()..last_digit.get_location() + digit_spelled.len(),
                    &digit_spelled_converted,
                )
            }
        } else {
            line.replace_range(
                last_digit.get_location()..last_digit.get_location() + digit_spelled.len(),
                &digit_spelled_converted,
            )
        }
    };

    line
}

pub fn part_two(input: &str) -> Option<u32> {
    let value: u32 = input
        .lines()
        // replace
        .map(|line| {
            let line = replace_first_digit(line.to_string());
            let line = replace_last_digit(line.to_string());
            line
        })
        .map(|line| {
            Instruction(
                // find first
                line.chars().find(|c| c.is_digit(10)).unwrap(),
                // find last
                line.chars().rev().find(|c| c.is_digit(10)).unwrap(),
            )
        })
        // convert instruction into u32
        .map(|instruction| u32::from(instruction))
        .sum();

    Some(value)
}

advent_of_code::main!(1);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("extra-examples", 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 1));
        assert_eq!(result, Some(23));
    }
}
