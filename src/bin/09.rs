use itertools::Itertools;

enum Part {
    One,
    Two,
}

fn find_differences(part: Part, numbers: Vec<i64>, last_numbers: Vec<i64>) -> (Vec<i64>, Vec<i64>) {
    // calculate differences
    let differences = numbers
        .iter()
        .tuple_windows()
        .map(|(a, b)| match part {
            // there must be a better way, but my brain is not smart enough
            Part::One => b - a,
            Part::Two => a - b,
        })
        .collect_vec();

    // stop when the differences are all 0
    if differences.iter().copied().all(|number| number == 0) {
        return (differences, last_numbers);
    }

    // add last numbers
    let mut last_numbers = last_numbers;
    last_numbers.push(*differences.last().unwrap());

    find_differences(part, differences, last_numbers)
}

pub fn part_one(input: &str) -> Option<i64> {
    let answer: i64 = input
        .lines()
        .map(|line| {
            let numbers = line
                .split_whitespace()
                .map(|number| number.parse::<i64>().unwrap())
                .collect_vec();

            let last_number = *numbers.last().unwrap();
            let (_, last_numbers) = find_differences(Part::One, numbers, vec![last_number]);
            last_numbers.iter().sum::<i64>()
        })
        .sum();
    Some(answer)
}

pub fn part_two(input: &str) -> Option<i64> {
    let answer: i64 = input
        .lines()
        .map(|line| {
            let numbers = line
                .split_whitespace()
                .map(|number| number.parse::<i64>().unwrap())
                .rev()
                .collect_vec();

            let last_number = *numbers.last().unwrap();
            let (_, last_numbers) = find_differences(Part::Two, numbers, vec![last_number]);
            last_numbers.into_iter().rev().fold(0, |acc, e| e - acc)
        })
        .sum();

    Some(answer)
}

advent_of_code::main!(9);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 9));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 9));
        assert_eq!(result, Some(2));
    }
}
