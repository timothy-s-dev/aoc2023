use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction { North, East, South, West }

struct Distances {
    distances: Vec<Vec<Option<u32>>>,
}
impl Distances {
    fn get_distance(&self, x: usize, y: usize) -> Option<u32> { self.distances[y][x] }
}
impl fmt::Debug for Distances {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for row in self.distances.iter() {
            for distance in row.iter() {
                if distance.is_none() {
                    s.push_str(if f.alternate() { "     " } else { " " });
                } else {
                    if f.alternate() {
                        s.push_str(&format!(" {:4}", distance.unwrap()));
                    } else {
                        s.push_str("#");
                    }
                }
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

#[derive(Copy, Clone)]
struct Pipe {
    directions: [Direction; 2],
}
impl Pipe {
    fn parse(c: char) -> Pipe {
        match c {
            '|' => Pipe { directions: [Direction::North, Direction::South] },
            '-' => Pipe { directions: [Direction::East, Direction::West] },
            'L' => Pipe { directions: [Direction::North, Direction::East] },
            'J' => Pipe { directions: [Direction::North, Direction::West] },
            'F' => Pipe { directions: [Direction::East, Direction::South] },
            '7' => Pipe { directions: [Direction::South, Direction::West] },
            _ => panic!("Invalid pipe character: {}", c),
        }
    }

    fn has_north(&self) -> bool { self.directions.iter().any(|d| *d == Direction::North) }
    fn has_east(&self) -> bool { self.directions.iter().any(|d| *d == Direction::East) }
    fn has_south(&self) -> bool { self.directions.iter().any(|d| *d == Direction::South) }
    fn has_west(&self) -> bool { self.directions.iter().any(|d| *d == Direction::West) }
}
impl fmt::Debug for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.has_north() && self.has_south() {
            write!(f, "║")
        } else if self.has_east() && self.has_west() {
            write!(f, "═")
        } else if self.has_north() && self.has_east() {
            write!(f, "╚")
        } else if self.has_north() && self.has_west() {
            write!(f, "╝")
        } else if self.has_east() && self.has_south() {
            write!(f, "╔")
        } else if self.has_south() && self.has_west() {
            write!(f, "╗")
        } else {
            panic!("Invalid pipe: {:?}", self.directions)
        }
    }
}

struct Grid {
    start_x: usize,
    start_y: usize,
    pipes: Vec<Vec<Option<Pipe>>>,
}
impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for (y, row) in self.pipes.iter().enumerate() {
            for (x, pipe) in row.iter().enumerate() {
                if pipe.is_none() {
                    s.push(' ');
                    continue;
                }
                if x == self.start_x && y == self.start_y {
                    s.push('S');
                    continue;
                }

                s.push(format!("{:?}", pipe.unwrap()).chars().next().unwrap());
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}
impl Grid {
    fn parse(lines: &Vec<String>) -> Grid {
        let mut pipes = Vec::new();
        let mut start_x: usize = 0;
        let mut start_y: usize = 0;
        for (y, line) in lines.iter().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    row.push(None);
                    start_x = x;
                    start_y = y;
                    continue;
                } else if c == '.' {
                    row.push(None);
                    continue;
                }
                row.push(Some(Pipe::parse(c)));
            }
            pipes.push(row);
        }
        let has_north = start_y > 0 && pipes[start_y - 1][start_x].is_some() &&
            pipes[start_y - 1][start_x].unwrap().has_south();
        let has_south = start_y < pipes.len() - 1 && pipes[start_y + 1][start_x].is_some() &&
            pipes[start_y + 1][start_x].unwrap().has_north();
        let has_west = start_x > 0 && pipes[start_y][start_x - 1].is_some() &&
            pipes[start_y][start_x - 1].unwrap().has_east();
        let has_east = start_x < pipes[start_y].len() - 1 && pipes[start_y][start_x + 1].is_some() &&
            pipes[start_y][start_x + 1].unwrap().has_west();
        if has_north && has_east {
            pipes[start_y][start_x] = Some(Pipe{ directions: [Direction::North, Direction::East] });
        } else if has_north && has_south {
            pipes[start_y][start_x] = Some(Pipe{ directions: [Direction::North, Direction::South] });
        } else if has_north && has_west {
            pipes[start_y][start_x] = Some(Pipe{ directions: [Direction::North, Direction::West] });
        } else if has_east && has_south {
            pipes[start_y][start_x] = Some(Pipe{ directions: [Direction::East, Direction::South] });
        } else if has_east && has_west {
            pipes[start_y][start_x] = Some(Pipe{ directions: [Direction::East, Direction::West] });
        } else if has_south && has_west {
            pipes[start_y][start_x] = Some(Pipe{ directions: [Direction::South, Direction::West] });
        } else {
            panic!("Invalid starting position");
        }
        Grid { start_x, start_y, pipes }
    }

    fn get_pipe(&self, x: usize, y: usize) -> Option<Pipe> { self.pipes[y][x] }

    fn calculate_distances(&self) -> Distances {
        // use breadth first starting from start_x, start_y to calculate distance to all other pipes
        let mut distances = vec![vec![None; self.pipes[0].len()]; self.pipes.len()];
        let mut queue = Vec::new();
        queue.push((self.start_x, self.start_y, 0));
        while !queue.is_empty() {
            let (x, y, distance) = queue.remove(0);
            if distances[y][x].is_some() {
                continue;
            }
            distances[y][x] = Some(distance);
            if self.pipes[y][x].unwrap().has_west() {
                queue.push((x - 1, y, distance + 1));
            }
            if self.pipes[y][x].unwrap().has_east() {
                queue.push((x + 1, y, distance + 1));
            }
            if self.pipes[y][x].unwrap().has_north() {
                queue.push((x, y - 1, distance + 1));
            }
            if self.pipes[y][x].unwrap().has_south() {
                queue.push((x, y + 1, distance + 1));
            }
        }
        Distances { distances }
    }

    fn simplify(&self, distances: &Distances) -> Grid {
        let mut pipes = Vec::new();
        for (y, row) in self.pipes.iter().enumerate() {
            let mut new_row = Vec::new();
            for (x, pipe) in row.iter().enumerate() {
                if distances.get_distance(x, y).is_some() {
                    new_row.push(*pipe);
                    continue;
                }
                new_row.push(None);
            }
            pipes.push(new_row);
        }
        Grid { start_x: self.start_x, start_y: self.start_y, pipes }
    }


    fn is_inside(&self, x: usize, y: usize) -> bool {
        if x == 0 || y == 0 || x == self.pipes[0].len() - 1 || y == self.pipes.len() - 1 {
            return false;
        }
        if self.pipes[y][x].is_some() {
            return false;
        }

        let mut pipes_str = self.pipes[y][0..x].iter()
            .map(|p| {
                match p {
                    None => ' ',
                    Some(p) => format!("{:?}", p).chars().next().unwrap(),
                }
            }).collect::<String>();

        pipes_str = pipes_str.replace(" ", "");
        pipes_str = pipes_str.replace("═", "");
        pipes_str = pipes_str.replace("╚╝", "");
        pipes_str = pipes_str.replace("╔╗", "");
        pipes_str = pipes_str.replace("╔╝", "║");
        pipes_str = pipes_str.replace("╚╗", "║");

        let pipes_west = pipes_str.len();

        pipes_west % 2 == 1
    }
}



fn main() {
    let is_part_one = common::is_part_one();
    let input_file_path = common::get_input_file_path();
    let mut grid = Grid::parse(&common::read_file_line_by_line(&input_file_path));
    let distances = grid.calculate_distances();

    println!("grid:\n{:#?}", grid);
    println!("distances:\n{:?}", distances);
    println!("Max Distance: {:?}", distances.distances.iter().flatten().max().unwrap());

    grid = grid.simplify(&distances);

    let mut tiles_inside: u32 = 0;
    for (y, row) in grid.pipes.iter().enumerate() {
        for (x, pipe) in row.iter().enumerate() {
            if grid.is_inside(x, y) {
                tiles_inside += 1;
                print!("!");
            } else if pipe.is_some() {
                print!("{:?}", pipe.unwrap());
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!("Tiles inside: {}", tiles_inside);
}
