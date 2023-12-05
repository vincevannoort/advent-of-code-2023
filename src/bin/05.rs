use core::panic;
use rayon::prelude::*;
use std::{collections::HashMap, ops::Range, str::FromStr};
type Source = u64;
type Destination = u64;
type SourceDestinationMapping = HashMap<Range<Source>, Destination>;

#[derive(Debug, Eq, PartialEq, Clone)]
struct ObjectMapping {
    source: String,
    destination: String,
    mapping: SourceDestinationMapping,
}

impl ObjectMapping {
    fn get_destination_by_source(&self, source: u64) -> u64 {
        if let Some((range, destination)) = self.mapping.iter().find(|(k, _)| k.contains(&source)) {
            destination + source - range.start
        } else {
            source
        }
    }
}

impl FromStr for ObjectMapping {
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
                .map(|(destination, source_start, amount)| {
                    let destination = destination.parse::<u64>().unwrap();
                    let source_start = source_start.parse::<u64>().unwrap();
                    let amount = amount.parse::<u64>().unwrap();
                    (source_start..source_start + amount, destination)
                }),
        );
        let object_mapping = ObjectMapping {
            source: from.to_string(),
            destination: to.to_string(),
            mapping,
        };

        Ok(object_mapping)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let seeds: Vec<u64> = seeds
        .split_once("seeds: ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let maps: Vec<_> = maps
        .split("\n\n")
        .map(|m| ObjectMapping::from_str(m).unwrap())
        .collect();

    let mut test = maps.iter().take(7);
    let seed_to_soil = test.next().unwrap();
    let soil_to_fertilizer = test.next().unwrap();
    let fertilizer_to_water = test.next().unwrap();
    let water_to_light = test.next().unwrap();
    let light_to_temperature = test.next().unwrap();
    let temperature_to_humidity = test.next().unwrap();
    let humidity_to_location = test.next().unwrap();

    dbg!(humidity_to_location.get_destination_by_source(
        temperature_to_humidity.get_destination_by_source(
            light_to_temperature.get_destination_by_source(
                water_to_light.get_destination_by_source(
                    fertilizer_to_water.get_destination_by_source(
                        soil_to_fertilizer
                            .get_destination_by_source(seed_to_soil.get_destination_by_source(79))
                    )
                )
            )
        )
    ));

    let lowest_location: u64 = seeds
        .into_iter()
        .map(|seed| {
            humidity_to_location.get_destination_by_source(
                temperature_to_humidity.get_destination_by_source(
                    light_to_temperature.get_destination_by_source(
                        water_to_light.get_destination_by_source(
                            fertilizer_to_water.get_destination_by_source(
                                soil_to_fertilizer.get_destination_by_source(
                                    seed_to_soil.get_destination_by_source(seed),
                                ),
                            ),
                        ),
                    ),
                ),
            )
        })
        .min()
        .unwrap();

    Some(lowest_location)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let seeds: Vec<u64> = seeds
        .split_once("seeds: ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let seed_ranges: Vec<Range<u64>> = seeds.chunks(2).map(|w| (w[0]..w[0] + w[1])).collect();

    let maps: Vec<_> = maps
        .split("\n\n")
        .map(|m| ObjectMapping::from_str(m).unwrap())
        .collect();

    let mut test = maps.iter().take(7);
    let seed_to_soil = test.next().unwrap();
    let soil_to_fertilizer = test.next().unwrap();
    let fertilizer_to_water = test.next().unwrap();
    let water_to_light = test.next().unwrap();
    let light_to_temperature = test.next().unwrap();
    let temperature_to_humidity = test.next().unwrap();
    let humidity_to_location = test.next().unwrap();

    let lowest_location = seed_ranges
        .par_iter()
        .map(|seed_range| {
            seed_range
                .clone()
                .map(|seed| {
                    if seed % 1000000 == 0 {
                        dbg!(seed);
                    }

                    humidity_to_location.get_destination_by_source(
                        temperature_to_humidity.get_destination_by_source(
                            light_to_temperature.get_destination_by_source(
                                water_to_light.get_destination_by_source(
                                    fertilizer_to_water.get_destination_by_source(
                                        soil_to_fertilizer.get_destination_by_source(
                                            seed_to_soil.get_destination_by_source(seed),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    )
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
