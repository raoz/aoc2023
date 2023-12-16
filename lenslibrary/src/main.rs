use std::{fs, str::FromStr};

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: u64,
}

impl Lens {
    fn focusing_power(&self, box_nr: usize, slot_ix: usize) -> u64 {
        self.focal_length * (box_nr as u64 + 1) * (slot_ix as u64 + 1)
    }
}

#[derive(Debug)]
enum Operation {
    Equals(String, u64),
    Dash(String),
}


impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with("-") {
            Ok(Operation::Dash(s[..s.len() - 1].to_string()))
        } else {
            let (key, value) = s.split_once('=').ok_or(())?;
            Ok(Operation::Equals(key.to_string(), value.parse().map_err(|_| ())?))
        }
    }
}

impl Operation {
    fn get_hash(&self) -> u64 {
        match self {
            Operation::Equals(key, _) => hash_string(&key),
            Operation::Dash(key) => hash_string(&key),
        }
    }
}

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

fn part_two(input: &str) -> u64 {
    let ops: Vec<Operation> = input
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut boxes: Vec<Vec<Lens>> = vec![];
    for _ in 0..256 {
        boxes.push(vec![]);
    }

    for op in ops {
        let hash = op.get_hash();
        let current_box = &mut boxes[hash as usize];
        match op {
            Operation::Equals(key, value) => {
                if let Some(existing) = current_box.iter_mut().find(|lens| lens.label == key) {
                    existing.focal_length = value;
                } else {
                    current_box.push(Lens {
                        label: key,
                        focal_length: value,
                    });
                }
            }
            Operation::Dash(key) => {
                current_box.retain(|lens| lens.label != key);
            }
        }
    }

    let mut total_power = 0;
    for (box_nr, lenses) in boxes.iter().enumerate() {
        for (slot_ix, lens) in lenses.iter().enumerate() {
            total_power += lens.focusing_power(box_nr, slot_ix);
        }
    }
    total_power
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
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

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE_INPUT), 145);
    }
}
