use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
};

use grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_offset(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Span {
    len: u32,
    dir: Direction,
}

impl Span {
    fn get_options(self, min_span: u32) -> Vec<Span> {
        if self.len > 0 && self.len < min_span {
            return vec![Span {
                len: self.len + 1,
                dir: self.dir,
            }];
        }
        match self.dir {
            Direction::Up => vec![
                Span {
                    len: self.len + 1,
                    dir: Direction::Up,
                },
                Span {
                    len: 1,
                    dir: Direction::Left,
                },
                Span {
                    len: 1,
                    dir: Direction::Right,
                },
            ],
            Direction::Down => vec![
                Span {
                    len: self.len + 1,
                    dir: Direction::Down,
                },
                Span {
                    len: 1,
                    dir: Direction::Left,
                },
                Span {
                    len: 1,
                    dir: Direction::Right,
                },
            ],
            Direction::Left => vec![
                Span {
                    len: self.len + 1,
                    dir: Direction::Left,
                },
                Span {
                    len: 1,
                    dir: Direction::Up,
                },
                Span {
                    len: 1,
                    dir: Direction::Down,
                },
            ],
            Direction::Right => vec![
                Span {
                    len: self.len + 1,
                    dir: Direction::Right,
                },
                Span {
                    len: 1,
                    dir: Direction::Up,
                },
                Span {
                    len: 1,
                    dir: Direction::Down,
                },
            ],
        }
    }

    fn up_to_spans(self) -> Vec<Span> {
        (1..=self.len)
            .map(|len| Span { dir: self.dir, len })
            .collect()
    }
}

fn parse_grid(input: &str) -> Grid<u32> {
    let lines = input.lines().collect::<Vec<_>>();
    Grid::from_vec(
        lines
            .iter()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
            .collect::<Vec<_>>(),
        lines[0].len(),
    )
}

fn min_cost(grid: &Grid<u32>, min_span: u32, max_span: u32) -> u32 {
    // (loss, x, y)
    let mut priority_queue = BinaryHeap::from(vec![(
        Reverse(0),
        0_i32,
        0_i32,
        Span {
            len: 0,
            dir: Direction::Right,
        },
    )]);
    let mut current_bests: HashMap<(i32, i32, Span), u32> = HashMap::new();
    let mut total_bests: HashMap<(i32, i32), u32> = HashMap::new();
    let mut visited: HashSet<(i32, i32, Span)> = HashSet::new();

    'outer: while let Some((Reverse(loss), x, y, span)) = priority_queue.pop() {
        if visited.contains(&(x, y, span)) {
            continue;
        }
        if let Some(best) = current_bests.get(&(x, y, span)) {
            if loss >= *best {
                continue;
            }
        }
        current_bests.insert((x, y, span), loss);
        visited.insert((x, y, span));
        if span.len >= min_span {
            if let Some(best) = total_bests.get_mut(&(x, y)) {
                if loss < *best {
                    *best = loss;
                }
            } else {
                total_bests.insert((x, y), loss);
            }
        }

        if vec![
            Span {
                dir: Direction::Right,
                len: max_span,
            },
            Span {
                dir: Direction::Down,
                len: max_span,
            },
        ]
        .iter()
        .flat_map(|s| s.up_to_spans())
        .filter(|s| s.len >= min_span)
        .all(|s| visited.contains(&(grid.cols() as i32 - 1, grid.rows() as i32 - 1, s)))
        {
            break 'outer;
        }

        for option in span.get_options(min_span) {
            let (x_offset, y_offset) = option.dir.get_offset();
            let (new_x, new_y) = (x + x_offset, y + y_offset);
            if new_x < 0
                || new_y < 0
                || new_x >= grid.cols() as i32
                || new_y >= grid.rows() as i32
                || option.len > max_span
            {
                continue;
            }
            if !visited.contains(&(new_x, new_y, option)) {
                priority_queue.push((
                    Reverse(loss + grid[(new_y as usize, new_x as usize)]),
                    new_x,
                    new_y,
                    option,
                ));
            }
        }
    }

    *total_bests
        .get(&(grid.cols() as i32 - 1, grid.rows() as i32 - 1))
        .unwrap()
}

fn part_one(input: &str) -> u32 {
    let grid = parse_grid(input);

    min_cost(&grid, 0, 3)
}

fn part_two(input: &str) -> u32 {
    let grid = parse_grid(input);

    min_cost(&grid, 4, 10)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 102);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 94);
    }

    #[test]
    fn test_part_two_unfortunate() {
        let input = r#"111111111111
999999999991
999999999991
999999999991
999999999991"#;
        assert_eq!(part_two(input), 71);
    }
}
