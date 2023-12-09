use std::fs;

fn get_derivative(values: Vec<i64>) -> Vec<i64> {
    values.windows(2).map(|x| x[1] - x[0]).collect::<Vec<_>>()
}

fn get_next_value(values: Vec<i64>) -> i64 {
    if values.iter().all(|&x| x == values[0]) {
        return values[0];
    }
    println!("{:?}", get_derivative(values.clone()));
    *values.last().unwrap() + get_next_value(get_derivative(values))
}

fn get_previous_value(values: Vec<i64>) -> i64 {
    if values.iter().all(|&x| x == values[0]) {
        return values[0];
    }
    *values.first().unwrap() - get_previous_value(get_derivative(values))
}

fn part_one(input: &[&str]) -> i64 {
    let mut total = 0;

    for line in input {
        let values = line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
        total += get_next_value(values);
    }

    total
}
fn part_two(input: &[&str]) -> i64 {
    let mut total = 0;

    for line in input {
        let values = line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
        total += get_previous_value(values);
    }

    total
}
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    println!("Part one: {}", part_one(&lines));
    println!("Part two: {}", part_two(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 114);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 2);
    }
}