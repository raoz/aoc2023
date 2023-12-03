use std::fs::File;
use std::io::{self, BufRead};

fn part_one(line: &str) -> u32 {
    let digits = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();
    let number = digits.first().unwrap() * 10 + digits.last().unwrap();
    number
}

fn part_two(line: &str) -> u32 {
    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digits = line
        .chars()
        .enumerate()
        .filter_map(|(idx, c)| {
            c.to_digit(10).or_else(|| {
                let substr = &line[idx..];
                numbers
                    .iter()
                    .position(|&s| substr.starts_with(s))
                    .map(|i| u32::try_from(i).unwrap() + 1)
            })
        })
        .collect::<Vec<_>>();
    let number = digits.first().unwrap() * 10 + digits.last().unwrap();
    number
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut sum1 = 0;
    let mut sum2 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        sum1 += part_one(&line.clone());
        sum2 += part_two(&line.clone());
    }

    println!("{sum1}");
    println!("{sum2}");
}
