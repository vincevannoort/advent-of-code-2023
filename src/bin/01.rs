use phf::phf_map;

#[derive(Debug)]
struct CalibrationValue(char, char);

impl From<CalibrationValue> for u32 {
    fn from(item: CalibrationValue) -> Self {
        format!("{}{}", item.0, item.1).parse().unwrap()
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let value: u32 = input
        .lines()
        // find numerical digits
        .map(|line| {
            CalibrationValue(
                // find first
                line.chars().find(|c| c.is_ascii_digit()).unwrap(),
                // find last
                line.chars().rev().find(|c| c.is_ascii_digit()).unwrap(),
            )
        })
        // convert instruction into u32
        .map(u32::from)
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
    // select line part (ex: `twoone` -> `woone` -> `oone`, ...)
    for (location, character) in line.chars().enumerate() {
        let line_part: &str = &line[location..];

        // try finding numerical digit (ex: '1', '2', ...)
        if character.is_ascii_digit() {
            return character.to_string().parse().unwrap();
        }

        // try finding spelled digit (ex: 'one', 'two', ...)
        if let Some(digit) = spelled_digits
            .keys()
            .find(|digit| line_part.starts_with(*digit))
        {
            return spelled_digits.get(digit).unwrap().to_owned();
        }
    }
    todo!()
}

pub fn part_two(input: &str) -> Option<u32> {
    let value: u32 = input
        .lines()
        // find numerical and/or spelled digits
        .map(|line| {
            let first_digit = find_digit(line.to_string(), &DIGITS);
            let last_digit = find_digit(line.chars().rev().collect(), &DIGITS_REVERSED);
            CalibrationValue(first_digit, last_digit)
        })
        .map(u32::from)
        .sum();

    Some(value)
}

advent_of_code::main!(1);

#[cfg(test)]
mod tests {
    use advent_of_code::template::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_file("examples", 1));
        assert_eq!(result, Some(187));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file("examples", 1));
        assert_eq!(result, Some(179));
    }
}
