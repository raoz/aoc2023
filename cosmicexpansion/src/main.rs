use std::fs;

fn expanded_universe_total_distance(input: &[&str], expansion_factor: usize) -> usize {
    let mut row_empty: Vec<bool> = vec![true; input.len()];
    let mut col_empty: Vec<bool> = vec![true; input[0].len()];
    let mut galaxies = vec![];

    for (y, row) in input.iter().enumerate() {
        for (x, col) in row.chars().enumerate() {
            if col == '#' {
                galaxies.push((x, y));
                row_empty[y] = false;
                col_empty[x] = false;
            }
        }
    }
    let mut total_distance = 0;
    for (i, galaxy) in galaxies.iter().enumerate() {
        let (x, y) = galaxy;
        for other_galaxy in galaxies.iter().skip(i) {
            let (x2, y2) = other_galaxy;
            let distance = x2.abs_diff(*x) + y2.abs_diff(*y);
            let col_range = *x.min(x2)..*x.max(x2);
            let row_range = *y.min(y2)..*y.max(y2);
            let expanded_rows = row_empty[row_range].iter().filter(|&&x| x).count();
            let expanded_cols = col_empty[col_range].iter().filter(|&&x| x).count();
            total_distance += distance + (expanded_rows + expanded_cols) * expansion_factor;
        }
    }
    total_distance
}



fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    println!("Part One: {}", expanded_universe_total_distance(&lines, 1));
    println!("Part Two: {}", expanded_universe_total_distance(&lines, 999999));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &[&str; 10] = &[
            "...#......",
            ".......#..",
            "#.........",
            "..........",
            "......#...",
            ".#........",
            ".........#",
            "..........",
            ".......#..",
            "#...#.....",
        ];

    #[test]
    fn test_expanded_universe() {
        assert_eq!(expanded_universe_total_distance(INPUT,1), 374);
    }

    #[test]
    fn test_more_expanded_universe() {
        assert_eq!(expanded_universe_total_distance(INPUT, 9), 1030);
    }

    #[test]
    fn test_even_more_expanded_universe() {
        assert_eq!(expanded_universe_total_distance(INPUT, 99), 8410);
    }   
}
