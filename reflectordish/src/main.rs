use std::{fs, hash};

fn perform_shift(input: &[String]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut support_levels = input[0].chars().map(|c| -1).collect::<Vec<i64>>();
    for (i, line) in input.iter().enumerate() {
        let mut new_line = String::new();
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    support_levels[j] = i as i64;
                    new_line.push('#');
                },
                '.' => {
                    new_line.push('.');
                },
                'O' => {
                    support_levels[j] += 1;
                    if support_levels[j] as usize == i {
                        new_line.push('O');
                    } else {
                        new_line.push('.');
                        unsafe{
                            result[support_levels[j] as usize].as_bytes_mut()[j] = b'O';
                        }
                    }
                    
                },
                _ => panic!("Unknown character {}", c),
            }
        }
        result.push(new_line);
    }
    result
}

fn rotate_90(input: &[String]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for (i, line) in input.iter().enumerate() {
        let mut new_line = String::new();
        for (j, c) in line.chars().enumerate() {
            new_line.push(input[input.len() - j - 1].chars().nth(i).unwrap());
        }
        result.push(new_line);
    }
    result
}

fn calculate_load_level(input: &[String]) -> u64 {
    let mut total_load = 0;
    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' | '.' => {},
                'O' => {
                    total_load += input.len() as u64 - i as u64;
                },
                _ => panic!("Unknown character {}", c),
            }
        }

    }
    total_load
}

fn part_one(input: &[&str]) -> u64 {
    // let mut total_load = 0;
    // let mut support_levels = input[0].chars().map(|c| -1).collect::<Vec<i64>>();
    // for (i, line) in input.iter().enumerate() {
    //     for (j, c) in line.chars().enumerate() {
    //         match c {
    //             '#' => {
    //                 support_levels[j] = i as i64;
    //             },
    //             '.' => {},
    //             'O' => {
    //                 support_levels[j] += 1;
    //                 total_load += input.len() as u64 - support_levels[j] as u64;
    //             },
    //             _ => panic!("Unknown character {}", c),
    //         }
    //     }

    // }
    // total_load
    let mut shifted = perform_shift(input.iter().map(|s| s.to_string()).collect::<Vec<String>>().as_slice());
    calculate_load_level(&shifted)
}

fn perform_cycle(input: &[String]) -> Vec<String> {
    let mut result = perform_shift(input);
    result = rotate_90(&result);
    result = perform_shift(&result);
    result = rotate_90(&result);
    result = perform_shift(&result);
    result = rotate_90(&result);
    result = perform_shift(&result);
    result = rotate_90(&result);
    result
}

fn part_two(input: &[&str]) -> u64 {
    let mut hash_map = std::collections::HashMap::new();
    let mut intermediate_value = input.iter().map(|s| s.to_string()).collect::<Vec<String>>();
    hash_map.insert(intermediate_value.join(""), 0);

    for iter in 1..=1000000 {
        intermediate_value = perform_cycle(&intermediate_value);
        let existing = hash_map.insert(intermediate_value.join(""), iter);
        if let Some(previous_iter) = existing {
            let loop_lenght = iter - previous_iter;
            let remaining_cycles = (1000000000 - iter) % loop_lenght;
            for _ in 0..remaining_cycles {
                intermediate_value = perform_cycle(&intermediate_value);
            }
            return calculate_load_level(&intermediate_value);
        }
    }
    unreachable!();
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.lines().collect();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &[&str] = &[
"O....#....",
"O.OO#....#",
".....##...",
"OO.#O....O",
".O.....O#.",
"O.#..O.#.#",
"..O..#O..O",
".......O..",
"#....###..",
"#OO..#....",
    ];

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 136);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 64);
    }
}