use std::fs;

use grid::Grid;
use regex::Regex;

fn parse_contour(input: &str) -> Vec<(i64, i64)> {
    let mut contour = vec![(0, 0)];
    let mut current_x = 0;
    let mut current_y = 0;
    let re = Regex::new(r"^([RLUD])\W(\d+)\W\(#(.+)\)$").unwrap();

    for line in input.lines() {
        let Some((_, [direction, distance, _color_code])) =
            re.captures(line).map(|caps| caps.extract())
        else {
            panic!("Invalid input: {}", line);
        };

        let distance = distance.parse::<i64>().unwrap();
        current_x += match direction {
            "R" => distance,
            "L" => -distance,
            _ => 0,
        };
        current_y += match direction {
            "U" => -distance,
            "D" => distance,
            _ => 0,
        };
        contour.push((current_x, current_y));
    }
    assert_eq!(*contour.last().unwrap(), (0, 0));

    contour
}

fn in_segment(segment: ((i64, i64), (i64, i64)), point: (i64, i64)) -> bool {
    let ((x1, y1), (x2, y2)) = segment;
    let (x, y) = point;
    if x1 == x2 {
        let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        x == x1 && y1 <= y && y <= y2
    } else {
        let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        y == y1 && x1 <= x && x <= x2
    }
}

fn contour_area(contour: &Vec<(i64, i64)>) -> u64 {
    let segments = contour
        .windows(2)
        .map(|window| (window[0], window[1]))
        .collect::<Vec<_>>();

    let min_x = *contour.iter().map(|(x, _)| x).min().unwrap();
    let min_y = *contour.iter().map(|(_, y)| y).min().unwrap();
    let max_x = *contour.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *contour.iter().map(|(_, y)| y).max().unwrap();

    let mut total_area = 0;

    for y in min_y..=max_y {
        let mut inside = false;
        let mut wall_lenght = 0;
        let mut prev_up = false;
        for x in min_x..=max_x {
            if segments.iter().any(|segment| in_segment(*segment, (x, y))) {
                total_area += 1;
                if wall_lenght == 0 {
                    prev_up = segments
                        .iter()
                        .any(|segment| in_segment(*segment, (x, y - 1)));
                    inside = !inside;
                }
                wall_lenght += 1;
                print!("#");
            } else {
                if wall_lenght > 1 {
                    let up = segments
                        .iter()
                        .any(|segment| in_segment(*segment, (x - 1, y - 1)));
                    if up == prev_up {
                        inside = !inside;
                    }
                }
                total_area += if inside { 1 } else { 0 };
                wall_lenght = 0;
                if inside {
                    print!("+");
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }
    total_area
}

fn part_one(input: &str) -> u64 {
    let contour = parse_contour(input);
    let min_x = contour.iter().map(|(x, _)| x).min().unwrap();
    let min_y = contour.iter().map(|(_, y)| y).min().unwrap();
    let max_x = contour.iter().map(|(x, _)| x).max().unwrap();
    let max_y = contour.iter().map(|(_, y)| y).max().unwrap();

    let mut map: Grid<bool> = Grid::new(
        (max_y - min_y + 1).try_into().unwrap(),
        (max_x - min_x + 1).try_into().unwrap(),
    );

    for window in contour.windows(2) {
        let [(x1, y1), (x2, y2)] = window else {
            unreachable!()
        };
        let (x1, y1) = (x1 - min_x, y1 - min_y);
        let (x2, y2) = (x2 - min_x, y2 - min_y);
        if x1 == x2 {
            let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            for y in y1..=y2 {
                map[(y.try_into().unwrap(), x1.try_into().unwrap())] = true;
            }
        } else {
            let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            for x in x1..=x2 {
                map[(y1.try_into().unwrap(), x.try_into().unwrap())] = true;
            }
        }
    }

    contour_area(&contour)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    println!("Part one: {}", part_one(&input));
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
        assert_eq!(part_one(TEST_INPUT), 62)
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
}
