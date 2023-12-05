use rayon::prelude::*;
use std::{collections::HashMap, ops::Range, str::FromStr};
type Source = u64;
type Destination = u64;
type SourceDestinationMapping = HashMap<Range<Source>, Destination>;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Mapping {
    source: String,
    destination: String,
    mapping: SourceDestinationMapping,
}

impl Mapping {
    fn get_destination_by_source(&self, source: u64) -> u64 {
        if let Some((range, destination)) = self.mapping.iter().find(|(k, _)| k.contains(&source)) {
            destination + source - range.start
        } else {
            source
        }
    }
}

impl FromStr for Mapping {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from_to, mapping) = s.split_once(" map:\n").unwrap();
        let (from, to) = from_to.split_once("-to-").unwrap();
        let mapping: SourceDestinationMapping = HashMap::from_iter(
            mapping
                .lines()
                .map(|line| {
                    let mut range = line.splitn(3, ' ');
                    (
                        range.next().unwrap(),
                        range.next().unwrap(),
                        range.next().unwrap(),
                    )
                })
                // given mapping `50 98 2`
                // maps source to destination
                // 98 -> 50
                // 99 -> 51
                .map(|(destination, source, amount)| {
                    let destination = destination.parse::<u64>().unwrap();
                    let source = source.parse::<u64>().unwrap();
                    let amount = amount.parse::<u64>().unwrap();
                    (source..(source + amount), destination)
                }),
        );

        Ok(Mapping {
            source: from.to_string(),
            destination: to.to_string(),
            mapping,
        })
    }
}

fn parse_seeds(s: &str) -> Vec<u64> {
    s.split_once("seeds: ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn parse_mappings(s: &str) -> Vec<Mapping> {
    s.split("\n\n")
        .map(|m| Mapping::from_str(m).unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let seeds: Vec<u64> = parse_seeds(seeds);
    let mappings: Vec<Mapping> = parse_mappings(maps);

    let lowest_location: u64 = seeds
        .into_iter()
        .map(|seed| {
            let mut number = seed;
            mappings.iter().for_each(|mapping| {
                number = mapping.get_destination_by_source(number);
            });
            number
        })
        .min()
        .unwrap();

    Some(lowest_location)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let seeds: Vec<u64> = parse_seeds(seeds);
    let seed_ranges: Vec<Range<u64>> = seeds.chunks(2).map(|w| (w[0]..w[0] + w[1])).collect();
    let mappings: Vec<Mapping> = parse_mappings(maps);

    let lowest_location = seed_ranges
        .par_iter()
        .map(|seed_range| {
            seed_range
                .clone()
                .map(|seed| {
                    let mut number = seed;
                    for mapping in mappings.iter() {
                        number = mapping.get_destination_by_source(number);
                    }
                    number
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

    Some(lowest_location)
}

advent_of_code::main!(5);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 5));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 5));
        assert_eq!(result, Some(46));
    }
}
