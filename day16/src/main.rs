#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty, // .
    VerticalSplitter, // |
    HorizontalSplitter, // -
    RightMirror, // /
    LeftMirror, // \
}
impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '.' => Tile::Empty,
            '|' => Tile::VerticalSplitter,
            '-' => Tile::HorizontalSplitter,
            '/' => Tile::RightMirror,
            '\\' => Tile::LeftMirror,
            _ => panic!("Unknown tile: {}", c),
        }
    }

    fn get_new_directions(&self, direction: Direction) -> Vec<Direction> {
        let mut new_directions: Vec<Direction> = Vec::new();

        match *self {
            Tile::Empty => new_directions.push(direction),
            _ => todo!(),
        };

        new_directions
    }
}

fn main() {
    let is_part_one = common::is_part_one();
    let input_file_path = common::get_input_file_path();
    let lines = &common::read_file_line_by_line(&input_file_path);

    println!("TODO: Complete puzzle (part {}) using file: {}",
             if is_part_one { 1 } else { 2 },
             input_file_path
    );
}
