fn get_min_hold_time(time: i64, target_distance: i64) -> i64 {
    let time_f = time as f64;
    let target_distance_f = target_distance as f64;
    f64::ceil((time_f - f64::sqrt(time_f.powf(2.0) - 4.0 * target_distance_f)) / 2.0) as i64
}
fn get_max_hold_time(time: i64, target_distance: i64) -> i64 {
    let time_f = time as f64;
    let target_distance_f = target_distance as f64;
    f64::floor((time_f + f64::sqrt(time_f.powf(2.0) - 4.0 * target_distance_f)) / 2.0) as i64
}

#[derive(Debug)]
struct Race {
    time: i64,
    target_distance: i64,
}
impl Race {
    fn parse_all(times_line: &str, distances_line: &str) -> Vec<Race> {
        let mut times = times_line
            .chars().skip(5).skip_while(|c| c.is_whitespace()).collect::<String>()
            .split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let mut target_distances = distances_line
            .chars().skip(9).skip_while(|c| c.is_whitespace()).collect::<String>()
            .split_whitespace().map(|x| x.parse::<i64>().unwrap() + 1).collect::<Vec<i64>>();

        if times.len() != target_distances.len() {
            panic!("Times and target distances must be the same length");
        }

        let mut races: Vec<Race> = Vec::new();
        for i in 0..times.len() {
            races.push(Race {
                time: times[i],
                target_distance: target_distances[i],
            });
        }
        races
    }

    fn parse_single(times_line: &str, distances_line: &str) -> Race {
        let mut time = times_line
            .chars().skip(5).collect::<String>()
            .replace(" ", "").parse::<i64>().unwrap();
        let mut target_distance = distances_line
            .chars().skip(9).collect::<String>()
            .replace(" ", "").parse::<i64>().unwrap();

        Race {
            time,
            target_distance,
        }
    }
}

fn main() {
    let is_part_one = common::is_part_one();
    let input_file_path = common::get_input_file_path();
    let lines = common::read_file_line_by_line(&input_file_path);
    if lines.len() != 2 {
        panic!("Input file must have two lines exactly");
    }

    if is_part_one {
        let races = Race::parse_all(&lines[0], &lines[1]);
        let mut i = 1;
        let mut result = 1;
        for race in races {
            let min_hold_time = get_min_hold_time(race.time, race.target_distance);
            let max_hold_time = get_max_hold_time(race.time, race.target_distance);
            let options = max_hold_time - min_hold_time + 1;
            println!("For race {} you have to hold for {}-{}ms ({} options)",
                     i, min_hold_time, max_hold_time, options
            );
            result *= options;
            i += 1;
        }
        println!("Result: {}", result);
    } else {
        let race = Race::parse_single(&lines[0], &lines[1]);
        let min_hold_time = get_min_hold_time(race.time, race.target_distance);
        let max_hold_time = get_max_hold_time(race.time, race.target_distance);
        let options = max_hold_time - min_hold_time + 1;
        println!("To win the race you have to hold for {}-{}ms ({} options)",
                 min_hold_time, max_hold_time, options
        );
        println!("Result: {}", options);
    }
}
