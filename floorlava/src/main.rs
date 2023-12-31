use std::fs;

use grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }    
}

#[derive(Debug)]
struct Beam {
    direction: Direction,
    position: (isize, isize),
}

impl Beam {
    fn new(direction: Direction, position: (isize, isize)) -> Self {
        Beam {
            direction,
            position,
        }
    }

    fn go(&self, direction: Direction) -> Beam {
        match direction {
            Direction::Up => Beam::new(Direction::Up, (self.position.0, self.position.1 - 1)),
            Direction::Down => Beam::new(Direction::Down, (self.position.0, self.position.1 + 1)),
            Direction::Left => Beam::new(Direction::Left, (self.position.0 - 1, self.position.1)),
            Direction::Right => Beam::new(Direction::Right, (self.position.0 + 1, self.position.1)),
        }
    }

    fn go_straight(&self) -> Beam {
        self.go(self.direction)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileKind {
    Open,
    ForwardMirror,
    BackwardMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

#[derive(Debug, Clone)]
struct Tile {
    kind: TileKind,
    entry_directions: Vec<Direction>,
}

impl Tile {
    fn new(kind: TileKind) -> Self {
        Tile {
            kind,
            entry_directions: Vec::new(),
        }
    }

    fn energized(&self) -> bool {
        !self.entry_directions.is_empty()
    }

    fn visited(&self, direction: Direction) -> bool {
        match self.kind {
            TileKind::Open => self.entry_directions.contains(&direction) || self.entry_directions.contains(&direction.opposite()),
            TileKind::HorizontalSplitter | TileKind::VerticalSplitter => self.energized(),
            _ => self.entry_directions.contains(&direction),
        }
    }

    fn visit(&mut self, beam: &Beam) -> Vec<Beam> {
        self.entry_directions.push(beam.direction);
        match self.kind {
            TileKind::Open => vec![beam.go_straight()],
            TileKind::ForwardMirror => match beam.direction {
                Direction::Up => vec![beam.go(Direction::Right)],
                Direction::Down => vec![beam.go(Direction::Left)],
                Direction::Left => vec![beam.go(Direction::Down)],
                Direction::Right => vec![beam.go(Direction::Up)],
            },
            TileKind::BackwardMirror => match beam.direction {
                Direction::Up => vec![beam.go(Direction::Left)],
                Direction::Down => vec![beam.go(Direction::Right)],
                Direction::Left => vec![beam.go(Direction::Up)],
                Direction::Right => vec![beam.go(Direction::Down)],
            },
            TileKind::VerticalSplitter => match beam.direction {
                Direction::Up | Direction::Down => vec![beam.go_straight()],
                Direction::Left | Direction::Right => {
                    vec![beam.go(Direction::Up), beam.go(Direction::Down)]
                }
            },
            TileKind::HorizontalSplitter => match beam.direction {
                Direction::Up | Direction::Down => {
                    vec![beam.go(Direction::Left), beam.go(Direction::Right)]
                }
                Direction::Left | Direction::Right => vec![beam.go_straight()],
            },
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::new(TileKind::Open),
            '/' => Tile::new(TileKind::ForwardMirror),
            '\\' => Tile::new(TileKind::BackwardMirror),
            '|' => Tile::new(TileKind::VerticalSplitter),
            '-' => Tile::new(TileKind::HorizontalSplitter),
            _ => panic!("Invalid tile {c}"),
        }
    }
}

fn get_energy_level_with_starting_beam(mut grid: Grid<Tile>, starting_beam: Beam) -> usize {
    let mut beams = vec![starting_beam];

    while let Some(beam) = beams.pop() {
        if beam.position.0 < 0 || beam.position.1 < 0 {
            continue;
        }
        if let Some(tile) = grid.get_mut(beam.position.1 as usize, beam.position.0 as usize) {
            if tile.visited(beam.direction) {
                continue;
            }
            beams.extend(tile.visit(&beam));
        }
    }

    grid.iter().filter(|t| t.energized()).count()
}

fn part_one(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let grid = Grid::from_vec(
        lines
            .iter()
            .flat_map(|l| l.chars().map(Tile::from))
            .collect::<Vec<_>>(),
        lines[0].len(),
    );

    get_energy_level_with_starting_beam(grid, Beam::new(Direction::Right, (0, 0)))
}

fn part_two(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let grid = Grid::from_vec(
        lines
            .iter()
            .flat_map(|l| l.chars().map(Tile::from))
            .collect::<Vec<_>>(),
        lines[0].len(),
    );

    let mut max_energy = 0;
    for i in 0..grid.rows() {
        max_energy = max_energy.max(get_energy_level_with_starting_beam(
            grid.clone(),
            Beam::new(Direction::Right, (0, i as isize)),
        ));
        max_energy = max_energy.max(get_energy_level_with_starting_beam(
            grid.clone(),
            Beam::new(Direction::Left, (grid.cols() as isize - 1, i as isize)),
        ));
    }
    for i in 0..grid.cols() {
        max_energy = max_energy.max(get_energy_level_with_starting_beam(
            grid.clone(),
            Beam::new(Direction::Down, (i as isize, 0)),
        ));
        max_energy = max_energy.max(get_energy_level_with_starting_beam(
            grid.clone(),
            Beam::new(Direction::Up, (i as isize, grid.rows() as isize - 1)),
        ));
    }
    max_energy
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 46);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 51);
    }
}
