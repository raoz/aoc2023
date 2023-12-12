use std::{fs, iter::repeat, collections::HashMap};


fn count_options(records: &str, correct_counts: &[usize], cache: &mut HashMap<(usize, usize), usize>) -> usize {
    let cache_key = (records.len(), correct_counts.len());
    if let Some(val) = cache.get(&cache_key) {
        return *val;
    }
    if correct_counts.is_empty() {
        return if records.chars().all(|c| c != '#') { 1 } else { 0 };
    }
    let needed_records = correct_counts.iter().sum::<usize>() + correct_counts.len() - 1;
    if records.len() < needed_records {
        return 0;
    }
    let mut ways = 0;
    let mut current_count = 0;
    for (i, c) in records.chars().enumerate() {
        match c {
            '#' => {
                current_count += 1;
            }
            '.' => {
                if current_count == correct_counts[0] {
                    let res = ways + count_options(&records[i+1..], &correct_counts[1..], cache);
                    cache.insert(cache_key, res);
                    return res;
                }
                if current_count != 0 {
                    cache.insert(cache_key, ways);
                    return ways;
                }
            }
            '?' => {
                if current_count == 0 {
                    ways += count_options(&records[i+1..], correct_counts, cache);
                }
                if current_count == correct_counts[0] {
                    ways += count_options(&records[i+1..], &correct_counts[1..],cache);
                }
                current_count += 1;
            }
            _ => unreachable!(),
        }
        
        if current_count > correct_counts[0] {
            cache.insert(cache_key, ways);
            return ways;
        }
    }
    if [current_count] == correct_counts {
        cache.insert(cache_key, ways + 1);
        return ways + 1;
    }
    cache.insert(cache_key, ways);
    return ways;
}

fn part_one(input: &[&str]) -> usize {
    let mut total = 0;
    for line in input {
        let (records, correct_counts) = line.split_once(' ').unwrap();
        let correct_counts = correct_counts
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let mut cache = HashMap::new();
        let result = count_options(records, &correct_counts, &mut cache);
        total += result;
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
        let mut cache = HashMap::new();
        let result = count_options(&records, &correct_counts, &mut cache);
        total += result;
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
    fn test_simple() {
        assert_eq!(count_options("??", &[1], &mut HashMap::new()), 2);
        assert_eq!(count_options("???", &[1], &mut HashMap::new()), 3);
        assert_eq!(count_options("?#?", &[1], &mut HashMap::new()), 1);
        assert_eq!(count_options("???", &[1, 1], &mut HashMap::new()), 1);
        assert_eq!(count_options("????", &[1, 2], &mut HashMap::new()), 1);
        assert_eq!(count_options("??.??", &[1, 2], &mut HashMap::new()), 2);
        assert_eq!(count_options("???.??", &[1, 2], &mut HashMap::new()), 3);
        assert_eq!(count_options("???.???", &[1, 2], &mut HashMap::new()), 6);
    }

    #[test]
    fn test_tricky() {
        assert_eq!(count_options("??????.??#.", &[2,3], &mut HashMap::new()), 5)
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 21);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 525152);
    }
}
