use num::BigInt;

#[derive(Debug, PartialEq)]
struct Galaxy {
    x: u64,
    y: u64,
}

#[derive(Debug)]
struct Image {
    galaxies: Vec<Galaxy>,
    empty_columns: Vec<u64>,
    empty_rows: Vec<u64>,
}
impl Image {
    fn parse(lines: &Vec<String>) -> Image {
        let mut galaxies = Vec::new();
        let mut empty_columns = Vec::new();
        let mut empty_rows = Vec::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        galaxies.push(Galaxy { x: x as u64, y: y as u64 });
                    }
                    '.' => {
                        // noop
                    }
                    _ => panic!("Unexpected character: {}", c),
                }
            }
        }

        let width = galaxies.iter().map(|g| g.x).max().unwrap_or(0) + 1;
        let height = galaxies.iter().map(|g| g.y).max().unwrap_or(0) + 1;

        for x in 0..width {
            if !galaxies.iter().any(|g| g.x == x) {
                empty_columns.push(x);
            }
        }
        for y in 0..height {
            if !galaxies.iter().any(|g| g.y == y) {
                empty_rows.push(y);
            }
        }

        Image {
            galaxies,
            empty_columns,
            empty_rows,
        }
    }
}

fn main() {
    let is_part_one = common::is_part_one();
    let input_file_path = common::get_input_file_path();
    let lines = &common::read_file_line_by_line(&input_file_path);

    let expansion_rate = if is_part_one { 2 } else { 1_000_000 };

    let image = Image::parse(lines);

    let mut distances_sum = 0;
    let mut distance_to_expand = 0;
    for index_a in 0..image.galaxies.len() - 1 {
        for index_b in index_a + 1..image.galaxies.len() {
            let galaxy_a = &image.galaxies[index_a];
            let galaxy_b = &image.galaxies[index_b];

            let min_x = galaxy_a.x.min(galaxy_b.x);
            let max_x = galaxy_a.x.max(galaxy_b.x);
            let min_y = galaxy_a.y.min(galaxy_b.y);
            let max_y = galaxy_a.y.max(galaxy_b.y);
            let distance = (max_x - min_x) + (max_y - min_y);

            let empty_columns_between = image.empty_columns.iter().filter(|x| **x > min_x && **x < max_x).count() as u64;
            let empty_rows_between = image.empty_rows.iter().filter(|y| **y > min_y && **y < max_y).count() as u64;

            distances_sum += distance;
            distance_to_expand += empty_columns_between + empty_rows_between;
        }
    }

    let expanded_distance = BigInt::from(distances_sum) +
        BigInt::from(distance_to_expand) * BigInt::from(expansion_rate - 1);

    println!("Result: {}", expanded_distance)
}
