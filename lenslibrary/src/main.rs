use std::fs;

fn hash_string(input: &str) -> u64 {
    input
        .as_bytes()
        .iter()
        .filter(|&&c| c != b'\n')
        .fold(0, |acc, &c| ((acc + u64::from(c)) * 17) % 256)
}

fn part_one(input: &str) -> u64 {
    input.split(',').map(hash_string).sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    println!("Part one: {}", part_one(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash() {
        assert_eq!(hash_string("HASH"), 52);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_INPUT), 1320);
    }
}
