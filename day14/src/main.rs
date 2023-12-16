use std::fmt::{Debug, Formatter, Result};

const OPEN_CHAR: char = '.';
const CUBE_CHAR: char = '#';
const ROUND_CHAR: char = 'O';

#[derive(Clone, Copy, Debug)]
enum Tile {
    Open,
    Cube,
    Round,
    Outside,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Platform {
    tiles: Vec<Tile>,
    width: i32,
    height: i32,
}

impl Platform {
    fn parse(lines: &Vec<String>) -> Platform {
        let mut tiles = Vec::new();

        for line in lines {
            for c in line.chars() {
                tiles.push(match c {
                    OPEN_CHAR => Tile::Open,
                    CUBE_CHAR => Tile::Cube,
                    ROUND_CHAR => Tile::Round,
                    _ => panic!("Invalid tile char: {}", c),
                });
            }
        }

        if tiles.len() % lines.len() != 0 {
            panic!("Invalid platform, must be rectangular");
        }

        let width = tiles.len() as i32 / lines.len() as i32;
        let height = lines.len() as i32;

        Platform {
            tiles,
            width,
            height,
        }
    }

    fn get_tile(&self, x: i32, y: i32) -> Tile {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return Tile::Outside;
        }
        let index = y * self.width + x;
        if index >= self.tiles.len() as i32 {
            return Tile::Outside;
        }
        self.tiles[index as usize]
    }

    fn get_offset(&self, direction: Direction) -> i32 {
        match direction {
            Direction::Up => -self.width,
            Direction::Down => self.width,
            Direction::Left => -1,
            Direction::Right => 1,
        }
    }

    fn tilt(&self, direction: Direction) -> Self {

    }
}

impl Debug for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get_tile(x, y) {
                    Tile::Open => write!(f, "{}", OPEN_CHAR),
                    Tile::Cube => write!(f, "{}", CUBE_CHAR),
                    Tile::Round => write!(f, "{}", ROUND_CHAR),
                    Tile::Outside => write!(f, " "),
                }?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let is_part_one = common::is_part_one();
    let input_file_path = common::get_input_file_path();
    let lines = &common::read_file_line_by_line(&input_file_path);

    let platform = Platform::parse(lines);
    println!("{:?}", platform);
}
