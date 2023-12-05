#[derive(Debug)]
struct CubeSet {
    red: u32,
    blue: u32,
    green: u32,
}

impl CubeSet {
    fn parse(line: &str) -> CubeSet {
        let mut red: u32 = 0;
        let mut blue: u32 = 0;
        let mut green: u32 = 0;
        let parts: Vec<&str> = line.split(", ").collect();

        for part in parts {
            let part_parts: Vec<&str> = part.split(" ").collect();
            let value = part_parts[0].parse().unwrap();
            let color = part_parts[1];
            match color {
                "red" => red = value,
                "blue" => blue = value,
                "green" => green = value,
                _ => panic!("Unknown color: {}", color),
            }
        }

        CubeSet { red, blue, green }
    }

    fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

#[derive(Debug)]
struct Game {
    game: u32,
    cube_sets: Vec<CubeSet>,
}

impl Game {
    fn parse(line: &str) -> Game {
        // split line on ": " to get the game and cube sets
        let parts: Vec<&str> = line.split(": ").collect();
        // trim "Game " from beginning of part[0] to get game number
        let game: u32 = parts[0][5..].parse().unwrap();
        // split part[1] on "; " to get the cube sets
        let cube_sets: Vec<&str> = parts[1].split("; ").collect();

        Game {
            game,
            cube_sets: cube_sets.iter().map(|x| CubeSet::parse(x)).collect()
        }
    }
    fn possible(&self, all_cubes: CubeSet) -> bool {
        self.cube_sets.iter().all(|x| {
            x.red <= all_cubes.red &&
            x.blue <= all_cubes.blue &&
            x.green <= all_cubes.green
        })
    }
    fn minimum_possible_set(&self) -> CubeSet {
        let mut red: u32 = 0;
        let mut blue: u32 = 0;
        let mut green: u32 = 0;
        for cube_set in &self.cube_sets {
            if cube_set.red > red {
                red = cube_set.red;
            }
            if cube_set.blue > blue {
                blue = cube_set.blue;
            }
            if cube_set.green > green {
                green = cube_set.green;
            }
        }
        CubeSet { red, blue, green }
    }
}

fn main() {
    let is_part_one = common::is_part_one();
    let input_file_path = common::get_input_file_path();
    let lines: Vec<String> = common::read_file_line_by_line(&input_file_path);
    let games = lines.iter().map(|l| Game::parse(l));

    if is_part_one {
        let mut possible_games_sum: u32 = 0;
        for game in games {
            if game.possible(CubeSet { red: 12, blue: 14, green: 13 }) {
                possible_games_sum += game.game;
            }
        }
        println!("Sum of possible games: {}", possible_games_sum);
    } else {
        let mut game_minimum_set_power_sum: u32 = 0;
        for game in games {
            let minimum_set = game.minimum_possible_set();
            game_minimum_set_power_sum += minimum_set.power();
        }
        println!("Sum of games minimum set powers: {}", game_minimum_set_power_sum);
    }
}
