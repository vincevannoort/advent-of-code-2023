use std::{
    collections::{HashMap, HashSet},
    ops::Range,
    str::FromStr,
};

#[derive(Debug, Eq, PartialEq, Clone)]
struct Mapping {
    from: String,
    to: String,
    seed_map: HashMap<u32, Range<u32>>,
}

impl FromStr for Mapping {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from_to, seed_map) = s.split_once(" map:\n").unwrap();
        let (from, to) = from_to.split_once("-to-").unwrap();
        let seed_map: HashMap<u32, Range<u32>> = HashMap::from_iter(
            seed_map
                .lines()
                .map(|line| {
                    let mut range = line.splitn(3, ' ');
                    (
                        range.next().unwrap(),
                        range.next().unwrap(),
                        range.next().unwrap(),
                    )
                })
                .map(|(seed, start, amount)| {
                    let seed = seed.parse::<u32>().unwrap();
                    let start = start.parse::<u32>().unwrap();
                    let amount = amount.parse::<u32>().unwrap();
                    (seed, start..start + amount)
                }),
        );
        Ok(Mapping {
            from: from.to_string(),
            to: to.to_string(),
            seed_map,
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let maps: Vec<_> = maps
        .split("\n\n")
        .map(|m| Mapping::from_str(m).unwrap())
        .collect();
    dbg!(seeds);
    dbg!(maps);
    Some(1)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(5);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 5));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 5));
        assert_eq!(result, None);
    }
}
