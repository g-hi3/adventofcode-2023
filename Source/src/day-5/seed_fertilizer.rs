use rayon::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Almanac {
    seeds: Vec<Seed>,
    maps: Vec<Map>
}

impl Almanac {
    pub fn new(str: &str) -> Option<Self> {
        let mut lines = str.lines();
        let seeds = extract_seeds(lines.next()?);

        let mut maps = Vec::<Map>::new();
        let mut current_map = String::new();
        for line in lines {
            if line.is_empty() {
                match Map::new(&current_map) {
                    None => println!("\"{current_map}\" produced invalid map!"),
                    Some(map) => maps.push(map)
                }
                current_map = String::new();
            } else {
                current_map += line;
                current_map += "\n";
            }
        }

        if let Some(map) = Map::new(&current_map) {
            maps.push(map);
        }

        Some(Self {
            seeds,
            maps
        })
    }

    pub fn find_lowest_location(self: &Self) {
        (&self.seeds).into_par_iter().for_each(|seed| {
            let mut min_location = Option::<u64>::None;
            println!("\nRunning seed {} -> {}", seed.range_start, seed.range_end());
            for seed in seed.range_start..=seed.range_end() {
                let location = self.transform(seed, "seed", "location");
                match location {
                    None => {}
                    Some(location) => match min_location {
                        None => min_location = Some(location),
                        Some(min_l) => if min_l > location {
                            min_location = Some(location);
                            println!("New min location: {min_l}");
                        }
                    }
                }
            }

            println!("Lowest location for seed {} -> {} was {min_location:?}", seed.range_start, seed.range_end());
        });
    }

    fn transform(self: &Self, input: u64, input_name: &str, output_name: &str) -> Option<u64> {
        let mut current_value = input;
        let mut current_name = &String::from(input_name);
        let output_name = &String::from(output_name);

        loop {
            let map = self.maps
                .iter()
                .find(|map| &map.source_name == current_name)?;
            current_value = map.convert(current_value);
            current_name = &map.destination_name;

            if current_name == output_name {
                return Some(current_value);
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Seed {
    range_start: u64,
    length: u64
}

impl Seed {
    fn new(str: &str) -> Option<Self> {
        let mut parts = str
            .split(' ')
            .filter_map(|part| part.parse::<u64>().ok())
            .collect::<Vec<u64>>();
        let range_start = *parts.get(0)?;
        let length = *parts.get(1)?;
        Some(Self {
            range_start,
            length
        })
    }

    fn range_end(self: &Self) -> u64 {
        self.range_start + self.length
    }
}

fn extract_seeds(line: &str) -> Vec<Seed> {
    let mut parts = line[7..]
        .split(' ')
        .filter_map(|part| part.parse::<u64>().ok());
    let mut range_start = 0;
    let mut length = 0;
    let mut next_part_is_length = false;
    let mut seeds = Vec::<Seed>::new();

    for part in parts {
        if next_part_is_length {
            length = part;
            seeds.push(Seed {
                range_start,
                length
            })
        } else {
            range_start = part;
        }
        next_part_is_length = !next_part_is_length;
    }

    seeds
}

#[derive(Debug, PartialEq)]
struct Map {
    source_name: String,
    destination_name: String,
    range_transformations: Vec<RangeTransformation>
}

impl Map {
    fn new(str: &str) -> Option<Self> {
        let mut lines = str.lines();
        let title = lines.next()?;
        let space_index = title.find(' ')?;
        let mut title_parts = title[..space_index].split('-');
        let source_name = title_parts.next()?;
        let destination_name = title_parts.last()?;
        let range_transformations = lines
            .filter_map(RangeTransformation::new)
            .collect::<Vec<RangeTransformation>>();
        Some(Self {
            source_name: String::from(source_name),
            destination_name: String::from(destination_name),
            range_transformations
        })
    }

    fn convert(self: &Self, input: u64) -> u64 {
        for range_transformer in &self.range_transformations {
            if input >= range_transformer.source_range_start
                && input <= range_transformer.source_range_end() {
                return input - range_transformer.source_range_start + range_transformer.destination_range_start
            }
        }

        input
    }
}

#[derive(Debug, PartialEq)]
struct RangeTransformation {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64
}

impl RangeTransformation {
    fn new(line: &str) -> Option<Self> {
        let parts = line
            .split(' ')
            .filter_map(|part| part.parse::<u64>().ok())
            .collect::<Vec<u64>>();
        let destination_range_start = *parts.get(0)?;
        let source_range_start = *parts.get(1)?;
        let range_length = *parts.get(2)?;
        Some(Self {
            destination_range_start,
            source_range_start,
            range_length
        })
    }

    fn source_range_end(self: &Self) -> u64 {
        self.source_range_start + self.range_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_transformation_new() {
        let range_transformation = RangeTransformation::new("52 86 4");
        assert_eq!(range_transformation, Some(RangeTransformation {
            destination_range_start: 52,
            source_range_start: 86,
            range_length: 4
        }));
    }

    #[test]
    fn test_map_new() {
        let map = Map::new("seed-to-soil map:
50 98 2
52 50 48");
        assert_eq!(map, Some(Map {
            source_name: String::from("seed"),
            destination_name: String::from("soil"),
            range_transformations: vec![
                RangeTransformation::new("50 98 2").unwrap(),
                RangeTransformation::new("52 50 48").unwrap()
            ]
        }));
    }

    #[test]
    fn test_almanac_new() {
        let almanac = Almanac::new("seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4");
        assert_eq!(almanac, Some(Almanac {
            seeds: vec![
                Seed { range_start: 79, length: 14},
                Seed { range_start: 55, length: 13 }],
            maps: vec![
                Map::new("seed-to-soil map:
50 98 2
52 50 48").unwrap(),
                Map::new("soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15").unwrap(),
                Map::new("fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4").unwrap(),
                Map::new("water-to-light map:
88 18 7
18 25 70").unwrap(),
                Map::new("light-to-temperature map:
45 77 23
81 45 19
68 64 13").unwrap(),
                Map::new("temperature-to-humidity map:
0 69 1
1 0 69").unwrap(),
                Map::new("humidity-to-location map:
60 56 37
56 93 4").unwrap()]
        }));
    }

    #[test]
    fn test_convert() {
        let map = Map::new("seed-to-soil map:
50 98 2
52 50 48").unwrap();
        assert_eq!(map.convert(0), 0);
        assert_eq!(map.convert(1), 1);
        assert_eq!(map.convert(48), 48);
        assert_eq!(map.convert(49), 49);
        assert_eq!(map.convert(50), 52);
        assert_eq!(map.convert(51), 53);
        assert_eq!(map.convert(96), 98);
        assert_eq!(map.convert(97), 99);
        assert_eq!(map.convert(98), 50);
        assert_eq!(map.convert(99), 51);
        assert_eq!(map.convert(79), 81);
        assert_eq!(map.convert(14), 14);
        assert_eq!(map.convert(55), 57);
        assert_eq!(map.convert(13), 13);
    }

    #[test]
    fn test_transform() {
        let almanac = Almanac::new("seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4").unwrap();
        assert_eq!(almanac.transform(79, "seed", "location"), Some(82));
        assert_eq!(almanac.transform(14, "seed", "location"), Some(43));
        assert_eq!(almanac.transform(55, "seed", "location"), Some(86));
        assert_eq!(almanac.transform(13, "seed", "location"), Some(35));
    }

    #[test]
    fn test_seed_new() {
        let seed = Seed::new("45 16");
        assert_eq!(seed, Some(Seed {
            range_start: 45,
            length: 16
        }))
    }

    #[test]
    fn test_extract_seeds() {
        let seeds = extract_seeds("seeds: 79 14 55 13");
        assert_eq!(seeds, vec![
            Seed { range_start: 79, length: 14 },
            Seed { range_start: 55, length: 13 }
        ])
    }
}