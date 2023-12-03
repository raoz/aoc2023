use std::{fs, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
enum SchematicPart {
    Number { index: usize },
    Dot,
    Symbol(char),
}

#[derive(Debug)]
struct Schematic {
    grid: Vec<Vec<SchematicPart>>,
    numbers: Vec<u32>,
}

impl FromStr for Schematic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = vec![];
        let mut grid = vec![];
        for line in s.lines() {
            let mut grid_line = vec![];
            let mut current_number = None;
            for char in line.chars() {
                if let Some(digit) = char.to_digit(10) {
                    if let Some(number) = current_number {
                        current_number = Some(number * 10 + digit);
                    } else {
                        current_number = Some(digit);
                    }
                    grid_line.push(SchematicPart::Number {
                        index: numbers.len(),
                    });
                } else {
                    if let Some(number) = current_number {
                        numbers.push(number);
                        current_number = None;
                    }
                    grid_line.push(match char {
                        '.' => SchematicPart::Dot,
                        x => SchematicPart::Symbol(x),
                    })
                }
            }
            if let Some(number) = current_number {
                numbers.push(number);
            }
            grid.push(grid_line);
        }
        Ok(Schematic { grid, numbers })
    }
}

fn part_one(schematic: &Schematic) -> u32 {
    let mut result = 0;
    let mut last_added = -1;
    for (y, line) in schematic.grid.iter().enumerate() {
        for (x, part) in line.iter().enumerate() {
            if let SchematicPart::Number { index } = part {
                let index = *index;
                if index as i32 <= last_added {
                    continue;
                }

                let x = x as i32;
                let y = y as i32;

                'outer: for x2 in x - 1..=x + 1 {
                    for y2 in y - 1..=y + 1 {
                        if x2 < 0 || y2 < 0 {
                            continue;
                        }
                        let x2 = x2 as usize;
                        let y2 = y2 as usize;
                        let part = schematic.grid.get(y2).and_then(|line| line.get(x2));
                        if let Some(SchematicPart::Symbol(_)) = part {
                            last_added = index as i32;
                            result += schematic.numbers[index];
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
    result
}

fn part_two(schematic: &Schematic) -> u32 {
    let mut result = 0;
    for (y, line) in schematic.grid.iter().enumerate() {
        for (x, part) in line.iter().enumerate() {
            if *part == SchematicPart::Symbol('*') {
                let x = x as i32;
                let y = y as i32;

                let mut indices = vec![];

                for x2 in x - 1..=x + 1 {
                    for y2 in y - 1..=y + 1 {
                        if x2 < 0 || y2 < 0 {
                            continue;
                        }
                        let x2 = x2 as usize;
                        let y2 = y2 as usize;
                        let part = schematic.grid.get(y2).and_then(|line| line.get(x2));
                        if let Some(SchematicPart::Number { index }) = part {
                            indices.push(index);
                        }
                    }
                }
                indices.sort();
                indices.dedup();
                let numbers = indices
                    .iter()
                    .map(|index| schematic.numbers[**index])
                    .collect::<Vec<_>>();

                if numbers.len() == 2 {
                    // is a gear
                    result += numbers[0] * numbers[1];
                }
            }
        }
    }
    result
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let schematic: Schematic = input.parse().unwrap();
    println!("Part one: {}", part_one(&schematic));
    println!("Part two: {}", part_two(&schematic));
}
