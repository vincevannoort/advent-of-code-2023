use itertools::Itertools;

fn find_differences(numbers: Vec<i64>, last_numbers: Vec<i64>) -> (Vec<i64>, Vec<i64>) {
    // calculate differences
    let differences = numbers
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();

    // stop when the differences are all 0
    if differences.iter().copied().all(|number| number == 0) {
        return (differences, last_numbers);
    }

    // add last numbers
    let mut last_numbers = last_numbers;
    last_numbers.push(*differences.last().unwrap());

    find_differences(differences, last_numbers)
}

pub fn part_one(input: &str) -> Option<i64> {
    let answer: i64 = input
        .lines()
        .map(|line| {
            dbg!(line);
            let numbers = line
                .split_whitespace()
                .map(|number| number.parse::<i64>().unwrap())
                .collect_vec();

            let last_number = *numbers.last().unwrap();
            let (_, last_numbers) = find_differences(numbers, vec![last_number]);

            dbg!(&last_numbers);
            last_numbers.iter().sum::<i64>()
        })
        .sum();
    Some(answer)
}

pub fn part_two(input: &str) -> Option<i64> {
    None
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
        assert_eq!(result, Some(5));
    }
}
