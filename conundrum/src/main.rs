use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Color {
    Blue,
    Green,
    Red
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blue" => Ok(Color::Blue),
            "green" => Ok(Color::Green),
            "red" => Ok(Color::Red),
            _ => Err(())
        }
    }
}

fn is_possible(color: &Color, count: u32) -> bool {
    match color {
        Color::Red => count <= 12,
        Color::Green => count <= 13,
        Color::Blue => count <= 14,
    }
}

fn part_one(lines: &Vec<String>) -> u32 {
    let mut total = 0;
    'outer: for line in lines {
        let (game, data) = line.split_once(": ").unwrap();
        let game_number = game.split_once(" ").unwrap().1.parse::<u32>().unwrap();
         for draw in data.split(";") {
            for (count, color) in draw.split(",").map(|x| x.trim().split_once(" ").unwrap()) {
                let count = count.parse::<u32>().unwrap();
                let color = color.parse::<Color>().unwrap();
                if !is_possible(&color, count) {
                    continue 'outer;
                }
            }
        }
        total += game_number;
    }
    total
}

fn part_two(lines: &Vec<String>) -> u32 {
    let mut total = 0;
    for line in lines {
        let (_, data) = line.split_once(": ").unwrap();
        let mut minimums: HashMap<Color, u32> = HashMap::new();
        for draw in data.split(&[';', ',']) {
            let (count, color) = draw.trim().split_once(" ").unwrap();
            let count = count.parse::<u32>().unwrap();
            let color = color.parse::<Color>().unwrap();
            minimums.entry(color).and_modify(|x| {*x = (*x).max(count)}).or_insert(count);
        }
        total += minimums.values().product::<u32>();
    }
    total
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(file).lines().collect::<Result<Vec<_>,_>>().unwrap();
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
