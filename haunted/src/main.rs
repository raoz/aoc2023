use std::{collections::HashMap, fs};

const SRC: &'static str = "AAA";
const DEST: &'static str = "ZZZ";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
}

fn parse_input(input: &[&str]) -> (Vec<Direction>, Vec<String>, HashMap<(String, Direction), String>) {
    let steps = input[0].chars().map(|c| match c{
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("Invalid direction"),
    }).collect();


    let mut locations = vec![];
    let mut map = HashMap::new();
    for line in input.iter().skip(2) {
        let (key, pair) = line.split_once(" = ").unwrap();
        let (left, right) = pair.strip_prefix('(').unwrap().strip_suffix(')').unwrap().split_once(", ").unwrap();
        locations.push(key.to_owned());
        map.insert((key.to_owned(), Direction::Left), left.to_owned());
        map.insert((key.to_owned(), Direction::Right), right.to_owned());
    }
    (steps, locations, map)
}

fn iter_locations<'a>(start: &str, steps: &'a [Direction], map: &'a HashMap<(String, Direction), String>) -> impl Iterator<Item = String> + 'a {
    steps.iter().cycle().scan(start.to_owned(), |location, &dir| {
        let next_location = map.get(&(location.to_string(), dir)).unwrap().to_owned();
        *location = next_location;
        Some(location.clone())
    })
}

fn iter_finishes(start: &str, steps: &[Direction], map: &HashMap<(String, Direction), String>) -> impl Iterator<Item = u64> {
    let first_finish = iter_locations(start, steps, map).position(|location| location.ends_with('Z')).unwrap() as u64 + 1;
    let cycle = iter_locations(start, steps, map).skip(first_finish as usize).position(|location| location.ends_with('Z')).unwrap() as u64 + 1;
    (0..).map(move |i| first_finish + i * cycle)
}

fn part_one(input: &[&str]) -> u64 {
    let (steps,_,  map) = parse_input(input);

    iter_locations(SRC, &steps, &map).take_while(|location| location != DEST).count() as u64 + 1
}

fn part_two(input: &[&str]) -> u64 {
    let (steps, locations,  map) = parse_input(input);
    let starting_nodes = locations.iter().filter(|location| location.ends_with('A')).cloned().collect::<Vec<_>>();
    let mut finishes_streams = starting_nodes.clone().into_iter().map(|location| iter_finishes(&location,&steps, &map)).collect::<Vec<_>>();

    let mut current_pos = 0;
    let mut aligned_count = 0;
    loop {
        for stream in finishes_streams.iter_mut() {
            let mut next_pos = stream.next().unwrap();
            while next_pos < current_pos {
                next_pos = stream.next().unwrap();
            }
            if next_pos == current_pos {
                aligned_count += 1;
                if aligned_count == starting_nodes.len() {
                    return current_pos;
                }
            } else {
                aligned_count = 1;
                current_pos = next_pos;
            }
        }
    }
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