use std::{str::FromStr, fs, fmt::Display};
use colored::Colorize;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Pipe {
    Ground,
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    StartingPoint,
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Pipe::Ground => '.',
            Pipe::NorthSouth => '|',
            Pipe::EastWest => '-',
            Pipe::NorthEast => 'L',
            Pipe::NorthWest => 'J',
            Pipe::SouthEast => 'F',
            Pipe::SouthWest => '7',
            Pipe::StartingPoint => 'S',
        };
        write!(f, "{}", c)
    }
}

impl Pipe {
    fn connects_to(&self, delta_x: i32, delta_y: i32) -> bool {
        if delta_x.abs() + delta_y.abs() != 1 {
            return false;
        }
        match self {
            Pipe::Ground => false,
            Pipe::NorthSouth => delta_x == 0,
            Pipe::EastWest => delta_y == 0,
            Pipe::NorthEast => delta_x == 1 || delta_y == -1,
            Pipe::NorthWest => delta_x == -1 || delta_y == -1,
            Pipe::SouthEast => delta_x == 1 || delta_y == 1,
            Pipe::SouthWest => delta_x == -1 || delta_y == 1,
            Pipe::StartingPoint => false,
        }

    }

    fn from_connections(connections: &[(i32, i32)]) -> Pipe {
        if connections.len() != 2 {
            panic!("Invalid connections")
        }
        for pipe in [Pipe::NorthSouth, Pipe::EastWest, Pipe::NorthEast, Pipe::NorthWest, Pipe::SouthEast, Pipe::SouthWest] {
            if connections.iter().all(|(x, y)| pipe.connects_to(*x, *y)) {
                return pipe;
            }
        }
        panic!("Invalid connections")
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Pipe>>,
    starting_point: (i32, i32),
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::new();
        let mut starting_point = (0, 0);
        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let pipe = match c {
                    '.' => Pipe::Ground,
                    '-' => Pipe::EastWest,
                    '|' => Pipe::NorthSouth,
                    'S' => Pipe::StartingPoint,
                    'L' => Pipe::NorthEast,
                    'J' => Pipe::NorthWest,
                    'F' => Pipe::SouthEast,
                    '7' => Pipe::SouthWest,
                    _ => panic!("Invalid character"),
                };
                if pipe == Pipe::StartingPoint {
                    starting_point = (x as i32, y as i32);
                }
                row.push(pipe);
            }
            grid.push(row);
        }
        Ok(Grid { grid, starting_point })
    }

}

impl Grid {
    fn replace_starting_point(&mut self) {
        let (start_x, start_y) = self.starting_point;

        let mut connections = vec![];

        for (x, y) in [(start_x - 1, start_y), (start_x  + 1, start_y), (start_x, start_y - 1), (start_x, start_y + 1)] {
            if let Some(pipe) = self.get(x, y) {
                if pipe.connects_to(start_x - x, start_y - y) {
                    connections.push((x - start_x, y - start_y));
                }
            }
        }

        self.grid[start_y as usize][start_x as usize] = Pipe::from_connections(&connections[..]);
    }

    fn get(&self, x: i32, y: i32) -> Option<&Pipe> {
        if x < 0 || y < 0 {
            return None;
        }
        self.grid.get(y as usize).and_then(|row| row.get(x as usize))
    }

    fn get_loop(&self) -> Vec<(i32, i32)> {
        let mut result = vec![];
        let mut pos = self.starting_point;
        let mut came_from = (0, 0);
        while pos != self.starting_point || result.is_empty() {
            let (x, y) = pos;
            let pipe = self.get(x, y).unwrap();
            for (delta_x, delta_y) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
                if (delta_x, delta_y) == came_from {
                    continue;
                }
                if pipe.connects_to(delta_x, delta_y) {
                    result.push(pos);
                    pos = (x + delta_x, y + delta_y);
                    came_from = (-delta_x, -delta_y);
                    break;
                }
            }            
        }
        result
    }

    fn count_loop_hits(&self, pos: (i32, i32), step_x: i32, step_y: i32, loop_tiles: &[(i32, i32)]) -> usize {
        let mut result = 0;

        let (mut x, mut y) = pos;
        let mut open_wall = None;

        while let Some(pipe) = self.get(x, y) {
            x += step_x;
            y += step_y;
            if loop_tiles.contains(&(x - step_x, y - step_y)) {
                match (pipe, step_x.abs(), step_y.abs()) {
                    (Pipe::NorthSouth, 1, _) => result += 1,
                    (Pipe::EastWest, _, 1) => result += 1,
                    (Pipe::NorthSouth, _, 1) => continue,
                    (Pipe::EastWest, 1, _) => continue,
                    (pipe, _, _) => {
                        if let Some(start_wall) = open_wall {
                            match (start_wall, pipe, step_x.abs(), step_y.abs()) {
                                (Pipe::NorthEast, Pipe::NorthWest, _, 1) => result += 1,
                                (Pipe::NorthWest, Pipe::NorthEast, _, 1) => result += 1,
                                (Pipe::SouthWest, Pipe::SouthEast, _, 1) => result += 1,
                                (Pipe::SouthEast, Pipe::SouthWest, _, 1) => result += 1,
                                (Pipe::NorthEast, Pipe::SouthWest, _, 1) => result += 1,
                                (Pipe::SouthWest, Pipe::NorthEast, _, 1) => result += 1,
                                (Pipe::NorthWest, Pipe::SouthEast, _, 1) => result += 1,
                                (Pipe::SouthEast, Pipe::NorthWest, _, 1) => result += 1,

                                (Pipe::NorthEast, Pipe::SouthEast, 1, _) => result += 1,
                                (Pipe::SouthEast, Pipe::NorthEast, 1, _) => result += 1,
                                (Pipe::NorthWest, Pipe::SouthWest, 1, _) => result += 1,
                                (Pipe::SouthWest, Pipe::NorthWest, 1, _) => result += 1,
                                (Pipe::NorthEast, Pipe::SouthWest, 1, _) => result += 1,
                                (Pipe::SouthWest, Pipe::NorthEast, 1, _) => result += 1,
                                (Pipe::NorthWest, Pipe::SouthEast, 1, _) => result += 1,
                                (Pipe::SouthEast, Pipe::NorthWest, 1, _) => result += 1,

                                (_, Pipe::EastWest, _, _) => continue,
                                (_, Pipe::NorthSouth, _, _) => continue,
                                _ => {},
                            }
                        } else {
                            open_wall = Some(*pipe);
                            continue;
                        }
                    }
                }
                open_wall = None;
            }
        }

        result
    }

    fn is_contained_by_loop(&self, x: i32, y: i32, loop_tiles: &[(i32, i32)]) -> bool {
        if loop_tiles.contains(&(x, y)) {
            return false;
        }
        self.count_loop_hits((x, y), 1, 0, loop_tiles) % 2 == 1 &&
        self.count_loop_hits((x, y), -1, 0, loop_tiles) % 2 == 1 &&
        self.count_loop_hits((x, y), 0, 1, loop_tiles) % 2 == 1 &&
        self.count_loop_hits((x, y), 0, -1, loop_tiles) % 2 == 1
    }
}

fn part_one(input: &str) -> usize {
    let mut grid = Grid::from_str(input).unwrap();

    grid.replace_starting_point();
    grid.get_loop().len() / 2
}

fn part_two(input: &str) -> usize {
    let mut grid = Grid::from_str(input).unwrap();
    grid.replace_starting_point();

    let grid_loop = grid.get_loop();

    let mut result = 0;

    for y in 0..grid.grid.len() {
        for x in 0..grid.grid[0].len() {
            if grid.is_contained_by_loop(x as i32, y as i32, &grid_loop) {
                result += 1;
                print!("{}", "I".red());
            } else {
                let mut tile_str = grid.get(x as i32, y as i32).unwrap().to_string();
                if grid_loop.contains(&(x as i32, y as i32)) {
                    tile_str = tile_str.blue().to_string();
                }
                print!("{}", tile_str);
            }
        }
        println!();
    }

    result
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
    fn test_part_one_simple() {
        assert_eq!(part_one(r#".....
.S-7.
.|.|.
.L-J.
....."#), 4);
    }
    #[test]
    fn test_part_one_simple_debris() {
        assert_eq!(part_one(r#"-L|F7
7S-7|
L|7||
-L-J|
L|-JF"#), 4);
    }

    #[test]
    fn test_part_one_complex() {
        assert_eq!(part_one(r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#), 8);
    }

    #[test]
    fn test_part_one_complex_debris() {
        assert_eq!(part_one(r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#), 8);
    }

    #[test]
    fn test_part_two_simple() {
        assert_eq!(part_two(r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#), 4);
    }

    #[test]
    fn test_part_two_larger() {
        assert_eq!(part_two(r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#), 8);
    }

    #[test]
    fn test_part_two_debris() {
        assert_eq!(part_two(r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#), 10);
    }   
}