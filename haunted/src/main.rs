use std::{collections::HashMap, fs};

use gcd::Gcd;

struct Map {
    steps: Vec<Direction>,
    locations: Vec<String>,
    map: HashMap<(String, Direction), String>,
}

const SRC: &str = "AAA";
const DEST: &str = "ZZZ";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
}

fn parse_input(input: &[&str]) -> Map {
    let steps = input[0]
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        })
        .collect();

    let mut locations = vec![];
    let mut map = HashMap::new();
    for line in input.iter().skip(2) {
        let (key, pair) = line.split_once(" = ").unwrap();
        let (left, right) = pair
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split_once(", ")
            .unwrap();
        locations.push(key.to_owned());
        map.insert((key.to_owned(), Direction::Left), left.to_owned());
        map.insert((key.to_owned(), Direction::Right), right.to_owned());
    }
    Map {
        steps,
        locations,
        map,
    }
}

fn iter_locations<'a>(
    start: &str,
    steps: &'a [Direction],
    map: &'a HashMap<(String, Direction), String>,
) -> impl Iterator<Item = String> + 'a {
    steps
        .iter()
        .cycle()
        .scan(start.to_owned(), |location, &dir| {
            let next_location = map.get(&((*location).to_string(), dir)).unwrap().clone();
            *location = next_location;
            Some(location.clone())
        })
}

struct Cycle {
    first_finish: u64,
    cycle: u64,
}

impl Cycle {
    fn combine(&self, other: &Cycle) -> Cycle {
        let finish_difference =
            (self.first_finish as i64 - other.first_finish as i64).unsigned_abs();
        let gcd = self.cycle.gcd(other.cycle);
        assert!(finish_difference % gcd == 0, "No solution");
        (0..)
            .map(|x| self.first_finish + x * self.cycle)
            .find(|x| *x >= other.first_finish && (x - other.first_finish) % other.cycle == 0)
            .map(|x| Cycle {
                first_finish: x,
                cycle: self.cycle * other.cycle / gcd,
            })
            .unwrap()
    }
}

fn find_cycle(
    start: &str,
    steps: &[Direction],
    map: &HashMap<(String, Direction), String>,
) -> Cycle {
    let first_finish = iter_locations(start, steps, map)
        .position(|location| location.ends_with('Z'))
        .unwrap()
        + 1;
    let cycle = iter_locations(start, steps, map)
        .skip(first_finish)
        .position(|location| location.ends_with('Z'))
        .unwrap() as u64
        + 1;
    Cycle {
        first_finish: first_finish as u64,
        cycle,
    }
}

fn part_one(input: &[&str]) -> u64 {
    let Map {
        steps,
        locations: _,
        map,
    } = parse_input(input);

    iter_locations(SRC, &steps, &map)
        .take_while(|location| location != DEST)
        .count() as u64
        + 1
}

fn part_two(input: &[&str]) -> u64 {
    let Map {
        steps,
        locations,
        map,
    } = parse_input(input);
    let starting_nodes = locations
        .iter()
        .filter(|location| location.ends_with('A'))
        .cloned()
        .collect::<Vec<_>>();

    let cycles = starting_nodes
        .iter()
        .map(|start| find_cycle(start, &steps, &map))
        .collect::<Vec<_>>();

    let common_cycle = cycles.iter().fold(
        Cycle {
            first_finish: 1,
            cycle: 1,
        },
        |acc, x| acc.combine(x),
    );
    common_cycle.first_finish
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.lines().collect::<Vec<_>>();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = vec![
            "RL",
            "",
            "AAA = (BBB, CCC)",
            "BBB = (DDD, EEE)",
            "CCC = (ZZZ, GGG)",
            "DDD = (DDD, DDD)",
            "EEE = (EEE, EEE)",
            "GGG = (GGG, GGG)",
            "ZZZ = (ZZZ, ZZZ)",
        ];
        assert_eq!(part_one(&input), 2);
    }
    #[test]
    fn test_part_one_2() {
        let input = vec![
            "LLR",
            "",
            "AAA = (BBB, BBB)",
            "BBB = (AAA, ZZZ)",
            "ZZZ = (ZZZ, ZZZ)",
        ];
        assert_eq!(part_one(&input), 6);
    }

    #[test]
    fn test_part_two() {
        let input = vec![
            "LR",
            "",
            "11A = (11B, XXX)",
            "11B = (XXX, 11Z)",
            "11Z = (11B, XXX)",
            "22A = (22B, XXX)",
            "22B = (22C, 22C)",
            "22C = (22Z, 22Z)",
            "22Z = (22B, 22B)",
            "XXX = (XXX, XXX)",
        ];
        assert_eq!(part_two(&input), 6);
    }
}
