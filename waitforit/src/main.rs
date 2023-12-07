use std::fs;

fn parse_numbers(line: &str) -> Vec<u64> {
    line.split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()
}

fn parse_poorly_kerned_number(line: &str) -> u64 {
    line.split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap()
}

fn winning_ways(time: u64, distance: u64) -> u64 {
    let a = -1.0;
    let b = time as f64;
    let c = -(distance as f64) - 0.1;
    let res1 = (-b + (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
    let res2 = (-b - (b * b - 4.0 * a * c).sqrt()) / (2.0 * a);
    let min_press_time = res1.ceil() as u64;
    let max_press_time = res2.floor() as u64;
    max_press_time - min_press_time + 1
}

fn part_one(lines: &[&str]) -> u64 {
    let times = parse_numbers(lines[0]);
    let distances = parse_numbers(lines[1]);
    let mut result = 1;
    for (time, distance) in times.iter().zip(distances.iter()) {
        result *= winning_ways(*time, *distance);
    }
    result
}

fn part_two(lines: &[&str]) -> u64 {
    let time = parse_poorly_kerned_number(lines[0]);
    let distance = parse_poorly_kerned_number(lines[1]);
    winning_ways(time, distance)
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines().collect::<Vec<_>>();
    println!("Part One: {}", part_one(&lines));
    println!("Part Two: {}", part_two(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[&str; 2] = &["Time:      7  15   30", "Distance:  9  40  200"];

    #[test]
    fn test_day_one() {
        assert_eq!(part_one(EXAMPLE), 288);
    }

    #[test]
    fn test_day_two() {
        assert_eq!(part_two(EXAMPLE), 71503);
    }
}
