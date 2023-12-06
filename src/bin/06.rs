use std::iter::zip;

fn find_distance(max_time: u64, hold_time: u64) -> u64 {
    if hold_time >= max_time {
        return 0;
    }
    (max_time - hold_time) * hold_time
}

fn parse_to_single_number(s: &str) -> u64 {
    s.split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

fn parse_to_multiple_number(s: &str) -> Vec<u64> {
    s.split_whitespace()
        .skip(1)
        .map(|l| l.parse::<u64>().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (durations, records) = input.split_once('\n').unwrap();
    let durations: Vec<u64> = parse_to_multiple_number(durations);
    let records: Vec<u64> = parse_to_multiple_number(records);

    let races: Vec<(u64, u64)> = zip(durations, records).collect();
    let mut total_wins: Vec<u64> = vec![];
    for race in races {
        let mut wins = 0;
        let (duration, record) = race;
        for number in 1..duration {
            if find_distance(duration, number) > record {
                wins += 1
            }
        }
        total_wins.push(wins);
    }
    Some(total_wins.iter().product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (durations, records) = input.split_once('\n').unwrap();
    let duration: u64 = parse_to_single_number(durations);
    let record: u64 = parse_to_single_number(records);

    let mut wins = 0;
    for number in 1..duration {
        if find_distance(duration, number) > record {
            wins += 1
        }
    }
    Some(wins)
}

advent_of_code::main!(6);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 6));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 6));
        assert_eq!(result, Some(71503));
    }
}
