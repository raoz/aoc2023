use std::{str::FromStr, fs};


#[derive(Debug, PartialEq)]
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

    fn loop_length(&self) -> usize {
        let mut length = 0;
        let mut pos = self.starting_point;
        let mut came_from = (0, 0);
        while pos != self.starting_point || length == 0 {
            let (x, y) = pos;
            let pipe = self.get(x, y).unwrap();
            for (delta_x, delta_y) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
                if (delta_x, delta_y) == came_from {
                    continue;
                }
                if pipe.connects_to(delta_x, delta_y) {
                    pos = (x + delta_x, y + delta_y);
                    came_from = (-delta_x, -delta_y);
                    length += 1;
                    break;
                }
            }            
        }
        length
    }
}

fn part_one(input: &str) -> usize {
    let mut grid = Grid::from_str(input).unwrap();

    grid.replace_starting_point();
    grid.loop_length() / 2
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part one: {}", part_one(&input));
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
}