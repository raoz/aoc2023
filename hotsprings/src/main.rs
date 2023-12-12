use std::{fs, iter::repeat};

fn test_bitsets(n: usize, func: impl Fn(&[bool]) -> bool) -> usize {
    let mut count = 0;
    let mut bitset = vec![false; n];
    loop {
        if func(&bitset) {
            count += 1
        }
        let mut i = 0;
        while i < n && bitset[i] {
            bitset[i] = false;
            i += 1;
        }
        if i == n {
            break;
        }
        bitset[i] = true;
    }
    count
}

fn count_options(records: &str, correct_counts: Vec<usize>) -> usize {
    let question_marks = records.chars().filter(|c| *c == '?').count();
    test_bitsets(question_marks, |bitset| {
        let mut counts = vec![];
        let mut current_count = 0;
        let mut current_bitset_index = 0;
        for c in records.chars() {
            let c = match c {
                '?' => {
                    current_bitset_index += 1;
                    if bitset[current_bitset_index - 1] {
                        '#'
                    } else {
                        '.'
                    }
                }
                x => x,
            };

            if c == '#' {
                current_count += 1;
            } else {
                if current_count > 0 {
                    counts.push(current_count);
                    if counts.len() > correct_counts.len() {
                        return false;
                    }
                }
                current_count = 0;
            }
        }
        if current_count > 0 {
            counts.push(current_count);
        }
        counts == correct_counts
    })
}

fn part_one(input: &[&str]) -> usize {
    let mut total = 0;
    for line in input {
        let (records, correct_counts) = line.split_once(' ').unwrap();
        let correct_counts = correct_counts
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let result = count_options(records, correct_counts);
        total += result;
        println!("{}: {}", line, result)
    }
    total
}

fn part_two(input: &[&str]) -> usize {
    let mut total = 0;
    for line in input {
        let (records, correct_counts) = line.split_once(' ').unwrap();
        let correct_counts = correct_counts
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let records = repeat(records).take(5).collect::<Vec<_>>().join("?");
        let correct_counts = repeat(correct_counts).take(5).flatten().collect::<Vec<_>>();
        let result = count_options(&records, correct_counts);
        total += result;
        println!("{}: {}", line, result)
    }
    total
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

    const INPUT: &[&str] = &[
        "???.### 1,1,3",
        ".??..??...?##. 1,1,3",
        "?#?#?#?#?#?#?#? 1,3,1,6",
        "????.#...#... 4,1,1",
        "????.######..#####. 1,6,5",
        "?###???????? 3,2,1",
    ];

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 21);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 525152);
    }
}
