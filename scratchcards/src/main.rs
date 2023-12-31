use std::{collections::HashSet, fs, str::FromStr};

struct Scratchcard {
    winning: HashSet<u32>,
    has: HashSet<u32>,
}

impl FromStr for Scratchcard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_header, card) = s.split_once(": ").unwrap();
        let (winning, has) = card.split_once(" | ").unwrap();
        let card = Scratchcard {
            winning: winning
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect(),
            has: has
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect(),
        };
        Ok(card)
    }
}

impl Scratchcard {
    fn matches(&self) -> usize {
        self.winning.intersection(&self.has).count()
    }
    fn points(&self) -> u32 {
        match self.matches() {
            0 => 0,
            x => 2u32.pow(u32::try_from(x).unwrap() - 1),
        }
    }
}

fn part_one(input: &[Scratchcard]) -> u32 {
    input.iter().map(Scratchcard::points).sum()
}

fn part_two(input: &[Scratchcard]) -> u32 {
    let mut copy_counts = input.iter().map(|_x| 1).collect::<Vec<_>>();
    for (i, card) in input.iter().enumerate() {
        let matches = card.matches();
        let copies = copy_counts[i];
        for copy_count in copy_counts.iter_mut().skip(i + 1).take(matches) {
            *copy_count += copies;
        }
    }
    copy_counts.iter().sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    let cards: Vec<Scratchcard> = lines.iter().map(|x| x.parse().unwrap()).collect();
    println!("Part one: {}", part_one(&cards));
    println!("Part two: {}", part_two(&cards));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str; 6] = &[
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    ];

    #[test]
    fn test_part_one() {
        let result = part_one(
            INPUT
                .iter()
                .map(|x| x.parse::<Scratchcard>().unwrap())
                .collect::<Vec<_>>()
                .as_slice(),
        );
        assert_eq!(result, 13);
    }
    #[test]
    fn test_part_two() {
        let result = part_two(
            INPUT
                .iter()
                .map(|x| x.parse::<Scratchcard>().unwrap())
                .collect::<Vec<_>>()
                .as_slice(),
        );
        assert_eq!(result, 30);
    }
}
