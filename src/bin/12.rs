use itertools::Itertools;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl Condition {
    fn from_char(c: char) -> Condition {
        match c {
            '.' => Condition::Operational,
            '#' => Condition::Damaged,
            '?' => Condition::Unknown,
            // convert binary
            '0' => Condition::Operational,
            '1' => Condition::Damaged,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
struct ConditionRecord {
    records: Vec<Condition>,
    damaged_groups: Vec<u64>,
}

fn parse_records(records: &str) -> Vec<Condition> {
    records.chars().map(Condition::from_char).collect_vec()
}

impl ConditionRecord {
    fn from_string(s: &str) -> Self {
        let (records, damaged_groups) = s.split_once(' ').unwrap();
        let records = parse_records(records);
        let damaged_groups = damaged_groups
            .split(',')
            .map(|n| n.parse::<u64>().unwrap())
            .collect_vec();
        ConditionRecord {
            records,
            damaged_groups,
        }
    }
    fn get_possible_arrangements_with_unknowns(&self) -> u64 {
        let unknowns = self
            .records
            .iter()
            .filter(|c| c == &&Condition::Unknown)
            .count();

        let mut options = 0;
        // find all permutations using unknowns
        (0..(2_i64.pow(unknowns as u32))).for_each(|f| {
            let binary = format!("{:032b}", f);
            let binary = &binary[binary.len() - unknowns..];

            // replace unknown records
            let mut unknown_replacements = parse_records(binary).into_iter();
            let records = self.records.clone();
            let replaced_records = records
                .into_iter()
                .map(|r| {
                    // replace unknowns
                    if r == Condition::Unknown {
                        unknown_replacements.next().unwrap()
                    } else {
                        r
                    }
                })
                .collect::<Vec<Condition>>();

            // get groups of damaged items, from the replace records
            let groups = group_items(replaced_records)
                .into_iter()
                .filter(|(_, c)| c == &Condition::Damaged)
                .map(|(count, _)| count as u64)
                .collect_vec();

            // check if damaged groups matches the groups we created
            if self.damaged_groups == groups {
                options += 1;
            }
        });
        options
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let answer = input
        .lines()
        .map(|line| {
            let condition_record = ConditionRecord::from_string(line);
            condition_record.get_possible_arrangements_with_unknowns()
        })
        .sum();

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

advent_of_code::main!(12);

// copied
fn group_items<T: PartialEq>(items: Vec<T>) -> Vec<(usize, T)> {
    let mut grouped_items: Vec<(usize, T)> = Vec::new();

    for item in items {
        if let Some(last) = grouped_items.last_mut() {
            if last.1 == item {
                last.0 += 1;
                continue;
            }
        }
        grouped_items.push((1, item));
    }

    grouped_items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 12));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 12));
        assert_eq!(result, None);
    }
}
