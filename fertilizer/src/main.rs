use std::{fs, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Entry {
    src_start: u64,
    dst_start: u64,
    length: u64,
}

impl Entry {
    fn map(&self, value: u64) -> u64 {
        if value < self.src_start || value >= self.src_start + self.length {
            return value;
        }
        let offset = value - self.src_start;
        self.dst_start + offset
    }
    fn map_reverse(&self, value: u64) -> u64 {
        if value < self.dst_start || value >= self.dst_start + self.length {
            return value;
        }
        let offset = value - self.dst_start;
        self.src_start + offset
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.src_start.partial_cmp(&other.src_start)
    }
}
impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.src_start.cmp(&other.src_start)
    }
}

#[derive(Debug, Clone)]
struct Map {
    entries: Vec<Entry>,
    rev_entries: Vec<Entry>,
}

impl Map {
    fn new(mut entries: Vec<Entry>) -> Self {
        entries.sort();
        let mut rev_entries = entries.clone();
        rev_entries.sort_by(|a, b| a.dst_start.cmp(&b.dst_start));
        Self {
            entries,
            rev_entries,
        }
    }
    fn find_entry(&self, value: u64) -> Option<&Entry> {
        let search_result = self.entries.binary_search_by(|entry| {
            if entry.src_start > value {
                std::cmp::Ordering::Greater
            } else if entry.src_start + entry.length <= value {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });
        if let Ok(index) = search_result {
            Some(&self.entries[index])
        } else {
            None
        }
    }

    fn find_rev_entry(&self, value: u64) -> Option<&Entry> {
        let search_result = self.rev_entries.binary_search_by(|entry| {
            if entry.dst_start > value {
                std::cmp::Ordering::Greater
            } else if entry.dst_start + entry.length <= value {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });
        if let Ok(index) = search_result {
            Some(&self.rev_entries[index])
        } else {
            None
        }
    }

    fn map(&self, value: u64) -> u64 {
        match self.find_entry(value) {
            Some(entry) => entry.map(value),
            None => value,
        }
    }

    fn map_reverse(&self, value: u64) -> u64 {
        let entry = self.find_rev_entry(value);

        match entry {
            Some(entry) => entry.map_reverse(value),
            None => value,
        }
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entries = Vec::new();
        for line in s.lines() {
            let mut parts = line.split_whitespace();
            let dst_start = parts.next().ok_or(())?.parse().map_err(|_| ())?;
            let src_start = parts.next().ok_or(())?.parse().map_err(|_| ())?;
            let length = parts.next().ok_or(())?.parse().map_err(|_| ())?;
            entries.push(Entry {
                src_start,
                dst_start,
                length,
            });
        }
        Ok(Map::new(entries))
    }
}

#[derive(Debug, Clone)]
struct CombinedMap {
    maps: Vec<Map>,
}

impl CombinedMap {
    fn map(&self, value: u64) -> u64 {
        self.maps.iter().fold(value, |acc, map| map.map(acc))
    }

    fn map_reverse(&self, value: u64) -> u64 {
        self.maps
            .iter()
            .rev()
            .fold(value, |acc, map| map.map_reverse(acc))
    }
}

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl FromStr for Almanac {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("\n\n").collect::<Vec<_>>();
        let seeds = parts[0]
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let maps = parts[1..]
            .iter()
            .map(|part| part.split_once(":\n").unwrap().1.parse::<Map>().unwrap())
            .collect::<Vec<_>>();
        Ok(Almanac { seeds, maps })
    }
}

fn part_one(almanac: &Almanac) -> u64 {
    let combined_map = CombinedMap {
        maps: almanac.maps.clone(),
    };
    let locations = almanac
        .seeds
        .iter()
        .map(|seed| combined_map.map(*seed))
        .collect::<Vec<_>>();

    *locations.iter().min().unwrap()
}

fn part_two(almanac: &Almanac) -> u64 {
    let combined_map = CombinedMap {
        maps: almanac.maps.clone(),
    };

    let seed_ranges = almanac
        .seeds
        .chunks(2)
        .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
        .collect::<Vec<_>>();

    for location in 0..u64::MAX {
        let seed = combined_map.map_reverse(location);
        let range = seed_ranges.iter().find(|range| range.contains(&seed));
        if let Some(range) = range {
            println!("seed: {seed} range: {range:?}");
            return location;
        }
    }
    unreachable!()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let almanac = input.parse::<Almanac>().unwrap();
    println!("Part one: {}", part_one(&almanac));
    println!("Part two: {}", part_two(&almanac));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"seeds: 79 14 55 13

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
56 93 4"#;

    #[test]
    fn test_day_one() {
        assert_eq!(part_one(&TEST_INPUT.parse().unwrap()), 35);
    }

    #[test]
    fn test_day_two() {
        assert_eq!(part_two(&TEST_INPUT.parse().unwrap()), 46);
    }
}
