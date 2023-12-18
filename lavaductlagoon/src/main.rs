use std::fs;

use regex::Regex;

struct Instruction {
    direction: char,
    distance: i64,
}

impl Instruction {
    fn from_normal_string(input: &str) -> Instruction {
        let re = Regex::new(r"^([RLUD])\W(\d+)").unwrap();
        let captures = re.captures(input).unwrap();
        let direction = captures.get(1).unwrap().as_str().to_string();
        let distance = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
        Instruction {
            direction: direction.chars().next().unwrap(),
            distance,
        }
    }

    fn from_swapped_string(input: &str) -> Instruction {
        let re = Regex::new(r"\(#(.+)\)$").unwrap();
        let captures = re.captures(input).unwrap();
        let hex = captures.get(1).unwrap().as_str();
        // parse hex
        let distance = i64::from_str_radix(&hex[..5], 16).unwrap();
        let direction = match hex.chars().last().unwrap() {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            _ => panic!("Invalid direction"),
        };
        Instruction {
            direction,
            distance,
        }
    }
}

fn parse_contour(input: &str, swap_len_hex: bool) -> Vec<(i64, i64)> {
    let mut contour = vec![(0, 0)];
    let mut current_x = 0;
    let mut current_y = 0;

    let parser = if swap_len_hex {
        Instruction::from_swapped_string
    } else {
        Instruction::from_normal_string
    };

    for instruction in input.lines().map(parser) {
        let Instruction {
            direction,
            distance,
        } = instruction;

        current_x += match direction {
            'R' => distance,
            'L' => -distance,
            _ => 0,
        };
        current_y += match direction {
            'U' => -distance,
            'D' => distance,
            _ => 0,
        };
        contour.push((current_x, current_y));
    }
    assert_eq!(*contour.last().unwrap(), (0, 0));

    contour
}

fn horizontally_intersects(segment: ((i64, i64), (i64, i64)), y: i64) -> bool {
    let ((_, y1), (_, y2)) = segment;
    y1.min(y2) <= y && y < y1.max(y2)
}

fn contour_area(contour: &[(i64, i64)]) -> u64 {
    let segments = contour
        .windows(2)
        .map(|window| (window[0], window[1]))
        .collect::<Vec<_>>();

    let vert_segments = segments
        .iter()
        .filter(|((x1, _), (x2, _))| x1 == x2)
        .copied()
        .collect::<Vec<_>>();
    let hor_segments = segments
        .iter()
        .filter(|((_, y1), (_, y2))| y1 == y2)
        .copied()
        .collect::<Vec<_>>();

    let min_y = *contour.iter().map(|(_, y)| y).min().unwrap();
    let max_y = *contour.iter().map(|(_, y)| y).max().unwrap();

    let mut total_area = 0;

    for y in min_y..=max_y {
        let mut prev_point = 0;
        let mut inside = false;

        let mut xs = vert_segments
            .iter()
            .filter(|&&v| horizontally_intersects(v, y))
            .map(|((x, _), _)| *x)
            .collect::<Vec<_>>();
        xs.sort_unstable();

        let horizontal_walls = hor_segments
            .iter()
            .filter(|&&h| h.0 .1 == y)
            .map(|((x1, _), (x2, _))| (*x1.min(x2), *x1.max(x2)))
            .collect::<Vec<_>>();

        total_area += horizontal_walls
            .iter()
            .map(|(x1, x2)| -> u64 { (x2 - x1 + 1).try_into().unwrap() })
            .sum::<u64>();

        for x in xs {
            if inside {
                let segment_area: u64 = (x - prev_point + 1).try_into().unwrap();
                total_area += segment_area;
                let contained_walls = horizontal_walls
                    .iter()
                    .filter(|(x1, x2)| *x1 <= x && *x2 >= prev_point);
                for (x1, x2) in contained_walls {
                    let a = (*x1).max(prev_point);
                    let b = (*x2).min(x);
                    let double_counted_area: u64 = (b - a + 1).try_into().unwrap();
                    total_area -= double_counted_area;
                }
            }
            inside = !inside;
            prev_point = x;
        }
        assert!(!inside);
    }
    total_area
}

fn part_one(input: &str) -> u64 {
    let contour = parse_contour(input, false);
    contour_area(&contour)
}

fn part_two(input: &str) -> u64 {
    let contour = parse_contour(input, true);
    contour_area(&contour)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 62);
    }

    #[test]
    fn test_simple() {
        assert_eq!(
            part_one(
                r#"R 3 (#70c710)
D 3 (#5713f0)
L 3 (#ffffff)
U 3 (#aaaaaa)"#
            ),
            16
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 952_408_144_115);
    }
}
