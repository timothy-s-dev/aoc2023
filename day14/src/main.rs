use std::fmt::{Debug, Formatter, Result};
use std::ops::Mul;

const OPEN_CHAR: char = '.';
const CUBE_CHAR: char = '#';
const ROUND_CHAR: char = 'O';

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Open,
    Cube,
    Round,
    Outside,
}

#[derive(Clone, Copy, Debug)]
struct Offset {
    x: i32,
    y: i32,
}

impl Mul<i32> for Offset {
    type Output = Self;
    fn mul(self, other: i32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn get_offset(&self) -> Offset {
        match self {
            Direction::Up => Offset { x: 0, y: -1 },
            Direction::Down => Offset { x: 0, y: 1 },
            Direction::Left => Offset { x: -1, y: 0 },
            Direction::Right => Offset { x: 1, y: 0 },
        }
    }
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
        let index = self.get_tile_index(x, y);
        if index >= self.tiles.len() {
            return Tile::Outside;
        }
        self.tiles[index]
    }

    fn get_tile_index(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    fn tilt(&self, direction: Direction) -> Self {
        let mut new_tiles = self.tiles.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get_tile(x, y) == Tile::Round {
                    let tile_index = self.get_tile_index(x, y);
                    new_tiles[tile_index] = Tile::Open;
                }
            }
        }
        for y in 0..self.height {
            for x in 0..self.width {
                let tile_index = self.get_tile_index(x, y);
                let tile = self.get_tile(x, y);
                if tile != Tile::Round {
                    continue;
                }

                let destination = self.find_roll_destination(x, y, direction);
                let destination_index = self.get_tile_index(destination.0, destination.1);

                // println!("{} {} -> {} {}", x, y, destination.0, destination.1);

                new_tiles[destination_index] = Tile::Round;
            }
        }
        Self {
            tiles: new_tiles,
            width: self.width,
            height: self.height,
        }
    }

    fn find_roll_destination(&self, x: i32, y: i32, direction: Direction) -> (i32, i32) {
        let offset = direction.get_offset();
        let mut x = x + offset.x;
        let mut y = y + offset.y;
        let mut round_count = 0;
        loop {
            let tile = self.get_tile(x, y);
            if tile == Tile::Round {
                round_count += 1;
            }
            if tile == Tile::Cube  || tile == Tile::Outside {
                x -= offset.x;
                y -= offset.y;
                break;
            }
            x += offset.x;
            y += offset.y;
        }
        let final_offset = offset * round_count;
        (x - final_offset.x, y - final_offset.y)
    }

    fn calculate_load(&self, x: i32, y: i32, direction: Direction) -> i32 {
        let mut load = 0;
        let mut x = x;
        let mut y = y;
        loop {
            let tile = self.get_tile(x, y);
            if tile != Tile::Outside {
                load += 1;
                x -= direction.get_offset().x;
                y -= direction.get_offset().y;
            } else {
                break;
            }
        }
        load
    }

    fn calculate_all_load(&self, direction: Direction) -> i32 {
        let mut load = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = self.get_tile(x, y);
                if tile != Tile::Round {
                    continue;
                }
                load += self.calculate_load(x, y, direction);
            }
        }
        load
    }

    fn spin(&self) -> Self {
        let mut result = self.tilt(Direction::Up);
        // println!("{:?}", result);
        result = result.tilt(Direction::Left);
        // println!("{:?}", result);
        result = result.tilt(Direction::Down);
        // println!("{:?}", result);
        result.tilt(Direction::Right)
    }

    fn get_hash(&self) -> u64 {
        let mut hash: u64 = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = self.get_tile(x, y);
                if tile == Tile::Round {
                    hash += (y * self.width + x) as u64;
                    hash *= 17;
                    hash %= 1000000007;
                }
            }
        }
        hash
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

    if is_part_one {
        let tilted = platform.tilt(Direction::Down);
        println!("{:?}", tilted);
        println!("Load: {}", tilted.calculate_all_load(Direction::Up));
    } else {
        let target_cycles = 1_000_000_000;
        let mut hashes: Vec<u64> = Vec::new();
        hashes.push(platform.get_hash());
        let mut cycles: u64 = 0;
        let mut spun = platform;
        let mut cycle_length = 0;
        loop {
            cycles += 1;
            spun = spun.spin();
            let hash = spun.get_hash();
            let found_cycle = hashes.iter().position(|&r| r == hash);
            hashes.push(hash);
            if let Some(cycle_start) = found_cycle {
                cycle_length = cycles - cycle_start as u64;
                println!("Loop found from {} - {}", cycle_start, cycles);
                break;
            }
        }

        let remaining_cycles = (target_cycles - cycles) % cycle_length;
        println!("Can skip {} cycles, {} remaining...", target_cycles - (remaining_cycles + cycles), remaining_cycles);
        for _ in 0..remaining_cycles {
            spun = spun.spin();
        }
        println!("{:?}", spun);

        println!("Load: {}", spun.calculate_all_load(Direction::Up));
    }
}
