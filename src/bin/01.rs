use phf::phf_map;

#[derive(Debug)]
struct Instruction(char, char);

impl From<Instruction> for u32 {
    fn from(item: Instruction) -> Self {
        format!("{}{}", item.0, item.1).parse().unwrap()
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

static DIGITS: phf::Map<&'static str, char> = phf_map! {
    "one" => '1',
    "two" => '2',
    "three" => '3',
    "four" => '4',
    "five" => '5',
    "six" => '6',
    "seven" => '7',
    "eight" => '8',
    "nine" => '9',
};

static DIGITS_REVERSED: phf::Map<&'static str, char> = phf_map! {
    "eno" => '1',
    "owt" => '2',
    "eerht" => '3',
    "ruof" => '4',
    "evif" => '5',
    "xis" => '6',
    "neves" => '7',
    "thgie" => '8',
    "enin" => '9',
};

fn find_digit(line: String, spelled_digits: &phf::Map<&'static str, char>) -> char {
    for (location, character) in line.chars().enumerate() {
        // digit found
        if character.is_digit(10) {
            return character.to_string().parse().unwrap();
        }
        let line_part: &str = &line[location..];
        for digit in spelled_digits.keys() {
            // spelled digit found
            if line_part.starts_with(digit) {
                return spelled_digits.get(digit).unwrap().to_owned();
            }
        }
    }
    todo!()
}

pub fn part_two(input: &str) -> Option<u32> {
    let value: u32 = input
        .lines()
        // replace
        .map(|line| {
            let first_digit = find_digit(line.to_string(), &DIGITS);
            let last_digit = find_digit(line.chars().rev().collect(), &DIGITS_REVERSED);
            Instruction(first_digit, last_digit)
        })
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
