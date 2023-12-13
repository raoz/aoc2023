use std::fs;

fn get_line_bitmap(line: &str) -> u64 {
    let mut bitmap = 0;
    for c in line.chars() {
        bitmap <<= 1;
        if c == '#' {
            bitmap |= 1;
        }
    }
    bitmap
}

fn get_horizontal_reflection_map(line: &str) -> u64 {
    let forward_bitmap = get_line_bitmap(line);
    let reverse_bitmap = get_line_bitmap(&line.chars().rev().collect::<String>());
    let mut reflection_map = 0;
    let len = line.len();
    for i in 1..len {
        let mask = ((1 << i) - 1) & ((1 << len - i) - 1);
        if (forward_bitmap >> i) & mask == (reverse_bitmap >> (len - i)) & mask {
            reflection_map |= 1 << (len - i);
        }
    }
    reflection_map
}

fn get_reflection_score(lines: &[&str], avoid_score: Option<u32>) -> Option<u32> {
    let line_length = lines[0].len();
    let mut reflection_map = (1 << line_length) - 1;
    for line in lines {
        reflection_map &= get_horizontal_reflection_map(line);
    }
    if let Some(avoid_score) = avoid_score {
        if avoid_score < 100 {
            let mask = (1 << line_length) - 1 ^ (1 << avoid_score);
            reflection_map &= mask;
        }
    }
    if reflection_map != 0 {
        let result = Some(reflection_map.trailing_zeros());
        return result;
    }
    // transpose
    let transposed_lines = (0..line_length)
        .map(|i| {
            lines
                .iter()
                .map(|line| line.chars().nth(i).unwrap())
                .collect::<String>()
        })
        .collect::<Vec<_>>();
    let transposed_line_length = transposed_lines[0].len();
    let mut reflection_map = (1 << transposed_line_length) - 1;
    for line in transposed_lines {
        reflection_map &= get_horizontal_reflection_map(&line);
    }
    if let Some(avoid_score) = avoid_score {
        if avoid_score >= 100 {
            let mask = (1 << transposed_line_length) - 1 ^ (1 << avoid_score / 100);
            reflection_map &= mask;
        }
    }
    if reflection_map == 0 {
        return None;
    }
    let result = Some(reflection_map.trailing_zeros() * 100);
    return result;
}

fn part_one(input: &str) -> u32 {
    let parts = input.split("\n\n").collect::<Vec<&str>>();

    parts
        .iter()
        .map(|p| {
            let lines = p.lines().collect::<Vec<&str>>();
            get_reflection_score(&lines, None).unwrap()
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    let parts = input.split("\n\n").collect::<Vec<&str>>();

    parts
        .iter()
        .map(|p| {
            let score_without_smudge = {
                let lines = p.lines().collect::<Vec<&str>>();
                get_reflection_score(&lines, None).unwrap()
            };
            for i in 0..p.len() {
                let mut desmudged = p.to_string();
                match desmudged.chars().nth(i).unwrap() {
                    '#' => desmudged.replace_range(i..i + 1, "."),
                    '.' => desmudged.replace_range(i..i + 1, "#"),
                    _ => continue,
                }
                let lines = desmudged.lines().collect::<Vec<&str>>();
                if let Some(score) = get_reflection_score(&lines, Some(score_without_smudge)) {
                    return score;
                }
            }
            panic!("No smudge found for {:?}", p);
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflectance_score_1() {
        let input = vec![
            "#.##..##.",
            "..#.##.#.",
            "##......#",
            "##......#",
            "..#.##.#.",
            "..##..##.",
            "#.#.##.#.",
        ];
        assert_eq!(get_reflection_score(&input, None), Some(5));
    }

    #[test]
    fn test_reflectance_score_2() {
        let input = vec![
            "#...##..#",
            "#....#..#",
            "..##..###",
            "#####.##.",
            "#####.##.",
            "..##..###",
            "#....#..#",
        ];
        assert_eq!(get_reflection_score(&input, None), Some(400));
    }

    #[test]
    fn test_part_one() {
        let input = fs::read_to_string("example.txt").unwrap();
        assert_eq!(part_one(&input), 405);
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string("example.txt").unwrap();
        assert_eq!(part_two(&input), 400);
    }

    #[test]
    fn test_tricky_desmudge() {
        let input = vec![
            ".####..", "###..#.", "..#.###", "#.####.", "#.####.", "..#.###", "###..#.", ".####..",
            "...#.#.", "...#.#.", ".####..", "###..#.", "..#.###",
        ];
        assert_eq!(get_reflection_score(&input, None), Some(400));
        assert_eq!(get_reflection_score(&input, Some(400)), Some(900));
    }
}
